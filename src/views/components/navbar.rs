use std::fmt::{self, Display, Formatter};

use models::user::User;
use views::layout::LayoutData;

struct NavbarEntry<'a, 'b> {
    name: &'a str,
    path: &'b str,
}

static NAVBAR_ENTRIES: &'static [NavbarEntry<'static, 'static>] = &[
    NavbarEntry {
        name: "Submissions",
        path: "/submissions/"
    },
    NavbarEntry {
        name: "Members",
        path: "/users/"
    },
];

#[derive(Clone, Debug)]
pub struct Navbar<'a> {
    path: String,
    user: &'a Option<User>,
}

impl<'a> Navbar<'a> {
    pub fn new(ld: &'a LayoutData) -> Navbar<'a> {
        let mut p = ld.url.path().join("/");

        p.insert(0, '/');

        Navbar {
            path: p,
            user: &ld.user,
        }
    }
}

impl<'a> Display for Navbar<'a> {
    fn fmt(&self, mut f: &mut Formatter) -> Result<(), fmt::Error> {

        f.write_str(&html!(
            nav.navbar.navbar-static-top.navbar-light.bg-faded {
                a.navbar-brand href="/" "Furratoria"
                ul.nav.navbar-nav {
                    @for item in NAVBAR_ENTRIES {
                        @if self.path.starts_with(item.path) {
                            li.nav-item.active {
                                a.nav-link href=(item.path) (item.name)
                            }
                        } @else {
                            li.nav-item {
                                a.nav-link href=(item.path) (item.name)
                            }
                        }
                    }
                }

                @if let &Some(ref user) = self.user {
                    ul.nav.navbar-nav.pull-xs-right {
                        div.dropdown {
                            li.nav-item.active {
                                a.nav-link href=(format!("/users/{}", user.id)) (user.name)
                            }
                            li.nav-item {
                                a.nav-link.small href="/logout" "Logout"
                            }
                        }
                    }
                } @else {
                    ul.nav.navbar-nav.pull-xs-right {
                        li.nav-item.active {
                            a.nav-link href="/login" "Login"
                        }
                        li.nav-item.active {
                            a.nav-link href="/users/new" "Register"
                        }
                    }
                }
            }
        ).into_string())
    }
}

