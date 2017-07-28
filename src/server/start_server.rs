extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hi Goga!"
}

fn rocket() -> rocket::Rocket {

    rocket::ignite()
        .mount("/", routes![hello])
}

pub fn run() {
    rocket().launch();
}
