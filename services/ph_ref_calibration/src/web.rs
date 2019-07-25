use std::ops::Deref;
use std::sync::{Arc, Mutex};

use rocket::State;
use uuid::Uuid;

use redis_context::RedisContext;

use super::external_id;
use super::model::*;
use super::predis;
use super::web_error::WebError;

use rocket::request::Form;

/// You need to Accept: text/plain in your get request
/// e.g.
/// ```
/// curl http://localhost:8000/id\?ext_id\=AAAA0000&device_type=temp -H "Accept: text/plain"
/// ```
#[get("/id?<ext_id..>", format = "text/plain")]
fn resolve_external_id(
    ext_id: Form<ExtId>,
    redis_ctx: State<Arc<Mutex<RedisContext>>>,
) -> Result<String, WebError> {
    let lock = redis_ctx.lock().unwrap();
    let namespace = lock.get_external_device_namespace(ext_id.0.device_type)?;
    Ok(format!(
        "{}\n",
        external_id::resolve(&ext_id.0.ext_id, namespace)?.to_string()
    ))
}

/// Try
/// ```
/// curl http://localhost:8000/sensors/ph/calibration\?ext_id\=aaaaffff000000f0\&device_type\=ph -H "Accept: text/csv"
/// ```
#[get("/sensors/ph/calibration?<ext_id>", format = "text/csv")]
fn lookup_ph_calibration_by_ext_id(
    ext_id: ExtId,
    redis_ctx: State<Arc<Mutex<RedisContext>>>,
) -> Result<String, WebError> {
    let lock = redis_ctx.lock().unwrap();
    println!("LOCKED"); // TODO
    let namespace = lock.get_external_device_namespace(ext_id.device_type)?;
    println!("namespace {:?}", namespace); // TODO
    let id = external_id::resolve(&ext_id.ext_id, namespace)?;
    println!("id {:?}", id); // TODO
    let calibration = predis::lookup_ph_calibration(id, lock.deref())?;
    println!("calibration {:?}", calibration); // TODO
    Ok(calibration.as_csv())
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
    Ok(calibration.as_csv())
}

pub fn startup(redis_ctx: Arc<Mutex<RedisContext>>) {
    rocket::ignite()
        .manage(redis_ctx)
        .mount(
            "/",
            routes![
                resolve_external_id,
                lookup_ph_calibration_by_ext_id,
                lookup_ph_calibration
            ],
        )
        .launch();
}
