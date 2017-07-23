use server::app::{GogaShellApp, GogaApp};
use api::databases::DatabasesEndpoint;
use api::schemas::SchemasEndpoint;
use api::tables::TablesEndpoint;
use server::sapper;


pub fn run() {
    let mut sapp = sapper::SapperApp::new();

    sapp.address("0.0.0.0")
        .port(3210)
        .with_shell(Box::new(GogaShellApp))
        .add_module(Box::new(GogaApp))
        .add_module(Box::new(DatabasesEndpoint))
        .add_module(Box::new(SchemasEndpoint))
        .add_module(Box::new(TablesEndpoint));
    println!("listening on http://0.0.0.0:3210");

    sapp.run_http()
}
