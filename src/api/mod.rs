extern crate rocket;
extern crate serde;
extern crate rocket_contrib;


pub mod databases;
pub mod schemas;
pub mod tables;

use std::collections::HashMap;

use postgres::rows::Rows;
use postgres::types::{VARCHAR, INT4, INT8, TEXT};

fn norm_rows(rows: &Rows) -> Vec<HashMap<&str, String>> {
    let mut res: Vec<HashMap<&str, String>> = Vec::new();

    let columns = rows.columns();
    for row in rows.iter() {
        let mut r = HashMap::new();
        for (x, column) in columns.iter().enumerate() {
            let value = match column.type_() {
                &INT4 => row.get::<_, i32>(x).to_string(),
                &INT8 => row.get::<_, i64>(x).to_string(),
                &TEXT => row.get::<_, String>(x),
                &VARCHAR => row.get::<_, String>(x),
                // System Identifier
                ref mut name => row.get::<_, String>(x),
                _ => String::from("")
            };
            r.insert(column.name(), value);
            res.push(r.clone());
        }
    }
    res
}
