#![feature(custom_attribute)]
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use uuid::Uuid;

/// This enum represents various keys which should
/// exist in our database.  They each have a namespace
/// parameter `ns`, which indicates a common "root"
/// shared by all data for this particular prawn grow.
#[derive(Serialize, Deserialize)]
pub enum Key<'a, 'b> {
    Tank {
        #[serde(borrow)]
        ns: Namespace<'a>,
        id: u16,
    },
    Sensor {
        ns: Namespace<'a>,
        #[serde(borrow)]
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
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Namespace<'a>(pub &'a str);

/// A type of sensor.  ph, temp, ...
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct SensorType<'a>(pub &'a str);

/// Yields the key which allows you to access a specific
/// record in redis.
impl<'a, 'b> Key<'a, 'b> {
    pub fn to_string(&self) -> String {
        match self {
            Key::Tank { ns, id } => format!("{}/{}", Key::AllTanks { ns: *ns }.to_string(), id),
            Key::Sensor { ns, st, id } => format!(
                "{}/{}",
                Key::AllSensors { ns: *ns, st: *st }.to_string(),
                id
            ),
            Key::AllTanks { ns: Namespace(n) } => format!("{}/tanks", n),
            Key::AllSensorTypes { ns: Namespace(n) } => format!("{}/sensors", n),
            Key::AllSensors {
                ns,
                st: SensorType(st),
            } => format!("{}/{}", Key::AllSensorTypes { ns: *ns }.to_string(), st),
        }
    }
}

#[cfg(test)]
mod key_test {
    use super::*;
    use uuid::Uuid;

    fn prawnspace() -> Namespace<'static> {
        Namespace("prawnspace")
    }

    #[test]
    fn test_all_tanks() {
        let all_tanks = Key::AllTanks { ns: prawnspace() };
        assert_eq!(all_tanks.to_string(), "prawnspace/tanks");
    }

    #[test]
    fn test_single_tank() {
        let single_tank = Key::Tank {
            ns: prawnspace(),
            id: 1,
        };
        assert_eq!(single_tank.to_string(), "prawnspace/tanks/1");
    }

    #[test]
    fn test_all_sensors() {
        let all_sensors = Key::AllSensors {
            ns: prawnspace(),
            st: SensorType("ph"),
        };
        assert_eq!(all_sensors.to_string(), format!("prawnspace/sensors/ph"));
    }

    #[test]
    fn test_temp_sensor() {
        let temp_id = Uuid::new_v4();
        let temp_sensor = Key::Sensor {
            ns: prawnspace(),
            st: SensorType("temp"),
            id: temp_id,
        };
        assert_eq!(
            temp_sensor.to_string(),
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
        assert_eq!(
            ph_sensor.to_string(),
            format!("prawnspace/sensors/ph/{}", ph_id)
        );
    }

}

/// Represents a change to a value in Redis.
/// Currently only supports the minimum combinations
/// of key/value used by the prawnalith.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RDelta<'a, 'b> {
    AddSetMember {
        #[serde(borrow)]
        key: &'a str,
        #[serde(borrow)]
        val: &'b str,
    },
    UpdateHash {
        key: &'a str,
        fields: Vec<RField<'b>>,
    },
    UpdateString {
        key: &'a str,
        val: &'b str,
    },
}

/// A field which is stored in Redis.
#[derive(Serialize, Deserialize)]
pub struct RField<'a> {
    name: &'a str,
    val: &'a str,
}

#[cfg(test)]
mod rdelta_test {
    use super::*;
    use serde_json;
    use uuid::Uuid;

    fn ns() -> Namespace<'static> {
        Namespace("prawnspace")
    }

    fn id() -> Uuid {
        Uuid::parse_str("123e4567-e89b-12d3-a456-426655440000").unwrap()
    }

    #[test]
    fn rval_ser() {
        let counter = "1";
        assert_eq!(serde_json::to_string(counter).unwrap(), "\"1\"");
    }

    #[test]
    fn add_set_member_ser() {
        let set_friend = &RDelta::AddSetMember {
            key: &Key::AllSensorTypes { ns: ns() }.to_string(),
            val: &id().to_string(),
        };
        assert_eq!(serde_json::to_string(set_friend).unwrap(),
        r#"{"add_set_member":{"key":"prawnspace/sensors","val":"123e4567-e89b-12d3-a456-426655440000"}}"#);
    }

    #[test]
    fn update_hash_ser() {
        let fields: Vec<RField> = vec! [ RField { name: "temp_f", val: "82.31"} ] ;
        let new_potatoes = &RDelta::UpdateHash {
            key: &Key::Sensor { ns: ns(), st: SensorType("temp"), id: id() }.to_string(),
            fields
        };
    }
}
