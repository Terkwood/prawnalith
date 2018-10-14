use redis::Commands;
use uuid::Uuid;

use redis_context::RedisContext;

use super::model::*;

pub fn lookup_ph_calibration(
    id: Uuid,
    redis_ctx: &RedisContext,
) -> Result<PhCalibration, redis::RedisError> {
    let r: Vec<Option<f32>> = redis_ctx.conn.hget(
        format!("{}/sensors/ph/{}", redis_ctx.namespace, id),
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
