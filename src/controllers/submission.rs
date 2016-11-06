use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::modifiers::Redirect;
use iron_login::User as UserTrait;

use error::{self};
use views;
use models;
use views::layout::{LayoutData, OpenGraph};
use models::user::User;
use models::submission::{self, Visibility};

pub fn index(req: &mut Request) -> IronResult<Response> {
    let user = User::get_login(req).get_user();
    let sub_list = try!(models::submission::SubmissionFilter::new(None).with_viewer(user.as_ref()).run());

    let data = LayoutData::from_request(req);
    let mut resp = Response::with((status::Ok, try!(views::submission::index(&sub_list, &data, req, None))));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

pub fn create(req: &mut Request) -> IronResult<Response> {
    use models::submission::Submission;

    let user = User::get_login(req).get_user().unwrap();

    let new_submission = match models::submission::NewSubmission::new(&user) {
        Ok(new_submission) => new_submission,
        Err(_) => {
            // TODO: Add notice for error
            return Ok(Response::with(temp_redirect!("/submissions/")));
        }
    };

    let id = try!(Submission::create_from(new_submission));

    // TODO: Add config for url?
    return Ok(Response::with(temp_redirect!(format!("/submissions/{}/edit", id))));
}

pub fn show(req: &mut Request) -> IronResult<Response> {
    use router::Router;
    let submission = try!(find_by_id!(req, "id", submission));

    if let Some(id) = req.extensions.get::<Router>().unwrap().find("id") {
        if id != submission.full_id() {
            return Ok(Response::with(temp_redirect!(format!("/submissions/{}", submission.full_id()))));
        }
    } else {
            return Ok(Response::with(temp_redirect!("/")));
    }

    let mut data = LayoutData::from_request(req);
    data.meta = Some(OpenGraph {
        title: Some(submission.title.clone()),
        url: Some(url!(format!("/submissions/{}", submission.id))),
        image: {
            let image = try!(submission.get_image());
            if let Some(image) = image {
                Some(url!(image.get_with_size(1000, 1000)?.get_path()))
            } else {
                None
            }
        }
    });
    let mut resp = Response::with((status::Ok, try!(views::submission::show(&submission, &data, req))));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

pub fn edit(req: &mut Request) -> IronResult<Response> {
    let data = LayoutData::from_request(req);

    let submission = try!(find_by_id!(req, "id", submission));

    let mut resp = Response::with((status::Ok, try!(views::submission::edit(&submission, None, &data))));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

pub fn update(req: &mut Request) -> IronResult<Response> {
    use params::{Params, Value};
    use std::str::FromStr;

    let data = LayoutData::from_request(req);

    let submission = try!(find_by_id!(req, "id", submission));

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

    let sub_visibility = match map.get("sub_visibility") {
        Some(&Value::String(ref vis)) => Some(try!(i32::from_str(vis).map_err(|x| error::FurratoriaError::from(x)))),
        _ => None
    };

    let update_submission = match models::submission::UpdateSubmission::new(&submission, image, sub_name, sub_desc, sub_visibility) {
        Ok(update_submission) => update_submission,
        Err((us, err)) => {
            let mut submission = submission;
            if us.get_visibility() == Some(Visibility::Unpublished) || us.has_image() || submission.has_image() {
                try!(submission.update(&us));
                submission = iexpect!(try!(submission::find(submission.id)));
            }

            let mut resp = Response::with((status::Ok, try!(views::submission::edit(&submission, Some(err), &data))));
            resp.headers.set(ContentType::html());
            return Ok(resp);
        }
    };

    try!(submission.update(&update_submission));

    return Ok(Response::with((status::SeeOther, Redirect(url!(format!("/submissions/{}", submission.id))))))
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    let submission = try!(find_by_id!(req, "id", submission));

    try!(submission.delete());

    return Ok(Response::with(temp_redirect!("/submissions/")));
}

