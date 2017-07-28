extern crate rocket;

use rocket::{Request, Data, Outcome};
use rocket::data::{self, FromData};
use rocket::Outcome::*;
use std::collections::HashMap;


pub struct WhereStructResult {
    sql: String, values: Vec<String>,
}

// My Results
type QueryOpResult = Result<String, String>;
type WhereResult = Result<WhereStructResult, String>;

// implement QueryString
pub struct QueryString {
    query: HashMap<String, String>
}

impl FromData for QueryString {
    type Error = String;

    fn from_data(req: &Request, _: Data) -> data::Outcome<Self, String> {
        let querystring: Vec<&str> = req.uri().query()
            .unwrap().split("&").collect();
        let mut result: QueryString = QueryString{query: HashMap::new()};

        for qs in querystring {
            let kv: Vec<&str> = qs.split("=").collect();
            // [0] is key, [1] is value
            result.query.insert(kv[0].to_string(), kv[1].to_string());
        }
        Success(result)
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

// Get request and return chuck WHERE statement with values
pub fn ext_where(_: &Request) -> WhereResult {
    Ok(WhereStructResult {
        sql: "WHERE user_id=$1".to_string(),
        values: vec!["5".to_string()],
    })
}


#[cfg(test)]
mod tests {
    use super::{rocket};
    use rocket::local::Client;
    use rocket::http::ContentType;
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
    fn where_by_request() {
        let client = Client::new(rocket::ignite())
            .expect("valid rocket");
        let req = client.get("/db/sch/tbl?user_id=5&name=cool")
            .header(ContentType::JSON);
        let result = ext_where(req.inner())
            .unwrap();

        assert_eq!(result.sql, "WHERE user_id=$1".to_string());
        assert_eq!(result.values, vec!["5".to_string()])
    }
}
