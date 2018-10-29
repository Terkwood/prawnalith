extern crate paho_mqtt;
extern crate redis_context;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

use redis_context::RedisContext;
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
pub struct TrackerConfig {
    pub redis_auth: Option<String>,
    pub redis_host: Option<String>,
    pub redis_port: Option<u16>,
    pub redis_namespace: Option<String>,
    pub mqtt_host: Option<String>,
    pub mqtt_port: Option<u16>,
    pub mqtt_topic: String,
    pub mqtt_keep_alive: Option<u16>,
}

impl TrackerConfig {
    pub fn new() -> TrackerConfig {
        match envy::from_env::<TrackerConfig>() {
            Ok(config) => config,
            Err(e) => panic!("Unable to parse config ({})", e),
        }
    }

    pub fn to_redis_context(&self) -> RedisContext {
        RedisContext::new(
            (self.redis_host.clone().unwrap_or("127.0.0.1".to_string())).to_string(),
            self.redis_port.unwrap_or(6379),
            self.redis_auth.clone(),
            self.redis_namespace.clone().unwrap_or("".to_string()),
        )
    }
}

pub fn start_mqtt(/*mq_message_callback: MqttCallback, */ config: &TrackerConfig) {
    // DEFAULT CONFIGURATIONS LIVE HERE!
    let host = &config.mqtt_host.clone().unwrap_or("127.0.0.1".to_string());
    let port = &config.mqtt_port.clone().unwrap_or(1883);
    // mqtt spec states that this is measured in secs
    // see http://www.steves-internet-guide.com/mqtt-keep-alive-by-example/
    let keep_alive = &config.mqtt_keep_alive.unwrap_or(10);
    let topic = &config.mqtt_topic;
    // Specify client connection options
    /*let opts: MqttOptions = MqttOptions::new()
        .set_keep_alive(*keep_alive)
        .set_reconnect(3)
        .set_client_id(generate_mq_client_id())
        .set_broker(&format!("{}:{}", host, port)[..]);
    
    MqttClient::start(opts, Some(mq_message_callback))
        .expect("MQTT client couldn't start")
        .subscribe(vec![(topic, QoS::Level0)])
        .unwrap()*/
    unimplemented!()
}

fn generate_mq_client_id() -> String {
    format!("sensor_tracker/{}", Uuid::new_v4())
}

pub fn epoch_secs() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
