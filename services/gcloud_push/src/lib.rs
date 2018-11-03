//! # gcloud_push
//!
//! This is a service which pushes temperature and pH
//! data to google cloud pub sub.  The temp & pH
//! data is expected to reside in a Redis instance.

mod model;

/// On startup, clone all the different paths
/// relevant to our Redis database, and send them upstream
/// to the cloud instance via pub sub.
///
/// After we've started up, listen for
/// updates to various records and propagate them
/// when changed.
fn main() {
    println!("Hello, world!");
}
