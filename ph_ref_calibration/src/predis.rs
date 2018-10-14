use redis::Commands;
use uuid::Uuid;

use redis_context::RedisContext;

pub struct PhCalibration {
    pub ref_7_0: f32,
    pub ref_4_01: f32,
}

pub fn lookup_ph_calibration(
    id: Uuid,
    redis_ctx: &RedisContext,
) -> Result<PhCalibration, redis::RedisError> {
    let r: Vec<Option<f32>> = redis_ctx.conn.hget(
        format!("{}/sensors/ph/{}", redis_ctx.namespace, id),
        vec!["ref_7_0", "ref_4_01"],
    )?;
    Ok(PhCalibration {
        ref_7_0: r[0].unwrap_or(0.0),
        ref_4_01: r[1].unwrap_or(0.0),
    })
}
