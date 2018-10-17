use uuid::Uuid;

/// Message emitted to an MQTT channel by some device
/// with access to a pH sensor (SEN0169)

#[derive(Serialize, Deserialize, Debug)]
pub struct PhMessage {
    pub device_id: String,
    pub ph: f64,
    pub ph_mv: f64,
}

/// `external_device_id` is usually reported as a
/// e.g. "28654597090000e4"
impl PhMessage {
    pub fn id(&self, external_device_namespace: &Uuid) -> Result<Uuid, uuid::parser::ParseError> {
        Ok(Uuid::new_v5(
            &external_device_namespace,
            self.device_id.as_bytes(),
        ))
    }
}
