#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use pond::web;

fn main() {
    rocket::ignite().mount("/", routes![web::index]).launch();
}
