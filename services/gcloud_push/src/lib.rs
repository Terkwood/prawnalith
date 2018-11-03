//! # gcloud_push
//!
//! This is a service which pushes temperature and pH
//! data to google cloud pub sub.  The temp & pH
//! data is expected to reside in a Redis instance.

extern crate redis_context;

pub mod data;

use redis_context::RedisContext;

/// send *all* relevant redis data upstream
/// to the cloud instance via pub sub
pub fn clone_the_world(redis_ctx: &RedisContext) {}

/// pushes some recent data via gcloud pubsub
pub fn push() {}
