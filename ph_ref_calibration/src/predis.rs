use uuid::Uuid;

use redis_context::RedisContext;

pub struct PhCalibration {
    pub ref_7_0: f32,
    pub ref_4_01: f32,
}

fn lookup_ph_calibration(
    id: Uuid,
    redis_ctx: RedisContext,
) -> Result<PhCalibration, redis::RedisError> {
    unimplemented!()
}
