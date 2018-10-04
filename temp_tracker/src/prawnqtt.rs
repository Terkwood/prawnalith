use super::model;

use uuid::Uuid;
/*
pub fn mq_client(mq_host: &str, mq_port: u16, mq_keep_alive: u16) -> rumqtt::MqttClient {
    }
*/
pub fn generate_mq_client_id() -> String {
    format!("sensor_tracker/{}", Uuid::new_v4())
}
