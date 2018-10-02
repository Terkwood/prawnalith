#![feature(slice_concat_ext)]
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate envy;
extern crate redis;

use std::net::TcpStream;

use mqtt::control::variable_header::ConnectReturnCode;
use mqtt::packet::*;
use mqtt::TopicFilter;
use mqtt::{Decodable, Encodable, QualityOfService};

use redis::Commands;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
struct Config {
    redis_auth: Option<String>,
    redis_host: Option<String>,
    redis_port: Option<u16>,
    redis_namespace: Option<String>,
    mqtt_host: Option<String>,
    mqtt_port: Option<u16>,
    mqtt_topic: String,
    mqtt_keep_alive: Option<u16>,
}

fn generate_mqtt_client_id() -> String {
    format!("sensor_tracker/{}", Uuid::new_v4())
}

fn redis_connection_string(host: &str, port: Option<u16>, auth: Option<String>) -> String {
    let auth_string = match auth {
        Some(a) => format!(":{}@", a),
        None => "".to_string(),
    };

    let port_portion: String = port.map(|p| format!(":{}", p)).unwrap_or("".to_string());

    format!("redis://{}{}:{}", auth_string, host, port_portion)
}

fn main() {
    dotenv::dotenv().expect("Unable to load .env file");

    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(e) => panic!("Unable to parse config ({})", e),
    };

    let redis_host = &config.redis_host.unwrap_or("127.0.0.1".to_string());
    let redis_port: Option<u16> = config.redis_port;
    let redis_namespace = &config.redis_namespace.unwrap_or("".to_string());
    let redis_auth: Option<String> = config.redis_auth;

    let redis_client =
        redis::Client::open(&redis_connection_string(redis_host, redis_port, redis_auth)[..])
            .unwrap();
    let redis_conn = redis_client.get_connection().unwrap();

    let mqtt_host = &config.mqtt_host.unwrap_or("127.0.0.1".to_string());
    let mqtt_port = &config.redis_port.unwrap_or(1883);
    // mqtt spec states that this is measured in secs
    // see http://www.steves-internet-guide.com/mqtt-keep-alive-by-example/
    let mqtt_keep_alive = &config.mqtt_keep_alive.unwrap_or(10);
    let mqtt_topic = &config.mqtt_topic;
    let mqtt_channel_filter: (TopicFilter, QualityOfService) = (
        TopicFilter::new(mqtt_topic.to_string()).unwrap(),
        QualityOfService::Level0,
    );
    let mqtt_server_addr = format!("{}:{}", mqtt_host, mqtt_port);
    println!("Connecting to MQTT server {:?} ... ", mqtt_server_addr);
    let mut mqtt_stream = TcpStream::connect(mqtt_server_addr).unwrap();
    println!("Connected!");

    let z = i64::from_str_radix("1f", 16);
}
