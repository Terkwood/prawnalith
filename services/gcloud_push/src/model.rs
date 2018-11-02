use uuid::Uuid;

/// This enum represents various keys which should
/// exist in our database.  They each have a namespace
/// parameter `ns`, which indicates a common "root"
/// shared by all data for this particular prawn grow.
enum DataKey {
    Tank { ns: String, id: u16 },
    SensorTemp { ns: String, id: Uuid },
    SensorPH { ns: String, id: Uuid },
    AllTanks { ns: String },
    AllSensorTypes { ns: String },
    AllSensorsTemp { ns: String },
    AllSensorsPH { ns: String },
}

impl DataKey {
    /// Yields the key which allows you to access a specific
    /// record in redis.
    ///
    /// # Examples
    /// ```
    /// let all_tanks = AllTanks { "prawnspace" }
    /// assert_eq!(all_tanks.data_key(), "prawnspace/tanks")
    /// ```
    pub fn data_key(&self) -> String {
        match self {
            DataKey::Tank { ns, id } => format!(
                "{}/{}",
                DataKey::AllTanks { ns: ns.to_string() }.data_key(),
                id
            ),
            DataKey::SensorTemp { ns, id } => format!(
                "{}/{}",
                DataKey::AllSensorsTemp { ns: ns.to_string() }.data_key(),
                id
            ),
            DataKey::SensorPH { ns, id } => format!(
                "{}/{}",
                DataKey::AllSensorsPH { ns: ns.to_string() }.data_key(),
                id
            ),
            DataKey::AllTanks { ns } => format!("{}/tanks", ns),
            DataKey::AllSensorTypes { ns } => format!("{}/sensors", ns),
            DataKey::AllSensorsTemp { ns } => format!(
                "{}/temp",
                DataKey::AllSensorTypes { ns: ns.to_string() }.data_key()
            ),
            DataKey::AllSensorsPH { ns } => format!(
                "{}/ph",
                DataKey::AllSensorTypes { ns: ns.to_string() }.data_key()
            ),
        }
    }
}
