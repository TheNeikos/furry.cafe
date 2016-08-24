use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::modifiers::Redirect;
use iron::Url;

use error::{self};
use views;
use models;
use views::layout::LayoutData;

pub fn index(req: &mut Request) -> IronResult<Response> {
    let sub_list = try!(models::submission::last(20));

    let data = LayoutData::from_request(req);
    let mut resp = Response::with((status::Ok, template!(views::submission::index(&sub_list, &data))));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

pub fn new(req: &mut Request) -> IronResult<Response> {
    unimplemented!()
}

pub fn create(req: &mut Request) -> IronResult<Response> {
    unimplemented!()
}

pub fn show(req: &mut Request) -> IronResult<Response> {
    unimplemented!()
}

pub fn edit(req: &mut Request) -> IronResult<Response> {
    unimplemented!()
}

pub fn update(req: &mut Request) -> IronResult<Response> {
    unimplemented!()
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    unimplemented!()
}

