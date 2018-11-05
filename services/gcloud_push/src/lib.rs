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

use redis_context::RedisContext;
use redis_delta::RDelta;

use self::model::{PubSubClient, PubSubContext};
use self::pubsub::{PublishRequest, PubsubMessage};
use std::default::Default;

pub fn hello_world(_redis_ctx: &RedisContext, pubsub_ctx: &PubSubContext) {
    let mut msg = PubsubMessage::default();
    msg.data = Some("HELLO ANYBODY PLEAES!".to_string());
    let req = PublishRequest {
        messages: Some(vec![msg]),
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

/// pushes some recent data via gcloud pubsub
pub fn push_recent<E>(
    _redis_context: &RedisContext,
    _pubsub: &PubSubClient,
    _rdeltas: Vec<RDelta>,
) -> Result<(), E> {
    unimplemented!()
}
