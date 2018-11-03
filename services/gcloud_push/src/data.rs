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

#[derive(Copy, Clone)]
pub struct Namespace(&'static str);

#[derive(Copy, Clone)]
pub struct SensorType(&'static str);

/// Yields the key which allows you to access a specific
/// record in redis.
///
/// # Examples
/// ```
/// use gcloud_push::data::Key;
/// let all_tanks = Key::AllTanks { ns: "prawnspace".to_string() };
/// assert_eq!(all_tanks.key(), "prawnspace/tanks")
/// ```
///
/// namespace
/// ```
/// use gcloud_push::data::{Key, Namespace};
/// let nnn = Namespace { ns: "foo".to_string() };
/// let k = Key::Test { ns: nnn };
/// ```
impl Key {
    pub fn key(&self) -> String {
        match self {
            Key::Tank { ns, id } => format!("{}/{}", Key::AllTanks { ns: *ns }.key(), id),
            Key::Sensor { ns, st, id } => {
                format!("{}/{}", Key::AllSensors { ns: *ns, st: *st }.key(), id)
            }
            Key::AllTanks { ns: Namespace(n) } => format!("{}/tanks", n),
            Key::AllSensorTypes { ns: Namespace(n) } => format!("{}/sensors", n),
            Key::AllSensors {
                ns,
                st: SensorType(st),
            } => format!("{}/{}", Key::AllSensorTypes { ns: *ns }.key(), st),
        }
    }
}
