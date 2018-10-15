#![feature(slice_concat_ext)]
extern crate crossbeam_channel;
extern crate dotenv;
extern crate envy;
extern crate redis;
extern crate redis_context;
extern crate rumqtt;
#[macro_use]
extern crate serde_derive;
extern crate tracker_support;
extern crate uuid;

use std::thread;
use std::time::Duration;

use redis_context::RedisContext;
use tracker_support::TrackerConfig;

mod model;
mod prawnqtt;
mod predis;

fn main() {
    dotenv::dotenv().expect("Unable to load .env file");

    let config = TrackerConfig::new();

    // DEFAULT CONFIGURATIONS LIVE HERE!
    let mq_host = &config.mqtt_host.unwrap_or("127.0.0.1".to_string());
    let mq_port = &config.mqtt_port.unwrap_or(1883);
    // mqtt spec states that this is measured in secs
    // see http://www.steves-internet-guide.com/mqtt-keep-alive-by-example/
    let mq_keep_alive = &config.mqtt_keep_alive.unwrap_or(10);
    let mq_topic = &config.mqtt_topic;

    let redis_ctx = {
        let redis_host = &config.redis_host.unwrap_or("127.0.0.1".to_string());
        let redis_port: u16 = config.redis_port.unwrap_or(6379);
        let redis_auth: Option<String> = config.redis_auth;
        RedisContext::new(
            redis_host.to_string(),
            redis_port,
            redis_auth,
            config.redis_namespace.unwrap_or("".to_string()),
        )
    };

    let (update_s, update_r) = crossbeam_channel::bounded(5);

    thread::spawn(move || predis::receive_updates(update_r, &redis_ctx));

    let _ = tracker_support::start_mqtt(
        prawnqtt::create_mqtt_callback(update_s),
        mq_host,
        *mq_port,
        &mq_topic,
        *mq_keep_alive,
    );

    thread::sleep(Duration::from_secs(std::u64::MAX));
}
