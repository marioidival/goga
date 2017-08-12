use api::rocket_contrib::{Json, Value};
use api::norm_rows;

use postgresql::statements::Tables;
use postgresql::connection::DbConn;


#[get("/")]
pub fn tbl(conn: DbConn) -> Json<Value> {
    let rows = &conn.query(&*Tables, &[]).unwrap();
    let res = norm_rows(&rows);
    Json(json!(&res))
}
