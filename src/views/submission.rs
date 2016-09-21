use iron::Request;

use std::borrow::Cow;
use maud::PreEscaped;

use views;
use error;
use views::layout::LayoutData;
use views::components::user::UserLink;
use views::components::form::*;
use views::components::button::*;
use models::submission::{Submission, SubmissionError, NewSubmission};
use middleware::authorization::{self, UserAuthorization};

pub fn index(subs: &[Submission], data: &LayoutData, req: &mut Request) -> Result<String, error::FurratoriaError> {
    let mut buffer = String::new();
    let mut partial = String::new();
    try!(html!(partial,
        h1 { "Submissions" }

        @if req.current_user_can(authorization::LoggedIn) {
            a.btn.btn-primary href=^(url!("/submissions/new")) "New Submission"
        }

        div.submissions @for sub in subs {
            div a href=^(url!(format!("/submissions/{}", sub.id))) {
                div.card {
                    img.card-img-top src=^(try!(sub.get_image()).map(|x| x.get_path()).unwrap_or(String::from("/todo"))) /
                    div.card-block {
                        h4.card-title ^(sub.title)
                        h6.card-subtitle.text-muted {
                            "by "
                            ^({PreEscaped(UserLink(&try!(sub.get_submitter())))})
                        }
                    }
                }
            }
        }
    ));

    try!(views::layout::application(&mut buffer, Cow::Borrowed("Submissions"), Cow::Owned(partial), data));

    Ok(buffer)
}

pub fn new(errors: Option<SubmissionError>, data: &LayoutData, sub: Option<&NewSubmission>) -> Result<String, error::FurratoriaError> {
    let mut buffer = String::new();
    let mut partial = String::new();
    try!(html!(partial,
        div.row div class="col-sm-6 offset-sm-3" {
            h1 { "Upload new Submission" }

            ^(PreEscaped(Form::new(FormMethod::Post, "/submissions/")
              .with_encoding("multipart/form-data")
              .with_fields(&[
                   &Input::new("Image", "sub_image")
                        .with_type("file")
                        .with_errors(errors.as_ref().map(|x| &x.image)),
                   &Input::new("Title", "sub_name")
                        .with_value(sub.as_ref().map(|x| &x.title).unwrap_or(&""))
                        .with_errors(errors.as_ref().map(|x| &x.title)),
                   &Textarea::new("Description", "sub_desc")
                        .with_value(sub.as_ref().map(|x| &x.description).unwrap_or(&""))
                        .with_errors(None),
                   &Input::new("", "")
                        .with_value("Upload")
                        .with_type("submit")
                        .with_class("btn btn-primary")
              ])))
        }
    ));

    try!(views::layout::application(&mut buffer, Cow::Borrowed("Register"), Cow::Owned(partial), data));

    Ok(buffer)
}

pub fn show(sub: &Submission, data: &LayoutData, req: &mut Request) -> Result<String, error::FurratoriaError> {
    let mut buffer = String::new();
    let mut partial = String::new();

    let image = match try!(sub.get_image()) {
        Some(i) => i,
        None => {
            return Err(error::FurratoriaError::Template(Box::new(error::FurratoriaError::NotFound)))
        }
    };

    let user = try!(sub.get_submitter());

    try!(html!(partial,
        div.submission {
            div.row div class="col-md-10 offset-md-1" {
                div.submission.clearfix {
                    img src=^(image.get_path()) alt=^(format!("{}'s Submission", user.name)) /
                }

                div {
                    h1.title { ^sub.title }
                    span.author {
                        "by "
                        ^(PreEscaped(UserLink(&user)))
                    }
                }
            }

            @if req.current_user_can(authorization::LoggedIn) {
                div.row div class="col-md-10 offset-md-1" {
                    div.sub_actions {
                        a.btn.btn-primary href=^(url!(format!("/users/{}/edit", user.id))) "Favorit"
                        " "
                        a.btn.btn-secondary href=^(image.get_path()) "Full Size"
                        " "
                        @if req.current_user_can(authorization::SameUserAuth) {
                            a.btn.btn-info href=^(url!(format!("/submissions/{}/edit", sub.id))) "Edit"
                            " "
                        }
                        a.btn.btn-danger href=^(url!(format!("/users/{}/profile/edit", user.id))) "Signal"
                    }
                }
            }

            div.row div class="col-md-10 offset-md-1" {
                div.submission_description {
                    ^(views::markdown::parse(&sub.description))
                }
            }


        }
    ));

    try!(views::layout::application(&mut buffer, Cow::Owned(format!("{}", sub.title)), Cow::Owned(partial), data));

    Ok(buffer)
}

pub fn edit(sub: &Submission, errors: Option<SubmissionError>, data: &LayoutData) -> Result<String, error::FurratoriaError> {
    let mut buffer = String::new();
    let mut partial = String::new();
    try!(html!(partial,
        div.row div class="col-sm-6 offset-sm-3" {
            h1 { "Update your Submission" }

            ^(PreEscaped(Form::new(FormMethod::Post, &format!("/submissions/{}", sub.id))
              .with_encoding("multipart/form-data")
              .with_fields(&[
                   &Input::new("Image", "sub_image")
                        .with_type("file")
                        .with_errors(errors.as_ref().map(|x| &x.image)),
                   &Input::new("Title", "sub_name")
                        .with_value(&sub.title[..])
                        .with_errors(errors.as_ref().map(|x| &x.title)),
                   &Textarea::new("Description", "sub_desc")
                        .with_value(&sub.description)
                        .with_errors(None),
                   &Input::new("", "")
                        .with_value("Update")
                        .with_type("submit")
                        .with_class("btn btn-primary")
              ])))
        }

        div.row div class="col-sm-6 offset-sm-3" {
            ^(PreEscaped(Button::new("Delete", &format!("/submissions/{}/delete", sub.id)).with_method(RequestMethod::Post)))
        }
    ));

    try!(views::layout::application(&mut buffer, Cow::Borrowed("Register"), Cow::Owned(partial), data));

    Ok(buffer)
}
