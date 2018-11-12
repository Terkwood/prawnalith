#![feature(proc_macro_hygiene, decl_macro, bind_by_move_pattern_guards)]
extern crate base64;
extern crate frank_jwt;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate regex;
extern crate serde_json;

pub mod authentication;
mod authorization;
pub mod claims;
pub mod config;
pub mod key_pairs;
mod redis_conn;
pub mod web;
