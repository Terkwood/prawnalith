#![feature(slice_concat_ext)]
extern crate dotenv;
extern crate envy;
extern crate paho_mqtt;
extern crate redis;
extern crate redis_context;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

mod config;
mod logic;
mod model;
mod prawnqtt;
mod predis;

fn main() {
    dotenv::dotenv().expect("Unable to load .env file");

    let config = config::TrackerConfig::new();
    let config_clone = config.clone();

    let (rx, mqtt_cli) = prawnqtt::start_mqtt(&config);

    logic::receive_updates(rx, &config_clone.to_redis_context(), mqtt_cli);
    println!("unreachable");
}
