use api::rocket_contrib::{Json, Value};

use postgresql::statements::DATABASES;
use postgresql::connection::DbConn;

#[get("/")]
pub fn dbs(conn: DbConn) -> Json<Value> {
    let query = format!("SELECT json_agg(s) FROM ({}) s", &*DATABASES);
    let rows = &conn.query(&query, &[]).unwrap();
    let result: Value = rows.get(0).get("json_agg");
    Json(result)
}
