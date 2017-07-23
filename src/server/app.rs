use server::sapper::Result;
use server::sapper::SapperModule;
use server::sapper::SapperAppShell;
use server::sapper::Request;
use server::sapper::Response;
use server::sapper::PathParams;
use server::sapper::SapperRouter;
use sapper_std;

#[derive(Clone)]
pub struct GogaApp;
pub struct GogaShellApp;

impl SapperAppShell for GogaShellApp {
    fn before(&self, req: &mut Request) -> Result<()> {
        println!("before all");
        sapper_std::init(req)?;
        Ok(())
    }

    fn after(&self, req: &Request, res: &mut Response) -> Result<()> {
        println!("after all");
        sapper_std::finish(req, res)?;
        Ok(())
    }
}

impl GogaApp {
    fn index(req: &mut Request) -> Result<Response> {
        let params = get_path_params!(req);
        let hello = t_param!(params, "hello").parse::<String>().unwrap();

        let mut response = Response::new();
        response.write_body(format!("hello, {}!", hello).to_string());
        Ok(response)
    }
}

impl SapperModule for GogaApp {

    fn router(&self, router: &mut SapperRouter) -> Result<()> {
        router.get("/:hello", GogaApp::index);

        Ok(())
    }
}
