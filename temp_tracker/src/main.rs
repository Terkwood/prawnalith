#![feature(slice_concat_ext)]
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate envy;
extern crate redis;
extern crate rumqtt;
extern crate uuid;

use std::thread;
use std::time::Duration;

use rumqtt::{MqttCallback, QoS};
use rumqtt::{MqttClient, MqttOptions};
use uuid::Uuid;

mod config;
pub mod model;
mod prawnqtt;
mod predis;

fn main() {
    dotenv::dotenv().expect("Unable to load .env file");

    let config = config::Config::new();

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
        predis::RedisContext::new(
            redis_host.to_string(),
            redis_port,
            redis_auth,
            config.redis_namespace.unwrap_or("".to_string()),
        )
    };

    let external_device_namespace = redis_ctx.get_external_device_namespace().unwrap();
    println!("external device namespace is {}", external_device_namespace);

    let callback = move |msg: rumqtt::Message| {
        println!("Received payload:\n\t{:?}", msg);
        let deser: Result<model::TempMessage, _> =
            serde_json::from_str(std::str::from_utf8(&*msg.payload).unwrap());
        match deser {
            Err(_) => println!("\t[!] couldn't deserialize [!]"),
            Ok(temp) => {
                println!("\t{:?}", temp);
                println!(
                    "\tInternal ID for device: {}",
                    model::compute_internal_id(&temp.device_id, &external_device_namespace).unwrap()
                );
            }
        }
    };

    let mq_message_callback = MqttCallback::new().on_message(callback);

    // Specify client connection options
    let opts: MqttOptions = MqttOptions::new()
        .set_keep_alive(*mq_keep_alive)
        .set_reconnect(3)
        .set_client_id(prawnqtt::generate_mq_client_id())
        .set_broker(&format!("{}:{}", mq_host, mq_port)[..]);
    let mq_cli = MqttClient::start(opts, Some(mq_message_callback))
        .expect("MQTT client couldn't start")
        .subscribe(vec![(mq_topic, QoS::Level0)]);

    // next:
    // deserialize json from temp sensor channel
    // query & update redis
    // publish message to led channel
    thread::sleep(Duration::from_secs(std::u64::MAX));
}
