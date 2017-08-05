#[get("/")]
pub fn sch() -> String {
    format!("Getting list of schemas")
}
