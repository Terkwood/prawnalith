use uuid::Uuid;

/// This enum represents various keys which should
/// exist in our database.  They each have a namespace
/// parameter `ns`, which indicates a common "root"
/// shared by all data for this particular prawn grow.
pub enum Key<'a, 'b> {
    Tank {
        ns: Namespace<'a>,
        id: u16,
    },
    Sensor {
        ns: Namespace<'a>,
        st: SensorType<'b>,
        id: Uuid,
    },
    AllTanks {
        ns: Namespace<'a>,
    },
    AllSensorTypes {
        ns: Namespace<'a>,
    },
    AllSensors {
        ns: Namespace<'a>,
        st: SensorType<'b>,
    },
}

/// Namespace precedes the rest of a key, e.g.
/// `prawnspace/tanks`
#[derive(Copy, Clone)]
pub struct Namespace<'a>(pub &'a str);

/// A type of sensor.  ph, temp, ...
#[derive(Copy, Clone)]
pub struct SensorType<'a>(pub &'a str);

/// Yields the key which allows you to access a specific
/// record in redis.
impl<'a, 'b> Key<'a, 'b> {
    pub fn key(&self) -> String {
        match self {
            Key::Tank { ns, id } => format!("{}/{}", Key::AllTanks { ns: *ns }.key(), id)
                .trim()
                .to_lowercase(),
            Key::Sensor { ns, st, id } => {
                format!("{}/{}", Key::AllSensors { ns: *ns, st: *st }.key(), id)
                    .trim()
                    .to_lowercase()
            }
            Key::AllTanks { ns: Namespace(n) } => format!("{}/tanks", n).to_lowercase(),
            Key::AllSensorTypes { ns: Namespace(n) } => {
                format!("{}/sensors", n).trim().to_lowercase()
            }
            Key::AllSensors {
                ns,
                st: SensorType(st),
            } => format!("{}/{}", Key::AllSensorTypes { ns: *ns }.key(), st)
                .trim()
                .to_lowercase(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use uuid::Uuid;

    fn prawnspace() -> Namespace<'static> {
        Namespace("prawnspace")
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
            st: SensorType("ph"),
        };
        assert_eq!(all_sensors.key(), format!("prawnspace/sensors/ph"));
    }

    #[test]
    fn test_temp_sensor() {
        let temp_id = Uuid::new_v4();
        let temp_sensor = Key::Sensor {
            ns: prawnspace(),
            st: SensorType("TEMP"),
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
            ns: Namespace("prawnspace"),
            st: SensorType("ph"),
            id: ph_id,
        };
        assert_eq!(ph_sensor.key(), format!("prawnspace/sensors/ph/{}", ph_id));
    }

}
