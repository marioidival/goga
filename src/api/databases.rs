use api::sapper::{Result, SapperModule, SapperRouter, Request, Response};

#[derive(Clone)]
pub struct DatabasesEndpoint;

impl DatabasesEndpoint {
    fn get_databases(_: &mut Request) -> Result<Response> {
        let databases = json!({
            "databases": "not found",
        });
        res_json!(databases)
    }
}

impl SapperModule for DatabasesEndpoint {
    fn router(&self, router: &mut SapperRouter) -> Result<()> {
        router.get("/databases", DatabasesEndpoint::get_databases);
        Ok(())
    }
}
