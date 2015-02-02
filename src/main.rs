extern crate iron;
extern crate router;
extern crate mount;
extern crate "static" as static_file;
extern crate "iron-defer" as defer;

use iron::prelude::*;
use iron::{status, AfterMiddleware};
use router::Router;
use mount::Mount;
use static_file::Static;
use defer::Defer;

fn main() {
	fn hello_world(_: &mut Request) -> IronResult<Response> {
		Ok(Response::with((status::Ok, "Hello world")))
	}

	fn about(_: &mut Request) -> IronResult<Response> {
		Ok(Response::with((status::Ok, "About")))
	}

	let mut mount = Mount::new();

	mount.mount("/", Static::new(Path::new(".")));

	let mut router = Router::new();

    router.get("/", hello_world);
    router.get("/about", about);

    let defer = Defer::using(router, mount);

    Iron::new(defer).listen("localhost:3000").unwrap();
}
