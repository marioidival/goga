use api::rocket_contrib::{Json, Value};
use api::norm_rows;

use postgresql::statements::Databases;
use postgresql::connection::DbConn;

#[get("/")]
pub fn dbs(conn: DbConn) -> Json<Value> {
    let rows = &conn.query(&*Databases, &[]).unwrap();
    let res = norm_rows(&rows);
    Json(json!(&res))
}
