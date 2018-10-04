use uuid::Uuid;

pub fn generate_mq_client_id() -> String {
    format!("sensor_tracker/{}", Uuid::new_v4())
}
