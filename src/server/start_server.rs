extern crate rocket;

use api::{databases,schemas,tables};

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/databases", routes![databases::dbs])
        .mount("/schemas", routes![schemas::sch])
        .mount("/tables", routes![tables::tbl])
}

pub fn run() {
    rocket().launch();
}
