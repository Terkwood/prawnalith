use uuid::Uuid;

/// This message is emitted to an MQTT channel by
/// some device with access to a temp sensor (DS18B20, etc)
#[derive(Serialize, Deserialize, Debug)]
pub struct SensorMessage {
    pub device_id: String,
    pub temp_f: Option<f64>,
    pub temp_c: Option<f64>,
    pub ph: Option<f64>,
    pub mv: Option<f64>,
}

/// `external_device_id` is usually reported as a
/// e.g. "28654597090000e4"
impl SensorMessage {
    pub fn measurements(&self) -> Vec<Measurement> {
        let mut v: Vec<Measurement> = vec![];
        if let (Some(temp_f), Some(temp_c)) = (self.temp_f, self.temp_c) {
              v.push(Measurement::Temp { temp_f, temp_c })
        }

        if let     (Some(ph), Some(mv)) = (self.ph, self.mv) {
             v.push(Measurement::PH { ph, mv })
            
        }

        v
    }
}

pub struct DeviceId {
    pub external_id: String
}

impl DeviceId {
    pub fn id(&self, external_device_namespace: &Uuid) -> Result<Uuid, uuid::parser::ParseError> {
        Ok(Uuid::new_v5(
            &external_device_namespace,
            self.external_id.as_bytes(),
        ))
    }
}

#[derive(Debug)]
pub enum Measurement {
    Temp { temp_f: f64, temp_c: f64 },
    PH { ph: f64, mv: f64 }
}

impl Measurement {
    pub fn name(&self) -> String {
        match self {
            Measurement::Temp { temp_f, temp_c } => "temp".to_string(),
            Measurement::PH { ph, mv } => "ph".to_string()
        }
    }
}