use std::borrow::Cow;
use maud::{PreEscaped, RenderOnce};

use views;
use views::layout::LayoutData;
use models::submission::Submission;

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
