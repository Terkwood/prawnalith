/// Fetch from the "pond" service, which will return our tank data
impl PondService {
     pub fn new() -> Self {
        Self {
            web: FetchService::new(),
        }
}
}


/// A struct to hold data returned by the HTTP request
/// for tanks' temp & ph info.
#[derive(Debug, Deserialize)]
pub struct Tank {
    pub id: u16,
    pub name: Option<String>,
    pub temp_f: Option<f32>,
    pub temp_c: Option<f32>,
    pub temp_update_time: Option<u64>,
    pub temp_update_count: Option<u32>,

    pub ph: Option<f32>,
    pub ph_mv: Option<f32>,
    pub ph_update_time: Option<u64>,
    pub ph_update_count: Option<u32>,
}
