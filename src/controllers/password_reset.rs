use iron::prelude::*;
use iron::status;

use views;
use views::layout::LayoutData;

pub fn ask_reset(req: &mut Request) -> IronResult<Response> {
    let data = LayoutData::from_request(req);
    let resp = Response::with((status::Ok, try!(views::password_reset::ask_reset(&data))));
    Ok(resp)
}

pub fn reset(req: &mut Request) -> IronResult<Response> {
    use params::{Params, Value};

    let data = LayoutData::from_request(req);

    let map = req.get_ref::<Params>().unwrap();

    let email = match map.get("user_email") {
        Some(&Value::String(ref email)) => Some(&email[..]),
        _ => None
    };

    let resp = Response::with((status::Ok, try!(views::password_reset::reset_sent(email, &data))));

    return Ok(resp);
}
