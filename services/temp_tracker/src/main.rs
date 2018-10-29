#![feature(slice_concat_ext)]
extern crate crossbeam_channel;
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

    let (update_s, update_r) = crossbeam_channel::bounded(5);

    thread::spawn(move || predis::receive_updates(update_r, &config_clone.to_redis_context()));

    let _ = prawnqtt::do_something_with_paho();

    // TODO RIP ☠️
    //let _ = tracker_support::start_mqtt(prawnqtt::create_mqtt_callback(update_s), &config);

    thread::sleep(Duration::from_secs(std::u64::MAX));
}
