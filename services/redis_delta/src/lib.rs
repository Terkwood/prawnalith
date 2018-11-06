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

/// Represents a change to a value in Redis.
/// Currently only supports the minimum combinations
/// of key/value used by the prawnalith.
/// The `time` field represents epoch secs in UTC
/// for when this record was retrieved.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RDelta<'a> {
    UpdateSet {
        #[serde(borrow)]
        key: &'a str,
        vals: Vec<String>,
        time: u64,
    },
    UpdateHash {
        key: &'a str,
        fields: Vec<RField>,
        time: u64,
    },
    UpdateString {
        key: &'a str,
        val: String,
        time: u64,
    },
}

/// A field which is stored in Redis.
#[derive(Serialize, Deserialize, Debug)]
pub struct RField {
    pub name: String,
    pub val: String,
}

/// Represents a message that lets you know that a specific
/// string, hash, or set has changed.  It does not include
/// the data which has changed, though in the case of hashes,
/// it *does* include a list of fields which were changed.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum REvent {
    SetUpdated { key: String },
    HashUpdated { key: String, fields: Vec<String> },
    StringUpdated { key: String },
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

#[cfg(test)]
mod rdelta_test {
    use super::*;
    use serde_json;
    use uuid::Uuid;

    fn ns() -> Namespace<'static> {
        Namespace("prawnspace")
    }

    fn id_str() -> String {
        "123e4567-e89b-12d3-a456-426655440000".to_string()
    }

    fn id() -> Uuid {
        Uuid::parse_str(&id_str()).unwrap()
    }

    #[test]
    fn rval_ser() {
        let counter = "1";
        assert_eq!(serde_json::to_string(counter).unwrap(), "\"1\"");
    }

    #[test]
    fn update_set_ser() {
        let set_friend = &RDelta::UpdateSet {
            key: &Key::AllSensorTypes { ns: ns() }.to_string(),
            vals: vec![id_str()],
            time: 0,
        };
        assert_eq!(serde_json::to_string(set_friend).unwrap(),
        r#"{"update_set":{"key":"prawnspace/sensors","vals":["123e4567-e89b-12d3-a456-426655440000"],"time":0}}"#);
    }

    #[test]
    fn update_hash_ser() {
        let fields: Vec<RField> = vec![RField {
            name: "temp_f".to_string(),
            val: "82.31".to_string(),
        }];
        let new_potatoes = &RDelta::UpdateHash {
            key: &Key::Sensor {
                ns: ns(),
                st: SensorType("temp"),
                id: id(),
            }
            .to_string(),
            fields,
            time: 0,
        };

        assert_eq!(serde_json::to_string(new_potatoes).unwrap(),
         r#"{"update_hash":{"key":"prawnspace/sensors/temp/123e4567-e89b-12d3-a456-426655440000","fields":[{"name":"temp_f","val":"82.31"}],"time":0}}"#);
    }

    #[test]
    fn update_string_ser() {
        let uk = &Key::AllTanks { ns: ns() }.to_string();
        let uv = "2";
        let update = &RDelta::UpdateString {
            key: uk,
            val: uv.to_string(),
            time: 0,
        };

        let expected = &r#"{"update_string":{"key":"prawnspace/tanks","val":"2","time":0}}"#;
        assert_eq!(serde_json::to_string(update).unwrap(), expected.to_string());

        let deser: RDelta = serde_json::from_str(expected).unwrap();
        match deser {
            RDelta::UpdateString { key, val, time } => {
                assert_eq!(key, *uk);
                assert_eq!(val, uv.to_string());
                assert_eq!(time, 0);
            }
            _ => assert!(false),
        }
    }
}
