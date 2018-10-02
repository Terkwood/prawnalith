#![feature(slice_concat_ext)]
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate envy;
extern crate redis;

use redis::Commands;
use uuid::Uuid;

fn generate_client_id() -> String {
    format!("sensor_tracker/{}", Uuid::new_v4())
}

#[derive(Deserialize, Debug, Clone)]
struct Config {
    redis_auth: Option<String>,
    redis_host: Option<String>,
    redis_port: Option<u16>,
    redis_namespace: Option<String>,
    mqtt_host: Option<String>,
    mqtt_port: Option<u16>,
    mqtt_topic: String,
}

fn main() {
    dotenv::dotenv().expect("Unable to load .env file");

    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(e) => panic!("Unable to parse config ({})", e),
    };

    let redis_host = &config.redis_host.unwrap_or("127.0.0.1".to_string());
    let redis_port = &config.redis_port.unwrap_or(6379);
    let redis_namespace = &config.redis_namespace.unwrap_or("".to_string());
    let mqtt_host = &config.mqtt_host.unwrap_or("127.0.0.1".to_string());
    let mqtt_port = &config.redis_port.unwrap_or(1883);

    let z = i64::from_str_radix("1f", 16);
}
