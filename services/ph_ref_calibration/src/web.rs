use std::ops::Deref;
use std::sync::{Arc, Mutex};

use rocket::State;
use uuid::Uuid;

use crate::RedisConn;

use crate::external_id;
use crate::model::*;
use crate::predis;
use crate::web_error::WebError;

use rocket::request::Form;

/// You need to Accept: text/plain in your get request
/// e.g.
/// ```
/// curl http://localhost:8000/id\?ext_id\=AAAA0000&device_type=temp -H "Accept: text/plain"
/// ```
#[get("/id?<ext_id..>", format = "text/plain")]
fn resolve_external_id(redis_conn: RedisConn, ext_id: Form<ExtId>) -> Result<String, WebError> {
    let namespace = predis::get_external_device_namespace(
        redis_conn,
        unimplemented!("namespace"),
        &ext_id.0.device_type,
    )?;
    Ok(format!(
        "{}\n",
        external_id::resolve(&ext_id.0.ext_id, namespace)?.to_string()
    ))
}

/// Try
/// ```
/// curl http://localhost:8000/sensors/ph/calibration\?ext_id\=aaaaffff000000f0 -H "Accept: text/csv"
/// ```
#[get("/sensors/ph/calibration?<ext_id>", format = "text/csv")]
fn lookup_ph_calibration_by_ext_id(
    redis_conn: RedisConn,
    ext_id: String,
) -> Result<String, WebError> {
    let namespace =
        predis::get_external_device_namespace(redis_conn, unimplemented!("namespace"), "ph")?;

    println!("namespace {:?}", namespace); // TODO
    let id = external_id::resolve(&ext_id, namespace)?;

    println!("id {:?}", id); // TODO
    let calibration = predis::lookup_ph_calibration(redis_conn, unimplemented!("namespace"), id)?;

    println!("calibration {:?}", calibration); // TODO
    Ok(calibration.as_csv())
}

/// Try
/// ```
/// curl http://localhost:8000/sensors/ph/ffffffff-ffff-aaaa-eeee-bbbbddddaaaa/calibration -H "Accept: text/csv"
/// ```
#[get("/sensors/ph/<uuid>/calibration", format = "text/csv")]
fn lookup_ph_calibration(redis_conn: RedisConn, uuid: String) -> Result<String, WebError> {
    let id = Uuid::parse_str(&uuid)?;

    let calibration = predis::lookup_ph_calibration(redis_conn, unimplemented!("namespace"), id)?;
    Ok(calibration.as_csv())
}

pub fn startup(redis_namespace: &str) {
    rocket::ignite()
        .attach(RedisConn::fairing())
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
