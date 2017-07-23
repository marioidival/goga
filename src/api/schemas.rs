use api::sapper::{Result, SapperModule, SapperRouter, Request, Response};

#[derive(Clone)]
pub struct SchemasEndpoint;

impl SchemasEndpoint {
    fn get_schemas(_: &mut Request) -> Result<Response> {
        let schemas = json!({
            "schemas": "not found",
        });
        res_json!(schemas)
    }
}

impl SapperModule for SchemasEndpoint {
    fn router(&self, router: &mut SapperRouter) -> Result<()> {
        router.get("/schemas", SchemasEndpoint::get_schemas);
        Ok(())
    }
}
