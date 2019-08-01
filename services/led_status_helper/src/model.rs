/// This data is stored on a redis device record
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct SensorReadings {
    pub temp_f: Option<f64>,
    pub temp_c: Option<f64>,
    pub ph: Option<f64>,
    pub humidity: Option<f64>,
    pub heat_index_c: Option<f64>,
    pub heat_index_f: Option<f64>,
    pub update_time: Option<u64>,
}
