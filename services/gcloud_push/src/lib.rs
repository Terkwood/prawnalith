//! # gcloud_push
//!
//! This is a service which pushes temperature and pH
//! data to google cloud pub sub.  The temp & pH
//! data is expected to reside in a Redis instance.
#![feature(custom_attribute)]
extern crate google_pubsub1 as pubsub;
extern crate hyper;
extern crate hyper_native_tls;
extern crate redis_context;
#[macro_use]
extern crate serde_derive;
extern crate yup_oauth2;

pub mod config;
mod model;

use base64;
use redis::Commands;
use redis_context::RedisContext;
use redis_delta::{RDelta, REvent, RField};

use self::model::PubSubContext;
use self::pubsub::PublishRequest;
use std::default::Default;
use std::time::SystemTime;

pub fn hello_world(_redis_ctx: &RedisContext, pubsub_ctx: &PubSubContext) {
    let message = google_pubsub1::PubsubMessage {
        // Base64 encoded!
        data: Some(base64::encode("HELLO ANYONE!".as_bytes())),
        ..Default::default()
    };

    let req = PublishRequest {
        messages: Some(vec![message]),
    };

    println!("Publishing to {}", &pubsub_ctx.fq_topic);
    pubsub_ctx
        .client
        .projects()
        .topics_publish(req, &pubsub_ctx.fq_topic)
        .doit()
        .unwrap();
}

/// send *all* relevant redis data upstream
/// to the cloud instance via pub sub
///
/// # How to do this
///
/// - Figure out how many tanks there are
/// - Query each tank individually
/// - Figure out what types of sensors there are
/// - For each type of sensor, query for all the sensor IDs
/// - Query each individual sensor of each type
///
/// Push as you satisfy each individual step.
pub fn clone_the_world(_redis_ctx: &RedisContext, _pubsub_ctx: &PubSubContext) {
    unimplemented!()
}

/// Publish a vec of redis changes (hash updates, string updates, etc)
/// to google pubsub system.
///
/// In order to get this done, we need to first retrieve a recent
/// copy of each piece of data referred to in the vec of events.
/// - For string or set updates, this simply means retrieving the
///   entire set, or the string itself.
/// - For hash field updates, we only retrieve the fields
///   which have been updated.
pub fn push_recent<E>(
    redis_ctx: &RedisContext,
    pubsub_ctx: &PubSubContext,
    redis_events: Vec<REvent>,
) {
    redis_events
        .iter()
        .for_each(|revent| match fetch(revent, redis_ctx) {
            Err(e) => eprintln!("Redis fetch error: {:?}", e),
            Ok(Some(found)) => {
                push(&found, pubsub_ctx).unwrap_or(eprintln!("Error pushing {:?}", &found))
            }
            _ => (),
        })
}

fn fetch<'a>(
    event: &'a REvent,
    ctx: &RedisContext,
) -> Result<Option<RDelta<'a>>, redis::RedisError> {
    match event {
        REvent::HashUpdated { key, fields } => {
            fetch_hash_delta(key, fields.to_vec(), ctx).map(|r| Some(r))
        }
        REvent::StringUpdated { key } => fetch_string_delta(key, ctx),
        REvent::SetUpdated { key } => fetch_set_delta(key, ctx),
    }
}

fn fetch_string_delta<'a>(
    key: &'a str,
    ctx: &RedisContext,
) -> Result<Option<RDelta<'a>>, redis::RedisError> {
    let found: Option<String> = ctx.conn.get(key)?;
    Ok(found.map(|f| RDelta::UpdateString {
        key,
        val: f,
        time: epoch_secs(),
    }))
}

fn fetch_set_delta<'a>(
    key: &'a str,
    ctx: &RedisContext,
) -> Result<Option<RDelta<'a>>, redis::RedisError> {
    let found: Option<Vec<String>> = ctx.conn.smembers(key)?;
    Ok(found.map(|f| RDelta::UpdateSet {
        key,
        vals: f,
        time: epoch_secs(),
    }))
}

fn fetch_hash_delta<'a>(
    key: &'a str,
    fields: Vec<String>,
    ctx: &RedisContext,
) -> Result<RDelta<'a>, redis::RedisError> {
    let fields_forever = fields.clone();
    let found: Vec<Option<String>> = ctx.conn.hget(key, fields)?;
    let zipped = fields_forever.iter().zip(found);
    let rfields: Vec<RField> = zipped
        .map(|(f, maybe_v)| {
            maybe_v.map(|v| RField {
                name: f.to_string(),
                val: v,
            })
        })
        .filter(|maybe| maybe.is_some())
        .map(|some| some.unwrap())
        .collect();

    Ok(RDelta::UpdateHash {
        key,
        fields: rfields,
        time: epoch_secs(),
    })
}

fn push(data: &RDelta, pubsub_ctx: &PubSubContext) -> Result<(), pubsub::Error> {
    let message = google_pubsub1::PubsubMessage {
        // This must be base64 encoded!
        data: Some(base64::encode(
            serde_json::to_string(data).unwrap().as_bytes(),
        )),
        ..Default::default()
    };

    let req = PublishRequest {
        messages: Some(vec![message]),
    };

    pubsub_ctx
        .client
        .projects()
        .topics_publish(req, &pubsub_ctx.fq_topic)
        .doit()
        .map(|_r| ())
}

fn epoch_secs() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
