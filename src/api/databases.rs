#[get("/")]
pub fn dbs() -> String {
    format!("Getting list of databases")
}
