use server::sapper::Result;
use server::sapper::SapperModule;
use server::sapper::Request;
use server::sapper::Response;
use server::sapper::SapperRouter;


#[derive(Clone)]
pub struct GogaApp;

impl GogaApp {
    fn index(req: &mut Request) -> Result<Response> {
        let mut response = Response::new();
        response.write_body("hello, boy!".to_string());
        Ok(response)
    }
}

impl SapperModule for GogaApp {
    fn router(&self, router: &mut SapperRouter) -> Result<()> {
        router.get("/", GogaApp::index);

        Ok(())
    }
}
