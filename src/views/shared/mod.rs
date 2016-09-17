
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

pub fn unauthorized(data: &LayoutData) -> Result<String, ::std::fmt::Error> {
    let mut buffer = String::new();
    let mut partial = String::new();
    try!(html!(partial,
        h1 { "Oops! You're not allowed to do that" }

        div.alert.alert-danger {
            strong "I can't let you do that"
            hr /
            p {
                "If you want you can try going "
                a href="javascript:history.go(-1)" "back"
                " or go to a "
                a href="/" "safe page"
            }
        }
    ));

    try!(views::layout::application(&mut buffer, Cow::Borrowed("Root"), Cow::Owned(partial), data));

    Ok(buffer)
}

pub fn notfound(data: &LayoutData) -> Result<String, ::std::fmt::Error> {
    let mut buffer = String::new();
    let mut partial = String::new();
    try!(html!(partial,
        h1 { "Nope, can't see what you're looking for" }

        div.alert.alert-warning {
            strong "I can't seem to find what you are looking for :C"
            hr /
            p {
                "If you want you can try going "
                a href="javascript:history.go(-1)" "back"
                " or go to a "
                a href="/" "safe page"
            }
        }
    ));

    try!(views::layout::application(&mut buffer, Cow::Borrowed("root"), Cow::Owned(partial), data));

    Ok(buffer)
}

pub fn internalerror(data: &LayoutData) -> Result<String, ::std::fmt::Error> {
    let mut buffer = String::new();
    let mut partial = String::new();
    try!(html!(partial,
        h1 { "Something took a wrong turn here... Sorry!" }

        div.alert.alert-warning {
            strong "Something broke and we can only show you this error page :C"
            hr /
            p {
                "If you want you can try going "
                a href="javascript:history.go(-1)" "back"
                " or go to a "
                a href="/" "safe page"
            }
        }
    ));

    try!(views::layout::application(&mut buffer, Cow::Borrowed("root"), Cow::Owned(partial), data));

    Ok(buffer)
}
