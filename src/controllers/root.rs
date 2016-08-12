
use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;

use views;

use views::layout::LayoutData;

pub fn handler(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::with((status::Ok, views::shared::root(&LayoutData::from_request(req)).unwrap()));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

