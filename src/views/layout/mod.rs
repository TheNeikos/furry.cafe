
use maud::PreEscaped;

use std::borrow::Cow;
use std::fmt;

pub fn application(mut data: &mut fmt::Write, title: Cow<str>, partial: Cow<str>) -> Result<(), fmt::Error> {
    html!(data, {
        html {
            head {
                title ^title
                link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/4.0.0-alpha.3/css/bootstrap.min.css" integrity="sha384-MIwDKRSSImVFAZCVLtU0LMDdON6KVCrZHyVQQj6e8wIEJkW4tvwqXrbMIya1vriY" crossorigin="anonymous" /
                script src="https://maxcdn.bootstrapcdn.com/bootstrap/4.0.0-alpha.3/js/bootstrap.min.js" integrity="sha384-ux8v3A6CPtOTqOzMKiuo3d/DomGaaClxFYdCu2HPMBEkf6x2xiDyJ7gkXU0MWwaD" crossorigin="anonymous" ""
            }

            body {
                div.container-fluid {
                    nav.navbar.navbar-static-top.navbar-light.bg-faded {
                        a.navbar-brand href="/" "ArtMoe"
                        ul.nav.navbar-nav {
                            li.nav-item {
                                a.nav-link href="/" "Home"
                            }
                        }
                    }

                    ^PreEscaped(partial)

                    hr /
                    footer {
                        p ^PreEscaped("ArtMoe 2016 &copy; Neikos")
                    }
                }
            }
        }
    })
}
