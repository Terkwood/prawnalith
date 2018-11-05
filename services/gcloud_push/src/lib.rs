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

use redis_context::RedisContext;
use redis_delta::REvent;

use self::model::{PubSubClient, PubSubContext};
use self::pubsub::PublishRequest;
use std::default::Default;

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
/// copy of each piece of data referred to in the RDelta
pub fn push_recent<E>(
    _redis_context: &RedisContext,
    _pubsub: &PubSubClient,
    _redis_events: Vec<RDeltaEvent>,
) -> Result<(), E> {
    unimplemented!()
}
