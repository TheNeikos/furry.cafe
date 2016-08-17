use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::modifiers::Redirect;
use iron::Url;

use error::{self};
use views;
use models;
use views::layout::LayoutData;

pub fn edit(req: &mut Request) -> IronResult<Response> {
    use router::Router;
    let id = match req.extensions.get::<Router>().unwrap().find("id") {
        Some(t) => {
            match t.parse::<_>() {
                Ok(t) => t,
                Err(_) => return Err(IronError::new(error::BadFormattingError::new(), temp_redirect!("/users/")))
            }
        }
        None => {
            return Err(IronError::new(error::BadFormattingError::new(), temp_redirect!("/users/")));
        }
    };

    let user = match try!(models::user::find(id)) {
        Some(u) => u,
        None => {
            let mut resp = Response::with(status::NotFound);
            resp.headers.set(ContentType::html());
            return Ok(resp)
        }
    };

    let data = LayoutData::from_request(req);
    let profile = try!(user.get_profile());
    let new_profile = models::user_profile::NewUserProfile::from(&profile);
    println!("{:#?}", new_profile);
    let mut resp = Response::with((status::Ok, template!(views::user_profile::edit(&user, &new_profile, None, &data))));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

pub fn update(req: &mut Request) -> IronResult<Response> {
    use params::{Params, Value};
    use router::Router;

    let data = LayoutData::from_request(req);

    let id = match req.extensions.get::<Router>().unwrap().find("id") {
        Some(t) => {
            match t.parse::<_>() {
                Ok(t) => t,
                Err(_) => return Err(IronError::new(error::BadFormattingError::new(), temp_redirect!("/users/")))
            }
        }
        None => {
            return Err(IronError::new(error::BadFormattingError::new(), temp_redirect!("/users/")));
        }
    };

    let user = match try!(models::user::find(id)) {
        Some(u) => u,
        None => {
            let mut resp = Response::with(status::NotFound);
            resp.headers.set(ContentType::html());
            return Ok(resp)
        }
    };

    let old_profile = try!(user.get_profile());

    let map = req.get_ref::<Params>().unwrap();
    let bio = match map.get("user_bio") {
        Some(&Value::String(ref bio)) => &bio[..],
        _ => &old_profile.bio
    };

    let new = models::user_profile::NewUserProfile::new(&user, bio);
    match user.set_profile(new.clone()) {
        Ok(()) => {
           Ok(Response::with((status::SeeOther, Redirect(url!(format!("/users/{}", user.id))))))
        }
        Err(err) => {
            let mut resp = Response::with((status::Ok, template!(views::user_profile::edit(&user, &new, None, &data))));
            resp.headers.set(ContentType::html());
            Ok(resp)
        }
    }
}
