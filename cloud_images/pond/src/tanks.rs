use crate::redis_conn::RedisDbConn;
use hashbrown::HashMap;
use redis_delta::{Key, Namespace};
use rocket_contrib::databases::redis::Commands;

/// A struct to hold data returned by the HTTP request
/// for tanks' temp & ph info.
#[derive(Debug, Serialize, Deserialize)]
pub struct Tank {
    pub id: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_f: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_c: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_update_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_update_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ph: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ph_mv: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ph_update_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ph_update_count: Option<u32>,
}

impl Tank {
    pub fn from_fields(id: u16, fields: &HashMap<&str, String>) -> Tank {
        let (temp_f, temp_c) = match (
            parse_maybe::<f32>(fields.get("temp_f")),
            parse_maybe::<f32>(fields.get("temp_c")),
        ) {
            (Some(f), Some(c)) => (Some(f), Some(c)),
            (Some(f), _) => (Some(f), Some(f_to_c(f))),
            (_, Some(c)) => (Some(c_to_f(c)), Some(c)),
            (_, _) => (None, None),
        };

        Tank {
            id,
            name: parse_maybe::<String>(fields.get("name")),
            temp_f,
            temp_c,
            temp_update_time: parse_maybe::<u64>(fields.get("temp_update_time")),
            temp_update_count: parse_maybe::<u32>(fields.get("temp_update_count")),
            ph: parse_maybe::<f32>(fields.get("ph")),
            ph_mv: parse_maybe::<f32>(fields.get("ph_mv")),
            ph_update_time: parse_maybe::<u64>(fields.get("ph_update_time")),
            ph_update_count: parse_maybe::<u32>(fields.get("ph_update_count")),
        }
    }
}

/// Fetch the status of all tanks from Redis.
pub fn fetch_all(conn: RedisDbConn, namespace: &str) -> Result<Vec<Tank>, redis::RedisError> {
    // figure out how many tanks you need to query
    let num_tanks = fetch_num_tanks(&conn, namespace)?;

    // query each tank for its status
    let mut result: Vec<Tank> = vec![];
    for id in 1..=num_tanks {
        if let Some(t_status) = fetch_tank_status(id, &conn, namespace)? {
            result.push(t_status);
        }
    }

    Ok(result)
}

fn fetch_num_tanks(conn: &RedisDbConn, namespace: &str) -> Result<u16, redis::RedisError> {
    let key = Key::AllTanks {
        ns: Namespace(namespace),
    }
    .to_string();
    conn.0.get(key)
}
const TANK_FIELDS: &[&'static str] = &[
    "name",
    "temp_f",
    "temp_c",
    "temp_update_time",
    "temp_update_count",
    "ph",
    "ph_mv",
    "ph_update_time",
    "ph_update_count",
];

/// Fetch the status of an individual tank from Redis
fn fetch_tank_status(
    id: u16,
    conn: &RedisDbConn,
    namespace: &str,
) -> Result<Option<Tank>, redis::RedisError> {
    let key = Key::Tank {
        ns: Namespace(namespace),
        id,
    }
    .to_string();

    let data: Vec<Option<String>> = conn.0.hget(&key, TANK_FIELDS)?;

    let no_results: bool = data.iter().all(|maybe| maybe.is_none());

    Ok(if no_results {
        None
    } else {
        Some({
            let mut with_field_names: HashMap<&str, String> = hashbrown::HashMap::new();

            let i: Vec<(&str, Option<String>)> = TANK_FIELDS.iter().map(|s| *s).zip(data).collect();
            for (field, maybe_val) in i {
                if let Some(val) = maybe_val {
                    with_field_names.insert(field, val);
                }
            }

            Tank::from_fields(id, &with_field_names)
        })
    })
}

fn f_to_c(f: f32) -> f32 {
    (f - 32f32) * 5f32 / 9f32
}
fn c_to_f(c: f32) -> f32 {
    (c * 9f32 / 5f32) + 32f32
}

fn parse_maybe<T>(maybe: Option<&String>) -> Option<T>
where
    T: std::str::FromStr,
{
    maybe.and_then(|s| s.parse::<T>().ok())
}
