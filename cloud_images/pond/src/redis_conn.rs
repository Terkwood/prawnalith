#[database("redis")]
pub struct RedisDbConn(pub redis::Connection);

/// Defines a structure that carries a pooled connection
/// to redis, as well as the "namespace" prefix used
/// by all of our keys.
///
/// Cargo.toml's local path specification is difficult to
/// use when building using rust-musl-builder.  In addition,
/// the type of the "DbConn" approach used by Rocket
/// is a bit different than what's needed by the
/// `redis_context` lib.  So we're redeclaring this here.
///
/// It may be useful to release a separate `redis_context`
/// crate to crates.io in the future!
pub struct RedisPoolContext {
    pub pool: rocket_contrib::databases::r2d2::Pool<
        rocket_contrib::databases::r2d2_redis::RedisConnectionManager,
    >,
    pub namespace: String,
}

pub struct RedisConnContext {
    pub namespace: String,
    pub conn: RedisDbConn,
}
