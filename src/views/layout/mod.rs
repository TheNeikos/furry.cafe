
use std::borrow::Cow;

use maud::{Markup, PreEscaped, Render};
use iron::Url;
use iron::Request;
use mount::OriginalUrl;
use iron_login::User as UserTrait;

use views::components::{Navbar, Column};
use models::user::User;

pub struct OpenGraph {
    pub title: Option<String>,
    pub url:   Option<Url>,
    pub image: Option<Url>,
}

impl Render for OpenGraph {
    fn render_to(&self, w: &mut String) {
        w.push_str(&html!{
            @if let Some(ref t) = self.title {
                meta property="og:title" content=(t) /
            }
            @if let Some(ref t) = self.url {
                meta property="og:url" content=(t) /
            }
            @if let Some(ref t) = self.image {
                meta property="og:image" content=(t) /
                meta name="twitter:image" content=(t) /
                meta name="twitter:card" content="photo" /
                link rel="image_src" href=(t) /
            }
        }.into_string());
    }
}

pub struct LayoutData {
    pub url: Url,
    pub user: Option<User>,
    pub meta: Option<OpenGraph>,
}

impl LayoutData {
    pub fn from_request(req: &mut Request) -> LayoutData {
        LayoutData {
            url: req.extensions.get::<OriginalUrl>().unwrap_or(&url!("")).clone(),
            user: User::get_login(req).get_user(),
            meta: None,
        }
    }
}


pub fn application(title: Cow<str>,
                   partial: Markup,
                   layout_data: &LayoutData,
                   ) -> Markup
{
    html!(
        (PreEscaped("<!DOCTYPE html>"))
        html {
            head {
                meta charset="utf-8" /
                meta http-equiv="X-UA-Compatible" content="IE=edge" /
                meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no" /
                title (format!("{} - Furry Café", title))
                link rel="stylesheet" href="/assets/application.min.css" /
                script src="/assets/external/js/jquery-3.1.0.min.js" ""
                script src="/assets/external/js/tether.min.js" ""
                script src="/assets/external/js/bootstrap.min.js" ""
                meta property="og:site_name" content="furry.cafe" /
                meta property="og:type" content="website" /
                @if let Some(ref info) = layout_data.meta {
                    (info)
                }
            }

            body {
                (Navbar::new(&layout_data))
                div.container-fluid {

                    (Column::new(html! {
                        div.alert.alert-warning {
                            strong "This site is still in Alpha! "
                            "If you have found an error or wish to make suggestions you can"
                            " chat me up on "
                            a href="https://telegram.me/TheNeikos" "Telegram"
                            " or submit an issue on "
                            a href="https://github.com/TheNeikos/furry.cafe" "Github"
                        }
                    }))

                    (partial)

                    hr /
                    footer {
                        p {
                            a.brand href="/"  "Furry Café "
                            (PreEscaped(" 2016 &copy; Neikos &mdash; "))
                            small.revision "Pre-Alpha"
                        }
                    }
                }
            }
        }
    )
}
