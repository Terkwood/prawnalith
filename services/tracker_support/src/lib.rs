extern crate paho_mqtt;
extern crate redis_context;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

use paho_mqtt::message::Message;
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
    pub mqtt_qos: Option<u16>,
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

pub fn start_mqtt(
    config: &TrackerConfig,
) -> (
    std::sync::mpsc::Receiver<Option<Message>>,
    paho_mqtt::Client,
) {
    // DEFAULT CONFIGURATIONS LIVE HERE!
    let host = &config.mqtt_host.clone().unwrap_or("127.0.0.1".to_string());
    let port = &config.mqtt_port.clone().unwrap_or(1883);
    // mqtt spec states that this is measured in secs
    // see http://www.steves-internet-guide.com/mqtt-keep-alive-by-example/
    let keep_alive = &config.mqtt_keep_alive.unwrap_or(10);
    let topic = &config.mqtt_topic;
    let qos = &config.mqtt_qos.unwrap_or(1);

    let server_uri = format!("tcp://{}:{}", host, port);
    let server_uri_print = server_uri.clone();

    // Create the client. Use an ID for a persisten session.
    // A real system should try harder to use a unique ID.
    let create_opts = paho_mqtt::CreateOptionsBuilder::new()
        .server_uri(server_uri)
        .client_id(generate_mq_client_id())
        .finalize();

    let mut cli = paho_mqtt::Client::new(create_opts).expect("Error creating the MQTT client");

    // Define the set of options for the connection
    let will = paho_mqtt::MessageBuilder::new()
        .topic(topic.to_string())
        .payload("Sync consumer lost connection")
        .finalize();

    let conn_opts = paho_mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(std::time::Duration::from_secs(*keep_alive as u64))
        .clean_session(false)
        .will_message(will)
        .finalize();

    // Make the connection to the broker
    println!("Connecting to the MQTT broker at {}", server_uri_print);
    if let Err(e) = cli.connect(conn_opts) {
        println!("Error connecting to the broker: {:?}", e);
        std::process::exit(1);
    };

    // Initialize the consumer & subscribe to topics
    println!("Subscribing to topic {}", topic);
    let rx = cli.start_consuming();

    if let Err(e) = cli.subscribe_many(&[topic], &[*qos as i32]) {
        println!("Error subscribing to topics: {:?}", e);
        cli.disconnect(None).unwrap();
        std::process::exit(1);
    };

    (rx, cli)
}

pub fn try_mqtt_reconnect(cli: &paho_mqtt::Client) -> bool {
    println!("MQTT connection lost...");
    for i in 0..12 {
        println!("Retrying MQTT connection ({})", i);
        std::thread::sleep(std::time::Duration::from_millis(5000));
        if cli.reconnect().is_ok() {
            println!("MQTT successfully reconnected");
            return true;
        }
    }
    println!("Unable to reconnect MQTT after several attempts.");
    false
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
