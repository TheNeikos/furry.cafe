use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::modifiers::Redirect;
use iron::Url;
use iron_login::User as UserTrait;

use views;
use models;
use views::layout::LayoutData;
use models::user::{User, UserError};

pub fn new(req: &mut Request) -> IronResult<Response> {
    let data = LayoutData::from_request(req);
    let mut resp = Response::with((status::Ok, template!(views::login::login(None, &data))));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

pub fn create(req: &mut Request) -> IronResult<Response> {
    use params::{Params, Value};

    let login = User::get_login(req);

    // TODO: Replace with custom data object
    let data = LayoutData::from_request(req);

    let map = req.get_ref::<Params>().unwrap();

    let email = match map.get("user_email") {
        Some(&Value::String(ref email)) => Some(&email[..]),
        _ => None
    };

    let password = match map.get("user_password") {
        Some(&Value::String(ref pass)) => Some(&pass[..]),
        _ => None
    };

    if email.is_none() || password.is_none() {
        let mut err = UserError::new();
        err.email.push("cannot be empty");
        err.password.push("cannot be empty");
        let mut resp = Response::with((status::Ok, template!(views::login::login(Some(err), &data))));
        resp.headers.set(ContentType::html());
        return Ok(resp);
    }

    let email = email.unwrap();
    let password = password.unwrap();

    let user = match models::user::with_email_password(email, password) {
        Ok(Some(user)) => user,
        _ => {
            // TODO: Actually tell the user no fitting was found
            let mut resp = Response::with((status::Ok, template!(views::login::login(None, &data))));
            resp.headers.set(ContentType::html());
            return Ok(resp);
        }
    };

    let login = login.log_in(user);

    // TODO: Add config for url?
    return Ok(Response::with((login, (status::SeeOther, Redirect(Url::parse("http://localhost:3000/").unwrap())))))
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    let data = LayoutData::from_request(req);
    let mut resp = Response::with((status::Ok, template!(views::login::logout(&data))));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

pub fn destroy(req: &mut Request) -> IronResult<Response> {
    let logout = User::get_login(req).log_out();

    // TODO: Add config for url?
    return Ok(Response::with((logout, (status::SeeOther, Redirect(Url::parse("http://localhost:3000/").unwrap())))))
}
