#![feature(slice_concat_ext)]
#![feature(bind_by_move_pattern_guards)]
extern crate dotenv;
extern crate envy;
extern crate paho_mqtt;
extern crate redis;
extern crate redis_context;
extern crate redis_delta;
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

    let redis_ctx = &config_clone.to_redis_context();
    let delta_event_topic = config
        .redis_delta_event_topic
        .unwrap_or(format!("{}/system/delta_events", &redis_ctx.namespace));
    logic::receive_updates(rx, redis_ctx, mqtt_cli, &delta_event_topic)
}
