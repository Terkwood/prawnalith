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

#[derive(Debug)]
pub enum Measurement {
    Temp {
        temp_f: f64,
        temp_c: f64,
    },
    PH {
        ph: f64,
        ph_mv: f64,
    },
    /// Digital humidity and temp, e.g. DHT11 sensor
    DHT {
        status: String,
        humidity: f64,
        temp_f: f64,
        temp_c: f64,
        heat_index_f: f64,
        heat_index_c: f64,
    },
}

impl Measurement {
    pub fn name(&self) -> String {
        match self {
            Measurement::Temp {
                temp_f: _,
                temp_c: _,
            } => "temp".to_string(),
            Measurement::PH { ph: _, ph_mv: _ } => "ph".to_string(),
            Measurement::DHT {
                status: _,
                humidity: _,
                temp_f: _,
                temp_c: _,
                heat_index_f: _,
                heat_index_c: _,
            } => "dht".to_string(),
        }
    }

    pub fn to_redis(&self) -> Vec<(&str, String)> {
        match self {
            Measurement::Temp { temp_f, temp_c } => vec![
                ("temp_f", temp_f.to_string()),
                ("temp_c", temp_c.to_string()),
            ],
            Measurement::PH { ph, ph_mv } => {
                vec![("ph", ph.to_string()), ("ph_mv", ph_mv.to_string())]
            }
            Measurement::DHT {
                status,
                humidity,
                temp_f,
                temp_c,
                heat_index_f,
                heat_index_c,
            } => vec![
                ("status", status.to_string()),
                ("humidity", humidity.to_string()),
                ("temp_f", temp_f.to_string()),
                ("temp_c", temp_c.to_string()),
                ("heat_index_f", heat_index_f.to_string()),
                ("heat_index_c", heat_index_c.to_string()),
            ],
        }
    }
}
