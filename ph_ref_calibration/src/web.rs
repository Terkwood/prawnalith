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
}

/// You need to Accept: text/plain in your get request
/// e.g.
/// ```
/// curl http://localhost:8000/id\?ext_id\=AAAA0000 -H "Accept: text/plain"
/// ```
#[get("/id?<ext_id>", format = "text/plain")]
fn resolve_external_id(
    ext_id: ExtId,
    redis_ctx: State<Arc<Mutex<RedisContext>>>,
) -> Result<String, WebError> {
    let lock = redis_ctx.lock().unwrap();
    let namespace = lock.get_external_device_namespace()?;
    Ok(format!(
        "{}\n",
        external_id::resolve(&ext_id.ext_id, namespace)?.to_string()
    ))
}

#[get("/sensors/ph/<uuid>/calibration", format = "text/csv")]
fn lookup_ph_calibration(
    uuid: String,
    redis_ctx: State<Arc<Mutex<RedisContext>>>,
) -> Result<String, WebError> {
    let id = Uuid::parse_str(&uuid)?;

    let calibration = predis::lookup_ph_calibration(id, redis_ctx.lock().unwrap().deref())?;
    Ok(format!(
        "ref_7_0,ref_4_01\n{},{}\n",
        calibration.ref_7_0, calibration.ref_4_01
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
