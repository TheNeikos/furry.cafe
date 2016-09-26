use maud::Markup;

use views;
use std::borrow::Cow;

use views::layout::LayoutData;

pub fn root(data: &LayoutData) -> Result<Markup, ::std::fmt::Error> {
    let body = html! {
        h1 { "Hello World!" }
    };

    Ok(views::layout::application(Cow::Borrowed("Homepage"), body, data))
}

pub fn unauthorized(data: &LayoutData) -> Result<Markup, ::std::fmt::Error> {
    let body = html! {
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
    };

    Ok(views::layout::application(Cow::Borrowed("Unauthorized"), body, data))
}

pub fn notfound(data: &LayoutData) -> Result<Markup, ::std::fmt::Error> {
    let body = html! {
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
    };

    Ok(views::layout::application(Cow::Borrowed("Not Found"), body, data))
}

pub fn internalerror(data: &LayoutData) -> Result<Markup, ::std::fmt::Error> {
    let body = html! {
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
    };

    Ok(views::layout::application(Cow::Borrowed("Error"), body, data))
}
