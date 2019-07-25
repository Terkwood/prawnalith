#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

extern crate redis_context;

mod config;
mod external_id;
mod model;
mod predis;
mod web;
mod web_error;

use std::sync::{Arc, Mutex};

use redis_context::RedisContext;

use self::config::Config;

fn main() {
    dotenv::dotenv().expect("Unable to load .env file");

    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(e) => panic!("Unable to parse config ({})", e),
    };

    let redis_ctx = {
        let redis_host = &config.redis_host.unwrap_or("127.0.0.1".to_string());
        let redis_port: u16 = config.redis_port.unwrap_or(6379);
        let redis_auth: Option<String> = config.redis_auth;
        RedisContext::new(
            redis_host.to_string(),
            redis_port,
            redis_auth,
            config.redis_namespace.unwrap_or("".to_string()),
        )
    };

    web::startup(Arc::new(Mutex::new(redis_ctx)));
}
