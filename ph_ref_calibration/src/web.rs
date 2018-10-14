use std::sync::{Arc, Mutex};

use rocket::State;
use uuid::Uuid;

use redis_context::RedisContext;

use super::external_id;
use super::external_id::ResolveError;

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
) -> Result<String, ResolveError> {
    let lock = redis_ctx.lock().unwrap();
    let namespace = lock.get_external_device_namespace()?;
    Ok(external_id::resolve(&ext_id.ext_id, namespace)?.to_string())
}

fn lookup_ph_calibration(
    id: Uuid,
    redis_ctx: State<Arc<Mutex<RedisContext>>>,
) -> Result<String, redis::RedisError> {
    unimplemented!()
}

pub fn startup(redis_ctx: Arc<Mutex<RedisContext>>) {
    rocket::ignite()
        .manage(redis_ctx)
        .mount("/", routes![resolve_external_id])
        .launch();
}
