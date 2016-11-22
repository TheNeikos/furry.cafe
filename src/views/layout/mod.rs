
use std::borrow::Cow;

use maud::{Markup, PreEscaped, Render};
use iron::Url;
use iron::Request;
use mount::OriginalUrl;
use iron_login::User as UserTrait;

use views::components::Navbar;
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
                title (format!("{} - Furry Café", title))
                link rel="stylesheet" href="/assets/external/css/bootstrap.min.css" /
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
                div.container-fluid {
                    (PreEscaped(Navbar::new(&layout_data)))

                    (partial)

                    hr /
                    footer {
                        p {
                            (PreEscaped("Furry 2016 &copy; Neikos &mdash; "))
                            small.revision (include_str!("../../../.git/refs/heads/master"))
                        }
                    }
                }
            }
        }
    )
}
