#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;

#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

pub mod postgresql;
pub mod server;
pub mod api;
