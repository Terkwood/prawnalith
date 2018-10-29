#![feature(slice_concat_ext)]
extern crate dotenv;
extern crate envy;
extern crate paho_mqtt;
extern crate redis;
extern crate redis_context;
#[macro_use]
extern crate serde_derive;
extern crate tracker_support;
extern crate uuid;

use std::thread;
use std::time::Duration;

use tracker_support::TrackerConfig;

mod model;
mod prawnqtt;
mod predis;

fn main() {
    dotenv::dotenv().expect("Unable to load .env file");

    let config = TrackerConfig::new();
    let config_clone = config.clone();

    let rx = tracker_support::start_mqtt(&config);

    predis::receive_updates(rx, &config_clone.to_redis_context());

    thread::sleep(Duration::from_secs(std::u64::MAX));
}
