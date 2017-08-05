#[get("/")]
pub fn tbl() -> String {
    format!("Getting list of tables")
}
