use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::modifiers::Redirect;
use iron_login::User as UserTrait;

use error::{self};
use views;
use models;
use views::layout::LayoutData;
use models::user::User;

pub fn index(req: &mut Request) -> IronResult<Response> {
    let sub_list = try!(models::submission::last(20));

    let data = LayoutData::from_request(req);
    let mut resp = Response::with((status::Ok, template!(views::submission::index(&sub_list, &data))));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

pub fn new(req: &mut Request) -> IronResult<Response> {
    let data = LayoutData::from_request(req);
    let mut resp = Response::with((status::Ok, template!(views::submission::new(None, &data, None))));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

pub fn create(req: &mut Request) -> IronResult<Response> {
    use params::{Params, Value};
    use models::submission::Submission;

    let user = User::get_login(req).get_user().unwrap();
    let data = LayoutData::from_request(req);

    let map = req.get_ref::<Params>().unwrap();

    let sub_name = match map.get("sub_name") {
        Some(&Value::String(ref name)) => Some(&name[..]),
        _ => None
    };

    let sub_desc = match map.get("sub_desc") {
        Some(&Value::String(ref name)) => Some(&name[..]),
        _ => None
    };

    let image = match map.get("sub_image") {
        Some(&Value::File(ref file)) => Some(file),
        _ => None
    };

    let new_submission = match models::submission::NewSubmission::new(&user, image, sub_name, sub_desc) {
        Ok(new_submission) => new_submission,
        Err((err, new_submission)) => {
            let mut resp = Response::with((status::Ok, template!(views::submission::new(Some(err), &data, Some(&new_submission)))));
            resp.headers.set(ContentType::html());
            return Ok(resp);
        }
    };

    let id = try!(Submission::create_from(new_submission));

    // TODO: Add config for url?
    return Ok(Response::with(temp_redirect!(format!("/submissions/{}", id))));
}

pub fn show(req: &mut Request) -> IronResult<Response> {
    use router::Router;

    let id = match req.extensions.get::<Router>().unwrap().find("id") {
        Some(t) => {
            match t.parse::<_>() {
                Ok(t) => t,
                Err(_) => return Err(IronError::new(error::BadFormattingError::new(), temp_redirect!("/submissions/")))
            }
        }
        None => {
            return Err(IronError::new(error::BadFormattingError::new(), temp_redirect!("/submissions/")));
        }
    };

    let submission = match try!(models::submission::find(id)) {
        Some(u) => u,
        None => {
            error!("Could not find submission with id: {}", id);
            let mut resp = Response::with(status::NotFound);
            resp.headers.set(ContentType::html());
            return Ok(resp)
        }
    };

    let data = LayoutData::from_request(req);
    let mut resp = Response::with((status::Ok, template!(views::submission::show(&submission, &data))));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

pub fn edit(_req: &mut Request) -> IronResult<Response> {
    unimplemented!()
}

pub fn update(_req: &mut Request) -> IronResult<Response> {
    unimplemented!()
}

pub fn delete(_req: &mut Request) -> IronResult<Response> {
    unimplemented!()
}

