use super::model;

use super::config::TrackerConfig;
use crossbeam::Receiver;
use rumqtt::{Message, MqttClient, MqttOptions, Notification, Publish, QoS, ReconnectOptions};
use std::sync::Arc;
use std::{thread, time::Duration};
use uuid::Uuid;

pub fn start_mqtt(config: &TrackerConfig) -> (Receiver<Option<Message>>, MqttClient) {
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

    let reconnection_options = ReconnectOptions::Always(10);
    let mqtt_options = MqttOptions::new(generate_mq_client_id(), host, *port)
        .set_keep_alive(*keep_alive)
        .set_reconnect_opts(reconnection_options)
        .set_clean_session(false);

    let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();
    mqtt_client
        .subscribe(topic, QoS::from_u8(*qos).expect("qos"))
        .unwrap();

    let (msg_in, msg_out) = crossbeam_channel::unbounded();

    thread::spawn(move || {
        for notification in notifications {
            match notification {
                Notification::Publish(p) => {
                    if let Err(e) = msg_in.send(deser_message(p.payload)) {
                        println!("err sending {:?}", e)
                    }
                }
            }
        }
    });

    (msg_out, mqtt_client)
}

fn DEAD_start_paho_mqtt(
    config: &TrackerConfig,
) -> (std::sync::mpsc::Receiver<Option<Message>>, MqttClient) {
    unimplemented!();
    /*
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
    */
}

fn generate_mq_client_id() -> String {
    format!("sensor_tracker/{}", Uuid::new_v4())
}

pub fn deser_message(payload: Arc<Vec<u8>>) -> Option<model::SensorMessage> {
    let r = std::str::from_utf8(payload);
    r.ok()
        .and_then(|s| serde_json::from_str(s).map(|r| Some(r)).unwrap_or(None))
}
