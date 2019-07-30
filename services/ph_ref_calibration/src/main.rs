#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

extern crate serde_derive;

mod config;
mod external_id;
mod model;
mod predis;
mod routes;
mod web_error;

use rocket_contrib::databases::redis;

use routes::*;

#[database("redis")]
pub struct RedisConn(redis::Connection);

struct Namespace(String);

fn main() {
    rocket::ignite()
        .attach(rocket::fairing::AdHoc::on_attach(
            "Namespace Config",
            |rocket| {
                let namespace = rocket
                    .config()
                    .get_str("namespace")
                    .unwrap_or("shrimpfiesta")
                    .to_string();
                Ok(rocket.manage(Namespace(namespace)))
            },
        ))
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
