use rocket::request::{FromForm, FormItems};

#[derive(Debug)]
pub struct StatementStructResult {
    pub sql: String,
}
pub struct WhereStructResult {
    pub sql: String,
    pub values: Vec<String>,
}

// My Results
type QueryOpResult = Result<&'static str, String>;
type StatementResult = Result<StatementStructResult, String>;
type WhereResult = Result<WhereStructResult, String>;

// TODO: Move there later
#[derive(Debug)]
pub struct Params {
    pub k: String,
    pub v: String,
}
// implement QueryString
#[derive(Debug)]
pub struct QueryString {
    pub query: Vec<Params>,
}

impl<'f> FromForm<'f> for QueryString {
    type Error = ();

    fn from_form(items: &mut FormItems<'f>, _: bool) -> Result<QueryString, ()> {
        let mut qs: QueryString = QueryString { query: Vec::new() };
        for (k, v) in items {
            qs.query.push(Params {
                k: k.to_string(),
                v: v.url_decode().unwrap(),
            });
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

fn collect_params(v: &Vec<Params>, collect_values: bool) -> Result<Vec<String>, String> {
    let mut pid: i32 = 0;
    v.iter()
        .filter(|param| !param.k.starts_with("_"))
        .map(|param| {
            pid = pid + 1;
            let result = match param.v.contains(".") {
                true => {
                    let value_splited = param.v.split(".").collect::<Vec<&str>>();
                    let operator = match query_operator(value_splited[0]) {
                        Ok(op) => op,
                        Err(e) => return Err(e),
                    };
                    (operator, value_splited[1])
                }
                _ => ("=", param.v.as_str()),
            };
            if collect_values {
                Ok(format!("{}", result.1))
            } else {
                Ok(format!("{}{}${}", param.k, result.0, pid))
            }
        })
        .collect()
}

// generic function to process statements: _select, _groupby, _order, _count
fn process_statement<F>(
    qs: &QueryString,
    stm: &str,
    stm_default: &str,
    stm_fmt: &str,
    stm_fn: F,
) -> StatementResult
where
    F: Fn(&str) -> String,
{
    let mut statement_syntax: String = String::from(stm_default);
    let statement_params = qs.query.iter().find(|p| p.k == String::from(stm));
    let columns = match statement_params {
        Some(param) => stm_fn(&param.v),
        _ => String::from(""),
    };
    if columns != "" {
        statement_syntax = format!("{}", String::from(stm_fmt).replace("{}", &columns));
    }
    Ok(StatementStructResult { sql: statement_syntax })
}

// Get QueryString and return chunck WHERE statement with values
pub fn ext_where(qs: &QueryString) -> WhereResult {
    let where_syntax: String;
    let ref iter_params = qs.query;
    let where_values = match collect_params(iter_params, true) {
        Ok(result) => result,
        Err(e) => return Err(e.to_string()),
    };
    let where_fields = match collect_params(iter_params, false) {
        Ok(result) => result,
        Err(e) => return Err(e.to_string()),
    };

    if where_fields.len() > 1 {
        where_syntax = format!("WHERE {}", where_fields.join(" AND "));
    } else {
        where_syntax = format!("WHERE {}", where_fields.join(" "));
    }

    Ok(WhereStructResult {
        sql: where_syntax,
        values: where_values,
    })
}

// Get QueryString and return SELECT statement
pub fn ext_select(qs: &QueryString) -> StatementResult {
    Ok(
        process_statement(qs, "_select", "SELECT * FROM", "SELECT {} FROM", |x| {
            String::from(x)
        }).unwrap(),
    )
}

// Get QueryString and return COUNT clause statement
pub fn ext_count(qs: &QueryString) -> StatementResult {
    // FIXME: Need handle errors
    Ok(
        process_statement(qs, "_count", "", "SELECT COUNT({}) FROM", |x| {
            let vs = x.split(",").collect::<Vec<&str>>();
            let result = if vs.len() > 1 { "" } else { x };
            String::from(result)
        }).unwrap(),
    )
}

// Get QueryString and return chunk ORDER BY statement
pub fn ext_order(qs: &QueryString) -> StatementResult {
    process_statement(qs, "_order", "", "ORDER BY {}", |x| {
        let vspl = &x.split(",").collect::<Vec<&str>>();
        let od: Vec<String> = vspl.iter()
            .map(|param| {
                let mut s = String::new();
                s.push_str(param); // omg
                if s.starts_with("-") {
                    s.remove(0);
                    s.push_str(" DESC")
                }
                format!("{}", s)
            })
            .collect();
        format!("{}", od.join(","))
    })
}

// Get QueryString and return chunk GROUP BY statement
pub fn ext_groupby(qs: &QueryString) -> StatementResult {
    Ok(
        process_statement(qs, "_groupby", "", "GROUP BY {}", |x| String::from(x)).unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use postgresql::commands::*;

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
                Err(e) => assert_eq!("", e),
            }
        }
    }

    #[test]
    fn get_opetators_error() {
        let invalid = "!tn".to_string();
        match query_operator(&invalid) {
            Ok(_) => println!("no result"),
            Err(e) => assert_eq!(format!("operator {} not found", invalid), e),
        }
    }

    #[test]
    fn where_by_request_one_querystring() {
        let mut hm = Vec::new();
        hm.push(Params {
            k: "user_id".to_string(),
            v: "5".to_string(),
        });

        let qs = QueryString { query: hm };
        let result = ext_where(&qs).unwrap();
        assert_eq!(result.sql, "WHERE user_id=$1".to_string());
        assert_eq!(result.values, vec!["5".to_string()])
    }

    #[test]
    fn where_by_request_with_orderby() {
        let mut hm = Vec::new();
        hm.push(Params {
            k: "user_id".to_string(),
            v: "5".to_string(),
        });
        hm.push(Params {
            k: "_order".to_string(),
            v: "user_id".to_string(),
        });

        let qs = QueryString { query: hm };
        let result = ext_where(&qs).unwrap();
        assert_eq!(result.sql, "WHERE user_id=$1".to_string());
        assert_eq!(result.values, vec!["5".to_string()])
    }

    #[test]
    fn where_by_request_n_querystring() {
        let mut hm = Vec::new();
        hm.push(Params {
            k: "user_id".to_string(),
            v: "5".to_string(),
        });
        hm.push(Params {
            k: "name".to_string(),
            v: "goga".to_string(),
        });

        let qs = QueryString { query: hm };
        let result = ext_where(&qs).unwrap();
        assert_eq!(result.sql, "WHERE user_id=$1 AND name=$2".to_string());
        assert_eq!(result.values, vec!["5".to_string(), "goga".to_string()])
    }

    #[test]
    fn where_by_request_with_query_operators() {
        let mut hm = Vec::new();
        hm.push(Params {
            k: "user_id".to_string(),
            v: "$ne.5".to_string(),
        });
        hm.push(Params {
            k: "name".to_string(),
            v: "goga".to_string(),
        });

        let qs = QueryString { query: hm };
        let result = ext_where(&qs).unwrap();
        assert_eq!(result.sql, "WHERE user_id!=$1 AND name=$2".to_string());
        assert_eq!(result.values, vec!["5".to_string(), "goga".to_string()])
    }

    #[test]
    fn where_by_request_with_query_invalid_operators() {
        let mut hm = Vec::new();
        hm.push(Params {
            k: "user_id".to_string(),
            v: "$nt.5".to_string(),
        });
        hm.push(Params {
            k: "name".to_string(),
            v: "goga".to_string(),
        });

        let qs = QueryString { query: hm };
        match ext_where(&qs) {
            Err(e) => assert_eq!(e, "operator $nt not found"),
            Ok(_) => println!("no result"),
        }
    }

    #[test]
    fn select_by_request_one_column() {
        let mut hm = Vec::new();
        hm.push(Params {
            k: "_select".to_string(),
            v: "name".to_string(),
        });

        let qs = QueryString { query: hm };
        let result = ext_select(&qs).unwrap();
        assert_eq!(result.sql, "SELECT name FROM");
    }

    #[test]
    fn select_by_request_n_column() {
        let mut hm = Vec::new();
        hm.push(Params {
            k: "_select".to_string(),
            v: "name,age".to_string(),
        });

        let qs = QueryString { query: hm };
        let result = ext_select(&qs).unwrap();
        assert_eq!(result.sql, "SELECT name,age FROM");
    }

    #[test]
    fn select_by_request_n_empty_value() {
        let mut hm = Vec::new();
        hm.push(Params {
            k: "_select".to_string(),
            v: "".to_string(),
        });

        let qs = QueryString { query: hm };
        let result = ext_select(&qs).unwrap();
        assert_eq!(result.sql, "SELECT * FROM");
    }

    #[test]
    fn select_by_request_without_select() {
        let mut hm = Vec::new();
        hm.push(Params {
            k: "user".to_string(),
            v: "blah".to_string(),
        });

        let qs = QueryString { query: hm };
        let result = ext_select(&qs).unwrap();
        assert_eq!(result.sql, "SELECT * FROM");
    }

    #[test]
    fn count_by_request() {
        let mut hm = Vec::new();
        hm.push(Params {
            k: "_count".to_string(),
            v: "name".to_string(),
        });

        let qs = QueryString { query: hm };
        let result = ext_count(&qs).unwrap();
        assert_eq!(result.sql, "SELECT COUNT(name) FROM");
    }

    #[test]
    fn count_by_request_all_fields() {
        let mut hm = Vec::new();
        hm.push(Params {
            k: "_count".to_string(),
            v: "*".to_string(),
        });

        let qs = QueryString { query: hm };
        let result = ext_count(&qs).unwrap();
        assert_eq!(result.sql, "SELECT COUNT(*) FROM");
    }

    #[test]
    fn count_by_request_two_fields() {
        let mut hm = Vec::new();
        hm.push(Params {
            k: "_count".to_string(),
            v: "name,age".to_string(),
        });

        let qs = QueryString { query: hm };
        match ext_count(&qs) {
            Err(e) => assert_eq!(e, "could not use more than one column in count function"),
            Ok(s) => assert_eq!(s.sql, ""),
        }
    }

    #[test]
    fn count_by_request_empty_value() {
        let mut hm = Vec::new();
        hm.push(Params {
            k: "_count".to_string(),
            v: "".to_string(),
        });

        let qs = QueryString { query: hm };
        let result = ext_count(&qs).unwrap();
        assert_eq!(result.sql, "")
    }

    #[test]
    fn order_by_request() {
        let mut hm = Vec::new();
        hm.push(Params {
            k: "_order".to_string(),
            v: "name".to_string(),
        });

        let qs = QueryString { query: hm };
        let result = ext_order(&qs).unwrap();
        assert_eq!(result.sql, "ORDER BY name")
    }

    #[test]
    fn order_by_request_desc() {
        let mut hm = Vec::new();
        hm.push(Params {
            k: "_order".to_string(),
            v: "-name".to_string(),
        });

        let qs = QueryString { query: hm };
        let result = ext_order(&qs).unwrap();
        assert_eq!(result.sql, "ORDER BY name DESC")
    }

    #[test]
    fn order_by_request_empty_params() {
        let mut hm = Vec::new();
        hm.push(Params {
            k: "_order".to_string(),
            v: "".to_string(),
        });

        let qs = QueryString { query: hm };
        let result = ext_order(&qs).unwrap();
        assert_eq!(result.sql, "")
    }

    #[test]
    fn order_by_request_without_params() {
        let mut hm = Vec::new();
        hm.push(Params {
            k: "name".to_string(),
            v: "toby".to_string(),
        });

        let qs = QueryString { query: hm };
        match ext_order(&qs) {
            Ok(result) => assert_eq!(result.sql, ""),
            Err(e) => assert_eq!("error on params", e),
        };
    }

    #[test]
    fn order_by_request_multiples_columns() {
        let mut hm = Vec::new();
        hm.push(Params {
            k: "_order".to_string(),
            v: "name,age".to_string(),
        });

        let qs = QueryString { query: hm };
        let result = ext_order(&qs).unwrap();
        assert_eq!(result.sql, "ORDER BY name,age")
    }

    #[test]
    fn order_by_request_multiples_columns_with_orders() {
        let mut hm = Vec::new();
        hm.push(Params {
            k: "_order".to_string(),
            v: "name,-age".to_string(),
        });

        let qs = QueryString { query: hm };
        let result = ext_order(&qs).unwrap();
        assert_eq!(result.sql, "ORDER BY name,age DESC")
    }

    #[test]
    fn groupby_by_request() {
        let mut hm = Vec::new();
        hm.push(Params {
            k: "_groupby".to_string(),
            v: "name".to_string(),
        });

        let qs = QueryString { query: hm };
        let result = ext_groupby(&qs).unwrap();
        assert_eq!(result.sql, "GROUP BY name")
    }

    #[test]
    fn groupby_by_request_empty_param() {
        let mut hm = Vec::new();
        hm.push(Params {
            k: String::from("_groupby"),
            v: String::from(""),
        });

        let qs = QueryString { query: hm };
        let result = ext_groupby(&qs).unwrap();
        assert_eq!(result.sql, "")
    }
}
