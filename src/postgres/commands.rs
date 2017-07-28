extern crate rocket;

use rocket::request::{FromForm, FormItems};


pub struct WhereStructResult {
    sql: String, values: Vec<String>,
}

// My Results
type QueryOpResult = Result<String, String>;
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

pub fn query_operator(qop: &String) -> QueryOpResult {
    match &qop[..] {
        "$eq" => Ok("=".to_string()),
        "$ne" => Ok("!=".to_string()),
        "$gt" => Ok(">".to_string()),
        "$lt" => Ok("<".to_string()),
        "$gte" => Ok(">=".to_string()),
        "$lte" => Ok("<=".to_string()),
        "$in" => Ok("IN".to_string()),
        "$nin" => Ok("NOT IN".to_string()),
        "$notnull" => Ok("IS NOT NULL".to_string()),
        "$null" => Ok("IS NULL".to_string()),
         _ => Err(format!("operator {} not found", qop)),
    }
}

// Get QueryString and return chunck WHERE statement with values
pub fn ext_where(qs: QueryString) -> WhereResult {
    let where_syntax: String;
    let mut pid: u32 = 0;
    let mut where_values: Vec<String> = Vec::new();
    let mut where_fields: Vec<String> = Vec::new();

    for params in &qs.query {
        pid = pid + 1;
        let result = match params.v.contains(".") {
            true => {
                let v: Vec<&str> = params.v.split(".").collect();
                let op = match query_operator(&v[0].to_string()) {
                    Ok(x) => x,
                    Err(e) => return Err(e.to_string())
                };
                (op, v[1].to_string())
            },
            _ => ("=".to_string(), params.v.to_string())
        };
        where_fields.push(format!("{}{}${}", params.k, result.0, pid));
        where_values.push(result.1);
    }

    if pid > 1 {
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
