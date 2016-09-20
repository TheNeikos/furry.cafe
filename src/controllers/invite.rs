use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::modifiers::Redirect;

use views;
use models;
use views::layout::LayoutData;

pub fn index(req: &mut Request) -> IronResult<Response> {
    let sub_list = try!(models::invite::all());

    let data = LayoutData::from_request(req);
    let mut resp = Response::with((status::Ok, try!(views::invite::index(&sub_list, &data))));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

pub fn create(_req: &mut Request) -> IronResult<Response> {
    use models::invite::Invite;

    let new_invite = models::invite::NewInvite::new();

    try!(Invite::create_from(new_invite));

    return Ok(Response::with((status::Found, Redirect(url!("/admin/invites")))));
}
