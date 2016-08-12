
use iron::prelude::*;
use iron::status;

pub fn handler(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "This is a test")))
}


