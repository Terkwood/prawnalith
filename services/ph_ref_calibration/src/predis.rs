use redis::Commands;
use uuid::Uuid;

use super::model::*;

pub struct RedisConfig {
    redis_host: String,
    redis_port: u16,
    redis_auth: Option<String>,
    namespace: String,
}

pub fn lookup_ph_calibration(
    id: Uuid,
    redis_cfg: &RedisConfig,
) -> Result<PhCalibration, redis::RedisError> {
    let r: Vec<Option<f32>> = redis_cfg.conn.hget(
        format!("{}/sensors/ph/{}", redis_cfg.namespace, id),
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
