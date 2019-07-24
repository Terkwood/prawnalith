use super::model;

use rumqtt::{MqttClient, MqttOptions, QoS, ReconnectOptions};
use std::{thread, time::Duration};

use super::config::TrackerConfig;
use crossbeam::Receiver;
use uuid::Uuid;

struct Client {}
struct Message {}

pub fn start_mqtt(config: &TrackerConfig) -> (Receiver<Option<Message>>, Client) {
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

    (unimplemented!(), unimplemented!())
}

fn DEAD_start_paho_mqtt(
    config: &TrackerConfig,
) -> (
    std::sync::mpsc::Receiver<Option<Message>>,
    Client,
) {

    unimplemented!();

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

fn generate_mq_client_id() -> String {
    format!("sensor_tracker/{}", Uuid::new_v4())
}

pub fn deser_message(msg: paho_mqtt::Message) -> Option<model::SensorMessage> {
    let r = std::str::from_utf8(&*msg.payload());
    r.ok()
        .and_then(|s| serde_json::from_str(s).map(|r| Some(r)).unwrap_or(None))
}
