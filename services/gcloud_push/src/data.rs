use uuid::Uuid;

/// This enum represents various keys which should
/// exist in our database.  They each have a namespace
/// parameter `ns`, which indicates a common "root"
/// shared by all data for this particular prawn grow.
pub enum Key {
    Tank {
        ns: Namespace,
        id: u16,
    },
    Sensor {
        ns: Namespace,
        st: SensorType,
        id: Uuid,
    },
    AllTanks {
        ns: Namespace,
    },
    AllSensorTypes {
        ns: Namespace,
    },
    AllSensors {
        ns: Namespace,
        st: SensorType,
    },
}

pub struct Namespace(pub String);

pub struct SensorType(String);
impl SensorType {
    pub fn new(sensor_type: &str) -> SensorType {
        SensorType(sensor_type.to_lowercase().trim().to_string())
    }
}

/// Yields the key which allows you to access a specific
/// record in redis.
impl Key {
    pub fn key(&self) -> String {
        match self {
            Key::Tank {
                ns: Namespace(n),
                id,
            } => format!(
                "{}/{}",
                Key::AllTanks {
                    ns: Namespace(n.to_string())
                }
                .key(),
                id
            ),
            Key::Sensor {
                ns: Namespace(n),
                st: SensorType(st),
                id,
            } => format!(
                "{}/{}",
                Key::AllSensors {
                    ns: Namespace(n.to_string()),
                    st: SensorType(st.to_string())
                }
                .key(),
                id
            ),
            Key::AllTanks { ns: Namespace(n) } => format!("{}/tanks", n),
            Key::AllSensorTypes { ns: Namespace(n) } => format!("{}/sensors", n),
            Key::AllSensors {
                ns: Namespace(n),
                st: SensorType(st),
            } => format!(
                "{}/{}",
                Key::AllSensorTypes {
                    ns: Namespace(n.to_string())
                }
                .key(),
                st
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use uuid::Uuid;

    fn prawnspace() -> Namespace {
        Namespace("prawnspace".to_string())
    }

    #[test]
    fn test_all_tanks() {
        let all_tanks = Key::AllTanks { ns: prawnspace() };
        assert_eq!(all_tanks.key(), "prawnspace/tanks");
    }

    #[test]
    fn test_single_tank() {
        let single_tank = Key::Tank {
            ns: prawnspace(),
            id: 1,
        };
        assert_eq!(single_tank.key(), "prawnspace/tanks/1");
    }

    #[test]
    fn test_all_sensors() {
        let all_sensors = Key::AllSensors {
            ns: prawnspace(),
            st: SensorType::new("ph"),
        };
        assert_eq!(all_sensors.key(), format!("prawnspace/sensors/ph"));
    }

    #[test]
    fn test_temp_sensor() {
        let temp_id = Uuid::new_v4();
        let temp_sensor = Key::Sensor {
            ns: prawnspace(),
            st: SensorType::new("temp"),
            id: temp_id,
        };
        assert_eq!(
            temp_sensor.key(),
            format!("prawnspace/sensors/temp/{}", temp_id)
        );
    }

    #[test]
    fn test_ph_sensor() {
        let ph_id = Uuid::new_v4();
        let ph_sensor = Key::Sensor {
            ns: Namespace("prawnspace".to_string()),
            st: SensorType::new("ph"),
            id: ph_id,
        };
        assert_eq!(ph_sensor.key(), format!("prawnspace/sensors/ph/{}", ph_id));
    }

}
