#![feature(proc_macro_hygiene, decl_macro)]
extern crate jsonwebtoken;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod auth;
pub mod web;
