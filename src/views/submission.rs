use std::borrow::Cow;
use maud::{PreEscaped, RenderOnce};

use views;
use views::layout::LayoutData;
use views::components::form::*;
use models::submission::{Submission, SubmissionError, NewSubmission};

pub fn index(subs: &[Submission], data: &LayoutData) -> Result<String, ::std::fmt::Error> {
    let mut buffer = String::new();
    let mut partial = String::new();
    try!(html!(partial,
        h1 { "Submissions" }

        @for sub in subs {
            div.submission {
                a.submission-link href=^(url!(format!("/submissions/{}", sub.id))) ^sub.title
            }
        }
    ));

    try!(views::layout::application(&mut buffer, Cow::Borrowed("Submissions"), Cow::Owned(partial), data));

    Ok(buffer)
}

pub fn new(errors: Option<SubmissionError>, data: &LayoutData, sub: Option<&NewSubmission>) -> Result<String, ::std::fmt::Error> {
    let mut buffer = String::new();
    let mut partial = String::new();
    try!(html!(partial,
        div.row div class="col-sm-6 offset-sm-3" {
            h1 { "Upload new Submission" }

            ^(PreEscaped(Form::new(FormMethod::Post, "/submissions/")
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
