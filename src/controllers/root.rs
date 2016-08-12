
use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;

use views;

pub fn handler(_req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::with((status::Ok, views::shared::root().unwrap()));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

