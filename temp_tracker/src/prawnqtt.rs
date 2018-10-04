use rumqtt::{MqttCallback, MqttClient, MqttOptions, QoS};
use uuid::Uuid;

fn generate_mq_client_id() -> String {
    format!("sensor_tracker/{}", Uuid::new_v4())
}
pub fn mq_request_handler(
    mq_message_callback: MqttCallback,
    mq_host: &str,
    mq_port: u16,
    mq_keep_alive: u16,
) -> rumqtt::MqttClient {
    // Specify client connection options
    let opts: MqttOptions = MqttOptions::new()
        .set_keep_alive(mq_keep_alive)
        .set_reconnect(3)
        .set_client_id(generate_mq_client_id())
        .set_broker(&format!("{}:{}", mq_host, mq_port)[..]);
    MqttClient::start(opts, Some(mq_message_callback)).expect("MQTT client couldn't start")
}
