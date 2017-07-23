use server::sapper::Result;
use server::sapper::SapperModule;
use server::sapper::SapperAppShell;
use server::sapper::Request;
use server::sapper::Response;
use server::sapper::SapperRouter;
use sapper_std;

#[derive(Clone)]
pub struct GogaApp;
pub struct GogaShellApp;

impl SapperAppShell for GogaShellApp {
    fn before(&self, req: &mut Request) -> Result<()> {
        sapper_std::init(req)?;
        Ok(())
    }

    fn after(&self, req: &Request, res: &mut Response) -> Result<()> {
        sapper_std::finish(req, res)?;
        Ok(())
    }
}

impl GogaApp {
    fn index(_: &mut Request) -> Result<Response> {
        let welcome = json!({
            "msg": "Welcome to Goga",
        });
        res_json!(welcome)
    }
}

impl SapperModule for GogaApp {
    fn router(&self, router: &mut SapperRouter) -> Result<()> {
        router.get("/", GogaApp::index);
        Ok(())
    }
}
