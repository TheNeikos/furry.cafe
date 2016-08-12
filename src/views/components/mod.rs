
use std::fmt::{self, Display, Formatter};

use iron::Url;

struct NavbarEntry<'a, 'b> {
    name: &'a str,
    path: &'b str,
}

static NAVBAR_ENTRIES: &'static [NavbarEntry<'static, 'static>] = &[
    NavbarEntry {
        name: "Home",
        path: "/"
    },
    NavbarEntry {
        name: "Members",
        path: "/users/"
    },
];

pub struct Navbar {
    path: String,
}

impl Navbar {
    pub fn new(u: &Url) -> Navbar {
        let mut p = u.path().join("/");

        p.insert(0, '/');

        Navbar {
            path: p,
        }
    }
}

impl Display for Navbar {
    fn fmt(&self, mut f: &mut Formatter) -> Result<(), fmt::Error> {

        html!(f,
            nav.navbar.navbar-static-top.navbar-light.bg-faded {
                a.navbar-brand href="/" "ArtMoe"
                ul.nav.navbar-nav {
                    @for item in NAVBAR_ENTRIES {
                        @if item.path == self.path {
                            li.nav-item.active {
                                a.nav-link href=^item.path ^item.name
                            }
                        } @else {
                            li.nav-item {
                                a.nav-link href=^item.path ^item.name
                            }
                        }
                    }
                }
            }
        )
    }
}


