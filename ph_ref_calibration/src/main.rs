#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod config;
mod routes;

fn main() {
    dotenv::dotenv().expect("Unable to load .env file");

    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(e) => panic!("Unable to parse config ({})", e),
    };

    rocket::ignite().mount("/", routes![routes::resolve_external_id]).launch();
}
