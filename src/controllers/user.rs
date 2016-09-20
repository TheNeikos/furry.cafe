use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::modifiers::Redirect;

use error::{self};
use views;
use models;
use views::layout::LayoutData;
use models::user;

pub fn index(req: &mut Request) -> IronResult<Response> {
    let user_list = try!(models::user::find_all());

    let data = LayoutData::from_request(req);
    let mut resp = Response::with((status::Ok, try!(views::user::index(&user_list, &data))));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

pub fn new(req: &mut Request) -> IronResult<Response> {
    let data = LayoutData::from_request(req);
    let mut resp = Response::with((status::Ok, try!(views::user::new(None, &data, None))));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

pub fn create(req: &mut Request) -> IronResult<Response> {
    use params::{Params, Value};
    use models::user::User;

    let data = LayoutData::from_request(req);

    let map = req.get_ref::<Params>().unwrap();

    let username = match map.get("user_name") {
        Some(&Value::String(ref name)) => Some(&name[..]),
        _ => None
    };

    let email = match map.get("user_email") {
        Some(&Value::String(ref email)) => Some(&email[..]),
        _ => None
    };

    let password = match map.get("user_password") {
        Some(&Value::String(ref pass)) => Some(&pass[..]),
        _ => None
    };

    let invite_code = match map.get("invite_code") {
        Some(&Value::String(ref code)) => Some(&code[..]),
        _ => None
    };

    let new_user = match models::user::NewUser::new(username, email, password) {
        Ok(new_user) => new_user,
        Err((err, new_user)) => {
            let mut resp = Response::with((status::Ok, try!(views::user::new(Some(err), &data, Some(&new_user)))));
            resp.headers.set(ContentType::html());
            return Ok(resp);
        }
    };

    let invite = {
        match invite_code {
            Some(code) => {
                info!("Trying code {}", code);
                match try!(models::invite::find_by_key(code)) {
                    Some(i) => {
                        if i.user_id.is_some() {
                            return Ok(Response::with(temp_redirect!("/users/")))
                        } else {
                            i
                        }
                    },
                    None => return Ok(Response::with(temp_redirect!("/users/")))
                }
            }
            None => return Ok(Response::with(temp_redirect!("/users/")))
        }
    };

    let id = try!(User::create_from(new_user));

    try!(invite.update(&models::invite::UpdateInvite::create_for(&invite, id)));

    // TODO: Add config for url?
    return Ok(Response::with(temp_redirect!("/users/")))
}

pub fn show(req: &mut Request) -> IronResult<Response> {
    let user = try!(find_by_id!(req, "id", user));

    let role = try!(user.get_role());
    let profile = try!(user.get_profile());
    let data = LayoutData::from_request(req);
    let mut resp = Response::with((status::Ok, try!(views::user::show(&user, role, &profile, &data))));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

pub fn edit(req: &mut Request) -> IronResult<Response> {
    let user = try!(find_by_id!(req, "id", user));

    let data = LayoutData::from_request(req);
    let mut resp = Response::with((status::Ok, try!(views::user::edit(&user, None, &data))));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

pub fn update(req: &mut Request) -> IronResult<Response> {
    use params::{Params, Value};

    let data = LayoutData::from_request(req);

    let user = try!(find_by_id!(req, "id", user));

    let map = req.get_ref::<Params>().unwrap();
    let username = match map.get("user_name") {
        Some(&Value::String(ref name)) => Some(&name[..]),
        _ => None
    };

    let password = match map.get("user_password") {
        Some(&Value::String(ref pass)) => Some(&pass[..]),
        _ => None
    };

    let avatar = match map.get("user_avatar") {
        Some(&Value::File(ref file)) => Some(file),
        _ => None
    };

    let update_user = match models::user::UpdateUser::new(username, password, avatar) {
        Ok(update_user) => update_user,
        Err(err) => {
            let mut resp = Response::with((status::Ok, try!(views::user::edit(&user, Some(err), &data))));
            resp.headers.set(ContentType::html());
            return Ok(resp);
        }
    };

    try!(user.update(&update_user));

    return Ok(Response::with((status::SeeOther, Redirect(url!(format!("/users/{}", user.id))))))
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    let user = try!(find_by_id!(req, "id", user));

    try!(user.delete());

    return Ok(Response::with((status::SeeOther, Redirect(url!("/users")))))
}

