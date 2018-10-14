use std::ops::Deref;
use std::sync::{Arc, Mutex};

use rocket::State;
use uuid::Uuid;

use redis_context::RedisContext;

use super::external_id;
use super::predis;

#[derive(FromForm)]
struct ExtId {
    ext_id: String,
    device_type: String,
}

/// You need to Accept: text/plain in your get request
/// e.g.
/// ```
/// curl http://localhost:8000/id\?ext_id\=AAAA0000&device_type=temp -H "Accept: text/plain"
/// ```
#[get("/id?<ext_id>", format = "text/plain")]
fn resolve_external_id(
    ext_id: ExtId,
    redis_ctx: State<Arc<Mutex<RedisContext>>>,
) -> Result<String, WebError> {
    let lock = redis_ctx.lock().unwrap();
    let namespace = lock.get_external_device_namespace(ext_id.device_type)?;
    Ok(format!(
        "{}\n",
        external_id::resolve(&ext_id.ext_id, namespace)?.to_string()
    ))
}

/// Try
/// ```
/// curl http://localhost:8000/sensors/ph/ffffffff-ffff-aaaa-eeee-bbbbddddaaaa/calibration -H "Accept: text/csv"
/// ```
#[get("/sensors/ph/<uuid>/calibration", format = "text/csv")]
fn lookup_ph_calibration(
    uuid: String,
    redis_ctx: State<Arc<Mutex<RedisContext>>>,
) -> Result<String, WebError> {
    let id = Uuid::parse_str(&uuid)?;

    let calibration = predis::lookup_ph_calibration(id, redis_ctx.lock().unwrap().deref())?;
    Ok(format!(
        "low_ph_ref,low_mv,hi_ph_ref,hi_mv\n{:.*},{:.*},{:.*},{:.*}\n",
        2,
        calibration.low.ph_ref,
        2,
        calibration.low.mv,
        2,
        calibration.hi.ph_ref,
        2,
        calibration.hi.mv,
    ))
}

pub fn startup(redis_ctx: Arc<Mutex<RedisContext>>) {
    rocket::ignite()
        .manage(redis_ctx)
        .mount("/", routes![resolve_external_id, lookup_ph_calibration])
        .launch();
}

/// All possible errors that this web app can throw
#[derive(Debug)]
pub enum WebError {
    RedisErr(redis::RedisError),
    ParseErr(uuid::parser::ParseError),
}

impl From<redis::RedisError> for WebError {
    fn from(error: redis::RedisError) -> Self {
        WebError::RedisErr(error)
    }
}

impl From<uuid::parser::ParseError> for WebError {
    fn from(error: uuid::parser::ParseError) -> Self {
        WebError::ParseErr(error)
    }
}
