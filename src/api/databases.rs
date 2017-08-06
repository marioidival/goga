use postgresql::connection::DbConn;

#[get("/")]
pub fn dbs(conn: DbConn) -> String {
    let rows = &conn.query("SELECT id, name FROM test", &[]).unwrap();
    for s in rows {
        println!("Found person {:?}", s);
    }
    format!("Getting list of databases")
}
