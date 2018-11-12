#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;

use pond::config::Config;
use pond::key_pairs;
use pond::web;
use std::process;
use std::thread;

fn main() {
    dotenv::dotenv().expect("Unable to load .env file");

    let config = Config::new();
    let config_buddy = config.clone();

    thread::spawn(move || {
        let ctx = config_buddy.redis_context();
        if ctx.is_err() {
            // if we can't create a connection to redis, all is lost.
            process::exit(1);
        }
        key_pairs::refresh_loop(&ctx.unwrap())
    });

    web::startup(config);
}
