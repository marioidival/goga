pub fn query_operator(qop: &String) -> Result<String, String> {
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

#[cfg(test)]
mod tests {
    use postgres::commands::*;

    #[test]
    fn it_get_opetators() {
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
    fn it_get_opetators_error() {
        let invalid = "!tn".to_string();
        match query_operator(&invalid) {
            Ok(_) => println!("no result"),
            Err(e) => assert_eq!(format!("operator {} not found", invalid), e)
        }
        // assert_eq!(expected, query_operatorv2(&invalid));
    }
}
