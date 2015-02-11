#![feature(path)]

extern crate iron;
extern crate router;
extern crate mount;
extern crate "static" as static_file;
extern crate "iron-defer" as defer;

use iron::prelude::*;
use iron::status;
use router::Router;
use mount::Mount;
use static_file::Static;
use defer::Defer;

fn main() {
	// ------------------------------------------------------
	// Dynamic handlers
	// ------------------------------------------------------

	fn hello_world(_: &mut Request) -> IronResult<Response> {
		Ok(Response::with((status::Ok, "Hello world")))
	}

	fn about(_: &mut Request) -> IronResult<Response> {
		Ok(Response::with((status::Ok, "About")))
	}

	// ------------------------------------------------------
	// Mount points
	// ------------------------------------------------------

	let mut mount = Mount::new();

	mount.mount("/", Static::new(Path::new("content")));

	// ------------------------------------------------------
	// Routes
	// ------------------------------------------------------

	let mut router = Router::new();

    router.get("/", Static::new(Path::new("content/helloworld.html")));
    router.get("/hello", hello_world);
    router.get("/about", about);

    // ------------------------------------------------------
	// Combine routes + mount points
	// ------------------------------------------------------

    let defer = Defer::using(router, mount);

    Iron::new(defer).listen("localhost:3000").unwrap();
}
