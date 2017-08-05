extern crate rocket;

use rocket::request::{FromForm, FormItems};


pub struct StatementStructResult {sql: String}
pub struct WhereStructResult {
    sql: String, values: Vec<String>,
}

// My Results
type QueryOpResult = Result<&'static str, String>;
type StatementResult = Result<StatementStructResult, String>;
type WhereResult = Result<WhereStructResult, String>;

// TODO: Move there later
pub struct Params {
    pub k: String, pub v: String,
}
// implement QueryString
pub struct QueryString {
    pub query: Vec<Params>
}

impl<'f> FromForm<'f> for QueryString {
    type Error = ();

    fn from_form(items: &mut FormItems<'f>, strict: bool) -> Result<QueryString, ()> {
        let mut qs: QueryString = QueryString{ query: Vec::new() };
        for (k, v) in items {
            qs.query.push(Params{k: k.to_string(), v: v.url_decode().unwrap()});
        }
        Ok(qs)
    }
}

pub fn query_operator(qop: &str) -> QueryOpResult {
    match &qop[..] {
        "$eq" => Ok("="),
        "$ne" => Ok("!="),
        "$gt" => Ok(">"),
        "$lt" => Ok("<"),
        "$gte" => Ok(">="),
        "$lte" => Ok("<="),
        "$in" => Ok("IN"),
        "$nin" => Ok("NOT IN"),
        "$notnull" => Ok("IS NOT NULL"),
        "$null" => Ok("IS NULL"),
         _ => Err(format!("operator {} not found", qop)),
    }
}

fn collect_params(v: &Vec<Params>, collect_values: bool) -> Result<Vec<String>,String> {
    let mut pid: i32 = 0;
    v.iter()
        .map(|param| {
            pid = pid + 1;
            let result = match param.v.contains(".") {
                true => {
                    let value_splited = param.v.split(".").collect::<Vec<&str>>();
                    let operator = match query_operator(value_splited[0]) {
                        Ok(op) => op,
                        Err(e) => return Err(e)
                    };
                    (operator, value_splited[1])
                },
                _ => ("=", param.v.as_str())
            };
            if collect_values {
                Ok(format!("{}", result.1))
            } else {
                Ok(format!("{}{}${}", param.k, result.0, pid))
            }
        })
        .collect()
}

// Get QueryString and return SELECT statement
pub fn ext_select(qs: QueryString) -> StatementResult {
    let mut select_syntax: String = "SELECT * FROM".to_string();
    let select_param = qs.query.iter().find(|param| param.k == "_select".to_string());
    let columns = match select_param {
        Some(param) => &param.v,
        _ => return Err("".to_string())
    };

    // empty string means that isn't `_select` command
    if columns != "" {
        select_syntax = format!("SELECT {} FROM", columns);
    }

    Ok(StatementStructResult{ sql: select_syntax })
}

// Get QueryString and return COUNT clause statement
pub fn ext_count(qs: QueryString) -> StatementResult {
    let mut count_syntax: String = String::new();
    let count_param = qs.query.iter().find(|param| param.k == "_count".to_string());
    let columns = match count_param {
        Some(param)=> {
            let vsplited = &param.v.split(",").collect::<Vec<&str>>();
            // FIXME, How to improve it?
            let u :usize = 2;
            if vsplited.len() >= u {
                return Err("could not use more than one column in count function".to_string())
            }
            &param.v
        },
        _ => return Err("".to_string())
    };

    if columns != "" {
        count_syntax = format!("SELECT COUNT({}) FROM", columns);
    }
    Ok(StatementStructResult{ sql: count_syntax })
}

// Get QueryString and return chunck WHERE statement with values
pub fn ext_where(qs: QueryString) -> WhereResult {
    let where_syntax: String;
    let ref iter_params = qs.query;
    let where_values = match collect_params(iter_params, true) {
        Ok(result) => result,
        Err(e) => return Err(e.to_string())
    };
    let where_fields = match collect_params(iter_params, false) {
        Ok(result) => result,
        Err(e) => return Err(e.to_string())
    };

    if where_fields.len() > 1 {
        where_syntax = format!("WHERE {}", where_fields.join(" AND "));
    } else {
        where_syntax = format!("WHERE {}", where_fields.join(" "));
    }

    Ok(WhereStructResult {
        sql: where_syntax,
        values: where_values
    })
}

// Get QueryString and return chunk ORDER BY statement
pub fn ext_order(qs: QueryString) -> StatementResult {
    let mut order_syntax: String = String::new();
    let order_params = qs.query.iter().find(|param| param.k == "_order".to_string());
    let columns = match order_params {
        Some(param) => {
            // need some refactor after!
            let vspl = &param.v.split(",").map(|s| s.to_string()).collect::<Vec<String>>();
            let od: Vec<String> = vspl.iter().map(|param| {
                let mut s = String::new();
                s.push_str(param); // omg
                if s.starts_with("-") {
                    s.remove(0);
                    s.push_str(" DESC")
                }
                format!("{}", s)
            }).collect();
            format!("{}", od.join(","))
        },
        _ => return Err("".to_string())
    };

    if columns != "" {
        order_syntax = format!("ORDER BY {}", columns);
    }
    Ok(StatementStructResult{ sql: order_syntax })
}

// Get QueryString and return chunk GROUP BY statement
pub fn ext_groupby(qs: QueryString) -> StatementResult {
    let mut groupby_syntax: String = String::new();
    let groupby_params = qs.query.iter().find(|param| param.k == String::from("_groupby"));
    let columns = match groupby_params {
        Some(param) => &param.v,
        _ => return Err(String::from(""))
    };

    if columns != "" {
        groupby_syntax = format!("GROUP BY {}", columns);
    }
    Ok(StatementStructResult{ sql: groupby_syntax })
}

#[cfg(test)]
mod tests {
    use postgres::commands::*;

    #[test]
    fn get_opetators() {
        use std::collections::HashMap;

        let mut test_cases = HashMap::new();
        test_cases.insert("$eq", "=");
        test_cases.insert("$ne", "!=");
        test_cases.insert("$gt", ">");
        test_cases.insert("$gte", ">=");
        test_cases.insert("$lt", "<");
        test_cases.insert("$lte", "<=");
        test_cases.insert("$in", "IN");
        test_cases.insert("$nin", "NOT IN");
        test_cases.insert("$notnull", "IS NOT NULL");
        test_cases.insert("$null", "IS NULL");

        for (op, expected) in &test_cases {
            match query_operator(&op.to_string()) {
                Ok(x) => assert_eq!(expected.to_string(), x),
                Err(e) => assert_eq!("", e)
            }
        }
    }

    #[test]
    fn get_opetators_error() {
        let invalid = "!tn".to_string();
        match query_operator(&invalid) {
            Ok(_) => println!("no result"),
            Err(e) => assert_eq!(format!("operator {} not found", invalid), e)
        }
    }

    #[test]
    fn where_by_request_one_querystring() {
        let mut hm = Vec::new();
        hm.push(Params{k: "user_id".to_string(), v: "5".to_string()});

        let qs = QueryString{ query: hm };
        let result = ext_where(qs)
            .unwrap();
        assert_eq!(result.sql, "WHERE user_id=$1".to_string());
        assert_eq!(result.values, vec!["5".to_string()])
    }

    #[test]
    fn where_by_request_n_querystring() {
        let mut hm = Vec::new();
        hm.push(Params{k: "user_id".to_string(), v: "5".to_string()});
        hm.push(Params{k: "name".to_string(), v: "goga".to_string()});

        let qs = QueryString{ query: hm };
        let result = ext_where(qs)
            .unwrap();
        assert_eq!(result.sql, "WHERE user_id=$1 AND name=$2".to_string());
        assert_eq!(result.values, vec!["5".to_string(), "goga".to_string()])
    }

    #[test]
    fn where_by_request_with_query_operators(){
        let mut hm = Vec::new();
        hm.push(Params{k: "user_id".to_string(), v: "$ne.5".to_string()});
        hm.push(Params{k: "name".to_string(), v: "goga".to_string()});

        let qs = QueryString{ query: hm };
        let result = ext_where(qs)
            .unwrap();
        assert_eq!(result.sql, "WHERE user_id!=$1 AND name=$2".to_string());
        assert_eq!(result.values, vec!["5".to_string(), "goga".to_string()])
    }

    #[test]
    fn where_by_request_with_query_invalid_operators(){
        let mut hm = Vec::new();
        hm.push(Params{k: "user_id".to_string(), v: "$nt.5".to_string()});
        hm.push(Params{k: "name".to_string(), v: "goga".to_string()});

        let qs = QueryString{ query: hm };
        match ext_where(qs) {
            Err(e) => assert_eq!(e, "operator $nt not found"),
            Ok(_) => println!("no result")
        }
    }

    #[test]
    fn select_by_request_one_column() {
        let mut hm = Vec::new();
        hm.push(Params{k: "_select".to_string(), v: "name".to_string()});

        let qs = QueryString{ query: hm };
        let result = ext_select(qs)
            .unwrap();
        assert_eq!(result.sql, "SELECT name FROM");
    }

    #[test]
    fn select_by_request_n_column() {
        let mut hm = Vec::new();
        hm.push(Params{k: "_select".to_string(), v: "name,age".to_string()});

        let qs = QueryString{ query: hm };
        let result = ext_select(qs)
            .unwrap();
        assert_eq!(result.sql, "SELECT name,age FROM");
    }

    #[test]
    fn select_by_request_n_empty_value() {
        let mut hm = Vec::new();
        hm.push(Params{k: "_select".to_string(), v: "".to_string()});

        let qs = QueryString{ query: hm };
        let result = ext_select(qs)
            .unwrap();
        assert_eq!(result.sql, "SELECT * FROM");
    }

    #[test]
    fn count_by_request() {
        let mut hm = Vec::new();
        hm.push(Params{k: "_count".to_string(), v: "name".to_string()});

        let qs = QueryString{ query: hm };
        let result = ext_count(qs)
            .unwrap();
        assert_eq!(result.sql, "SELECT COUNT(name) FROM");
    }

    #[test]
    fn count_by_request_all_fields() {
        let mut hm = Vec::new();
        hm.push(Params{k: "_count".to_string(), v: "*".to_string()});

        let qs = QueryString{ query: hm };
        let result = ext_count(qs)
            .unwrap();
        assert_eq!(result.sql, "SELECT COUNT(*) FROM");
    }

    #[test]
    fn count_by_request_two_fields() {
        let mut hm = Vec::new();
        hm.push(Params{k: "_count".to_string(), v: "name,age".to_string()});

        let qs = QueryString{ query: hm };
        match ext_count(qs) {
            Err(e) => assert_eq!(e, "could not use more than one column in count function"),
            Ok(_) => println!("no result")
        }
    }

    #[test]
    fn count_by_request_empty_value() {
        let mut hm = Vec::new();
        hm.push(Params{k: "_count".to_string(), v: "".to_string()});

        let qs = QueryString{ query: hm };
        let result = ext_count(qs)
            .unwrap();
        assert_eq!(result.sql, "")
    }

    #[test]
    fn order_by_request() {
        let mut hm = Vec::new();
        hm.push(Params{k: "_order".to_string(), v: "name".to_string()});

        let qs = QueryString{ query: hm };
        let result = ext_order(qs)
            .unwrap();
        assert_eq!(result.sql, "ORDER BY name")
    }

    #[test]
    fn order_by_request_desc() {
        let mut hm = Vec::new();
        hm.push(Params{k: "_order".to_string(), v: "-name".to_string()});

        let qs = QueryString{ query: hm };
        let result = ext_order(qs)
            .unwrap();
        assert_eq!(result.sql, "ORDER BY name DESC")
    }

    #[test]
    fn order_by_request_empty_params() {
        let mut hm = Vec::new();
        hm.push(Params{k: "_order".to_string(), v: "".to_string()});

        let qs = QueryString{ query: hm };
        let result = ext_order(qs)
            .unwrap();
        assert_eq!(result.sql, "")
    }

    #[test]
    fn order_by_request_multiples_columns() {
        let mut hm = Vec::new();
        hm.push(Params{k: "_order".to_string(), v: "name,age".to_string()});

        let qs = QueryString{ query: hm };
        let result = ext_order(qs)
            .unwrap();
        assert_eq!(result.sql, "ORDER BY name,age")
    }

    #[test]
    fn order_by_request_multiples_columns_with_orders() {
        let mut hm = Vec::new();
        hm.push(Params{k: "_order".to_string(), v: "name,-age".to_string()});

        let qs = QueryString{ query: hm };
        let result = ext_order(qs)
            .unwrap();
        assert_eq!(result.sql, "ORDER BY name,age DESC")
    }

    #[test]
    fn groupby_by_request() {
        let mut hm = Vec::new();
        hm.push(Params{k: "_groupby".to_string(), v: "name".to_string()});

        let qs = QueryString{ query: hm };
        let result = ext_groupby(qs)
            .unwrap();
        assert_eq!(result.sql, "GROUP BY name")
    }

    #[test]
    fn groupby_by_request_empty_param() {
        let mut hm = Vec::new();
        hm.push(Params{k: String::from("_groupby"), v: String::from("")});

        let qs = QueryString{ query: hm };
        let result = ext_groupby(qs)
            .unwrap();
        assert_eq!(result.sql, "")
    }
}
