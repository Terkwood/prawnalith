#![feature(decl_macro)]
#![feature(custom_attribute)]
#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate serde_derive;

mod config;
mod web;

use self::config::Config;

fn main() {
    dotenv::dotenv().expect("Unable to load .env file");

    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(e) => panic!("Unable to parse config ({})", e),
    };

    web::startup();
}
