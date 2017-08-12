use api::rocket_contrib::{Json, Value};
use api::norm_rows;

use postgresql::statements::Schemas;
use postgresql::connection::DbConn;


#[get("/")]
pub fn sch(conn: DbConn) -> Json<Value> {
    let rows = &conn.query(&*Schemas, &[]).unwrap();
    let res = norm_rows(&rows);
    Json(json!(&res))
}
