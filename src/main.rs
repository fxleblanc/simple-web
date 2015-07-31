extern crate iron;
extern crate router;
extern crate mount;

mod response_time;

use iron::prelude::*;
use router::Router;
use mount::Mount;
use response_time::ResponseTime;

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello world")))
}

fn main() {
    let mut router = Router::new();
    router.get("/hello", hello_world);

    let mut mount = Mount::new();
    mount.mount("/api", router);

    let mut chain = Chain::new(mount);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);
    Iron::new(chain).http("localhost:3000").unwrap();
}
