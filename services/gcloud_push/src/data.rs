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
pub struct Namespace(pub &'static str);

#[derive(Copy, Clone)]
pub struct SensorType(pub &'static str);

/// Yields the key which allows you to access a specific
/// record in redis.
///
/// # Examples
/// ```
/// use gcloud_push::data::{Namespace, Key};
/// use uuid::Uuid;
/// 
/// let ns = Namespace("prawnspace");
/// let all_tanks = Key::AllTanks { ns: ns };
/// assert_eq!(all_tanks.key(), "prawnspace/tanks");
/// 
/// let single_tank = Key::Tank { ns, id: 1 };
/// assert_eq!(single_tank.key(), "prawnspace/tanks/1");
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
