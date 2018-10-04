#![feature(slice_concat_ext)]
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate envy;
extern crate redis;
extern crate rumqtt;
extern crate uuid;

use rumqtt::MqttCallback;

use uuid::Uuid;

mod config;
mod prawnqtt;
mod predis;
mod temp_message;

/// `external_device_id` is usually reported as a
/// e.g. "28654597090000e4"
fn compute_internal_id(
    external_device_id: &str,
    external_device_namespace: Uuid,
) -> Result<Uuid, uuid::parser::ParseError> {
    Ok(Uuid::new_v5(
        &external_device_namespace,
        external_device_id.as_bytes(),
    ))
}

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

    let external_device_namespace = &redis_ctx.get_external_device_namespace().unwrap();
    println!("external device namespace is {}", external_device_namespace);

    let _mq_req_handler = prawnqtt::mq_client(mq_host, *mq_port, *mq_keep_alive);

    // next:
    // deserialize json from temp sensor channel
    // query & update redis
    // publish message to led channel
    loop {}
}
