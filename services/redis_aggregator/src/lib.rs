//! # Redis Aggregator
//!
//! This is a service which monitors changes in
//! prawn-related data stored in our Redis database.
//! It keeps track of changed keys, and periodically
//! queries Redis for the values related to those keys,
//! then pushing the values via Google pub sub.
#![feature(custom_attribute)]
extern crate google_pubsub1;
extern crate hyper;
extern crate hyper_native_tls;
extern crate redis_context;
#[macro_use]
extern crate serde_derive;
extern crate yup_oauth2;

pub mod config;
pub mod pubsub;

use base64;
use redis::Commands;
use redis_context::RedisContext;
use redis_delta::{RDelta, REvent, RField};

use self::pubsub::PubSubContext;
use std::default::Default;
use std::time::SystemTime;

/// Send *all* relevant redis data upstream
/// to the cloud instance via pub sub.
///
/// This is expected to be invoked on service startup.
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
pub fn clone_the_world(
    _redis_ctx: &RedisContext,
    _pubsub_ctx: &PubSubContext,
) -> Result<(), CloneErr> {
    let __all_ids: Vec<REvent> = instantiate_all_ids()?;

    unimplemented!()
}
pub enum CloneErr {
    Redis(redis::RedisError),
}
impl From<redis::RedisError> for CloneErr {
    fn from(error: redis::RedisError) -> Self {
        CloneErr::Redis(error)
    }
}

fn instantiate_all_ids() -> Result<Vec<REvent>, redis::RedisError> {
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
pub fn push_recent(
    redis_ctx: &RedisContext,
    pubsub_ctx: &PubSubContext,
    redis_events: Vec<REvent>,
) {
    redis_events
        .iter()
        .for_each(|revent| match fetch(revent, redis_ctx) {
            Err(e) => eprintln!("Redis fetch error: {:?}", e),
            Ok(Some(found)) => {
                if let Err(e) = push(&found, pubsub_ctx) {
                    eprintln!("Error pushing {:?}: {:?}", &found, e)
                }
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

fn push(data: &RDelta, pubsub_ctx: &PubSubContext) -> Result<(), google_pubsub1::Error> {
    let message = google_pubsub1::PubsubMessage {
        // This must be base64 encoded!
        data: Some(base64::encode(
            serde_json::to_string(data).unwrap().as_bytes(),
        )),
        ..Default::default()
    };

    let req = google_pubsub1::PublishRequest {
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
