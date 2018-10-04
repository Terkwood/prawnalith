use uuid::Uuid;

/// This message is emitted to an MQTT channel by
/// some device with access to a temp sensor (DS18B20, etc)
#[derive(Serialize, Deserialize, Debug)]
pub struct TempMessage {
    pub device_id: String,
    pub temp_f: f64,
    pub temp_c: f64,
}

/// `external_device_id` is usually reported as a
/// e.g. "28654597090000e4"
pub fn compute_internal_id(
    external_device_id: &str,
    external_device_namespace: &Uuid,
) -> Result<Uuid, uuid::parser::ParseError> {
    Ok(Uuid::new_v5(
        &external_device_namespace,
        external_device_id.as_bytes(),
    ))
}
