/// This message is emitted to an MQTT channel by
/// some device with access to a temp sensor (DS18B20, etc)
#[derive(Serialize, Deserialize, Debug)]
pub struct TempMessage {
    pub device_id: String,
    pub temp_f: f64,
    pub temp_c: f64,
}
