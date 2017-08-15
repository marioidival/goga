use api::rocket_contrib::{Json, Value};
use api::norm_rows;

use postgresql::statements::{SchemaTables, Tables};
use postgresql::connection::DbConn;


#[get("/")]
pub fn tbl(conn: DbConn) -> Json<Value> {
    let rows = &conn.query(&*Tables, &[]).unwrap();
    let res = norm_rows(&rows);
    Json(json!(&res))
}

#[get("/<database>/<schema>")]
pub fn all_tbl(conn: DbConn, database: String, schema: String) -> Json<Value> {
    let rows = &conn.query(&*SchemaTables, &[&database, &schema]).unwrap();
    let res = norm_rows(&rows);
    Json(json!(&res))
}

#[get("/<database>/<schema>/<table>")]
pub fn select_table(conn: DbConn, database: String, schema: String, table: String) -> Json<Value> {
    let select = format!("SELECT * FROM {}.{}.{}", database, schema, table);
    let rows = &conn.query(&select, &[]).unwrap();
    let res = norm_rows(&rows);
    Json(json!(&res))
}
