use redis::Commands;
use uuid::Uuid;

use crate::model::*;
use crate::RedisConn;

pub fn lookup_ph_calibration(
    redis_conn: &RedisConn,
    namespace: &str,
    id: Uuid,
) -> Result<PhCalibration, redis::RedisError> {
    let r: Vec<Option<f32>> = redis_conn.0.hget(
        format!("{}/sensors/ph/{}", namespace, id),
        vec!["low_ph_ref", "low_mv", "hi_ph_ref", "hi_mv"],
    )?;
    Ok(PhCalibration {
        low: PhRefValue {
            ph_ref: r[0].unwrap_or(0.0),
            mv: r[1].unwrap_or(0.0),
        },
        hi: PhRefValue {
            ph_ref: r[2].unwrap_or(0.0),
            mv: r[3].unwrap_or(0.0),
        },
    })
}

/// This is the "name" field that will be used to form a V5 UUID
pub fn get_external_device_namespace(
    redis_conn: &RedisConn,
    namespace: &str,
    device_type: &str,
) -> Result<Uuid, redis::RedisError> {
    let key = format!("{}/external_device_namespace", namespace);
    let r: Option<String> = redis_conn.0.hget(&key, device_type)?;

    match r {
        None => {
            let it = Uuid::new_v4();
            redis_conn.0.set(key, it.to_string())?;
            Ok(it)
        }
        Some(s) => {
            Ok(Uuid::parse_str(&s[..]).unwrap()) // fine.  just panic then.
        }
    }
}
