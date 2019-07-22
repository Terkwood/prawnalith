/// This message is emitted to an MQTT channel by
/// some device with access to a temp sensor (DS18B20, etc)
/// `external_device_id` is usually reported as a
/// e.g. "28654597090000e4"

#[derive(Serialize, Deserialize, Debug)]
pub struct SensorMessage {
    pub device_id: String,
    pub temp_f: Option<f64>,
    pub temp_c: Option<f64>,
    pub ph: Option<f64>,
    pub ph_mv: Option<f64>,
    pub status: Option<String>,
    pub humidity: Option<f64>,
    pub heat_index_c: Option<f64>,
    pub heat_index_f: Option<f64>,
}

impl SensorMessage {
    pub fn to_redis(&self) -> Vec<(&str, String)> {
        let mut data = vec![];
        if let Some(s) = &self.status {
            data.push(("status", s.to_string()));
        }
        if let Some(humidity) = self.humidity {
            data.push(("humidity", humidity.to_string()));
        }
        if let Some(tf) = self.temp_f {
            data.push(("temp_f", tf.to_string()));
        }
        if let Some(tc) = self.temp_c {
            data.push(("temp_c", tc.to_string()));
        }
        if let Some(hf) = self.heat_index_f {
            data.push(("heat_index_f", hf.to_string()));
        }
        if let Some(hc) = self.heat_index_c {
            data.push(("heat_index_c", hc.to_string()));
        }
        if let Some(ph) = self.ph {
            data.push(("ph", ph.to_string()))
        }
        if let Some(ph_mv) = self.ph_mv {
            data.push(("ph_mv", ph_mv.to_string()))
        }
        data
    }
}
