
use views;
use std::borrow::Cow;

use views::layout::LayoutData;

pub fn root(data: &LayoutData) -> Result<String, ::std::fmt::Error> {
    let mut buffer = String::new();
    let mut partial = String::new();
    try!(html!(partial,
        h1 { "Hello World!" }
    ));

    try!(views::layout::application(&mut buffer, Cow::Borrowed("Root"), Cow::Owned(partial), data));

    Ok(buffer)
}
