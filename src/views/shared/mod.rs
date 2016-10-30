use maud::{Markup};

use views;
use std::borrow::Cow;

use views::layout::LayoutData;
use views::components::Column;

pub fn root(data: &LayoutData) -> Result<Markup, ::std::fmt::Error> {
    let body = html! {
        div.row (Column::new(html! {
            h1 "Welcome to Furratoria!"
            p {
                "We're a bunch of Furries cruising the unknown worlds of artistic space. "
                "Here we share and discuss Artwork we have created or bought about our Characters."
            }

            h2 "Artworks"
            p {
                "Art in any form has its place around here. "
                "To help you organize it we have several features you can use once you are registered:"

                ul {
                    li "Artist Tagging — Collaboration? Commission? You can tag other users (or link to profiles on other sites) to always give credit"
                    li "Characters — You will be able to create a page just for your Character and tag him in pictures, making it easy to keep it all in one place"
                    li "Tagged Favorites — Only want the relevant ones? Filter them easily"
                }
            }

            p {
                strong "If that sounds exciting, come and join us! "
                a.btn.btn-primary href="/users/new" "Sign up"
            }
        }))
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
