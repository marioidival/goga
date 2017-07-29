extern crate rocket;

use rocket::request::{FromForm, FormItems};


pub struct WhereStructResult {
    sql: String, values: Vec<String>,
}

// My Results
type QueryOpResult = Result<&'static str, String>;
type WhereResult = Result<WhereStructResult, String>;

// TODO: Move there later
#[derive(Clone)]
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

fn collect_params(v: &Vec<Params>, collect_values: bool) -> Vec<String> {
    let mut pid: i32 = 0;
    v.iter()
        .map(|param| {
            pid = pid + 1;
            let p = param.clone();
            let result = match p.v.contains(".") {
                true => {
                    let value_splited = p.v.split(".").collect::<Vec<&str>>();
                    let operator = match query_operator(value_splited[0]) {
                        Ok(op) => op,
                        _ => "operator not found"
                    };
                    (operator, value_splited[1])
                },
                _ => ("=", p.v.as_str())
            };
            if collect_values {
                format!("{}", result.1)
            } else {
                format!("{}{}${}", p.k, result.0, pid)
            }
        })
        .collect()
}

// Get QueryString and return chunck WHERE statement with values
pub fn ext_where(qs: QueryString) -> WhereResult {
    let where_syntax: String;
    let ref iter_params = qs.query;
    let where_values = collect_params(iter_params, true);
    let where_fields = collect_params(iter_params, false);

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
}
