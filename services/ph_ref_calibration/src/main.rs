#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

mod config;
mod external_id;
mod model;
mod predis;
mod web;
mod web_error;

use rocket_contrib::databases::redis;

use crate::config::Config;

#[database("redis")]
pub struct RedisConn(redis::Connection);

fn main() {
    dotenv::dotenv().expect("Unable to load .env file");

    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(e) => panic!("Unable to parse config ({})", e),
    };

    let redis_namespace = &config.redis_namespace.unwrap_or("".to_string());

    // TODO
    web::startup(redis_namespace);
}
