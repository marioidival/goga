use api::rocket_contrib::{Json, Value};

use postgresql::statements::{SCHEMA_TABLES, TABLES};
use postgresql::connection::DbConn;
use postgresql::commands::*;


#[get("/")]
pub fn tbl(conn: DbConn) -> Json<Value> {
    let query = format!("SELECT json_agg(s) FROM ({}) s", &*TABLES);
    let rows = &conn.query(&query, &[]).unwrap();
    let result: Value = rows.get(0).get("json_agg");
    Json(result)
}

#[get("/<database>/<schema>")]
pub fn all_tbl(conn: DbConn, database: String, schema: String) -> Json<Value> {
    let query = format!("SELECT json_agg(s) FROM ({}) s", &*SCHEMA_TABLES);
    let rows = &conn.query(&query, &[&database, &schema]).unwrap();
    let result: Value = rows.get(0).get("json_agg");
    Json(result)
}

#[get("/<database>/<schema>/<table>")]
pub fn select_table(conn: DbConn, database: String, schema: String, table: String) -> Json<Value> {
    let select = format!("SELECT * FROM {}.{}.{}", database, schema, table);
    let query = format!("SELECT json_agg(s) FROM ({}) s", select);
    let rows = &conn.query(&query, &[]).unwrap();
    let result: Value = rows.get(0).get("json_agg");
    Json(result)
}

#[get("/<database>/<schema>/<table>?<qs>")]
pub fn select_table_qs(
    conn: DbConn,
    database: String,
    schema: String,
    table: String,
    qs: QueryString,
) -> Json<Value> {
    let where_result = ext_where(&qs).unwrap();
    let orderby_result = ext_order(&qs).unwrap();

    let select = format!(
        "SELECT * FROM {}.{}.{} {} {}",
        database,
        schema,
        table,
        where_result.sql,
        orderby_result.sql
    );

    let query = format!("SELECT json_agg(s) FROM ({}) s", select);
    let rows = &conn.query(&query, &[&where_result.values[0]]).unwrap();
    let result: Value = rows.get(0).get("json_agg");
    Json(result)
}
