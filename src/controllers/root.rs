use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron_login::User as UserTrait;

use controllers;
use views;
use views::layout::LayoutData;
use models::user::User;

pub fn handler(req: &mut Request) -> IronResult<Response> {
    let user = User::get_login(req).get_user();

    if user.is_some() {
        return controllers::submission::index(req);
    }


    let mut resp = Response::with((status::Ok, views::shared::root(&LayoutData::from_request(req)).unwrap()));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

