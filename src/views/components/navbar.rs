use maud::Render;

use models::user::User;
use views::layout::LayoutData;

struct NavbarEntry<'a, 'b> {
    name: &'a str,
    path: &'b str,
}

static NAVBAR_ENTRIES: &'static [NavbarEntry<'static, 'static>] = &[
    NavbarEntry {
        name: "Gallery",
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

impl<'a> Render for Navbar<'a> {
    fn render_to(&self, mut f: &mut String) {

        f.push_str(&html!(
            nav.navbar.navbar-muted {
                div.clearfix {
                    button.navbar-toggler.float-xs-right.hidden-sm-up.collapsed type="button" data-toggle="collapse" data-target="#fc-main-nav" ""
                    a.brand.hidden-sm-up href="/" "Furry Café"
                }
                div.navbar-toggleable-xs.collapse#fc-main-nav {
                    ul.nav.navbar-nav {
                        li.nav-item.active.hidden-xs-down {
                            a.nav-link.brand href="/" "Furry Café"
                        }
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
                        ul.nav.navbar-nav.float-sm-up-right {
                            div.dropdown {
                                li.nav-item.active {
                                    a.nav-link href=(format!("/users/{}", user.id)) (user.name)
                                }
                                li.nav-item {
                                    a.nav-link href="/logout" "Logout"
                                }
                            }
                        }
                    } @else {
                        ul.nav.navbar-nav.float-sm-up-right {
                            li.nav-item.active {
                                a.nav-link href="/login" "Login"
                            }
                            li.nav-item.active {
                                a.nav-link href="/users/new" "Register"
                            }
                        }
                    }
                }
            }
        ).into_string())
    }
}

