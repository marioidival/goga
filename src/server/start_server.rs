use server::app::GogaApp;
use server::sapper;


pub fn run() {
    let mut sapp = sapper::SapperApp::new();

    sapp.address("0.0.0.0")
        .port(3210)
        .add_module(Box::new(GogaApp));
    println!("listening on http://0.0.0.0:3210");

    sapp.run_http()
}
