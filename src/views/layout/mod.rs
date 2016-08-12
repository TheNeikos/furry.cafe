
use maud::PreEscaped;

use std::borrow::Cow;
use std::fmt;

pub fn application(mut data: &mut fmt::Write, title: Cow<str>, partial: Cow<str>) -> Result<(), fmt::Error> {
    html!(data, {
        html {
            head {
                title ^title
            }

            body {
                div.content {
                    ^PreEscaped(partial)
                }

                div.footer {
                    ^PreEscaped("ArtNeikos 2016 &copy; Neikos")
                }
            }
        }
    })
}
