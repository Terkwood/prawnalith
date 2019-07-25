use crate::config::TrackerConfig;
use crossbeam::Receiver;
use rumqtt::{MqttClient, MqttOptions, Notification, QoS, ReconnectOptions};
use uuid::Uuid;

pub fn start_mqtt(config: &TrackerConfig) -> Receiver<Notification> {
    // DEFAULT CONFIGURATIONS LIVE HERE!
    let host = &config.mqtt_host.clone().unwrap_or("127.0.0.1".to_string());
    let port = &config.mqtt_port.clone().unwrap_or(1883);
    // mqtt spec states that this is measured in secs
    // see http://www.steves-internet-guide.com/mqtt-keep-alive-by-example/
    let keep_alive = &config.mqtt_keep_alive.unwrap_or(10);
    let topic = &config.mqtt_topic;
    let qos = &config.mqtt_qos.unwrap_or(1);

    let reconnection_options = ReconnectOptions::Always(10);
    let mqtt_options = MqttOptions::new(generate_mq_client_id(), host, *port)
        .set_keep_alive(*keep_alive)
        .set_reconnect_opts(reconnection_options)
        .set_clean_session(false);

    let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();

    mqtt_client
        .subscribe(topic, QoS::from_u8(*qos).expect("qos"))
        .unwrap();

    notifications
}

fn generate_mq_client_id() -> String {
    format!("sensor_tracker/{}", Uuid::new_v4())
}
