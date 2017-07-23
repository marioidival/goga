use api::sapper::{Result, SapperModule, SapperRouter, Request, Response};

#[derive(Clone)]
pub struct TablesEndpoint;

impl TablesEndpoint {
    fn get_tables(_: &mut Request) -> Result<Response> {
        let tables = json!({
            "tables": "not found",
        });
        res_json!(tables)
    }
}

impl SapperModule for TablesEndpoint {
    fn router(&self, router: &mut SapperRouter) -> Result<()> {
        router.get("/tables", TablesEndpoint::get_tables);
        Ok(())
    }
}
