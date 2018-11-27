#![feature(proc_macro_hygiene, decl_macro, bind_by_move_pattern_guards)]
extern crate base64;
extern crate frank_jwt;
extern crate hashbrown;
#[macro_use]
extern crate lazy_static;
extern crate redis_delta;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate crypto;
extern crate regex;
extern crate serde_json;

pub mod authentication;
mod authorization;
pub mod claims;
pub mod config;
pub mod key_pairs;
pub mod push;
mod redis_conn;
mod tanks;
pub mod web;
