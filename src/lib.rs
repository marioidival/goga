#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;

pub mod postgresql;
pub mod server;
pub mod api;
