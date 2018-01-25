#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate rocket;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_json;

pub mod postgresql;
pub mod server;
pub mod api;
