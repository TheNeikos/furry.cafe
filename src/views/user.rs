use iron::Request;

use std::borrow::Cow;
use maud::{Markup, PreEscaped};

use views;
use error;
use views::layout::LayoutData;
use views::components::user::{UserAvatar, UserLink};
use views::components::form::*;
use views::components::Column;
use models::user::{UserError, User, NewUser};
use models::user_role::Role;
use models::user_profile::UserProfile;
use middleware::authorization::{self, UserAuthorization};

pub fn new(errors: Option<UserError>, data: &LayoutData, user: Option<&NewUser>) -> Result<Markup, error::FurryError> {
    let body = html! {
        div.row (Column::custom(6, 3, html! {
            h1 { "Register" }

            (PreEscaped(Form::new(FormMethod::Post, "/users/")
              .with_fields(&[
                   &Input::new("Name", "user_name")
                        .with_value(user.as_ref().map(|x| &x.name).unwrap_or(&""))
                        .with_errors(errors.as_ref().map(|x| &x.name)),
                   &Input::new("Email", "user_email")
                        .with_value(user.as_ref().map(|x| &x.email).unwrap_or(&""))
                        .with_errors(errors.as_ref().map(|x| &x.email)),
                   &Input::new("Password", "user_password")
                        .with_type("password")
                        .with_errors(errors.as_ref().map(|x| &x.password)),
                   &Input::new("Invite Key", "invite_code"),
                   &Input::new("", "")
                        .with_value("Register")
                        .with_type("submit")
                        .with_class("btn btn-primary")
              ])))
        }))
    };

    Ok(views::layout::application(Cow::Borrowed("Register"), body, data))
}

pub fn index(users: &[User], data: &LayoutData) -> Result<Markup, error::FurryError> {
    let body = html! {
        div.row (Column::new(html! {
            h1 { "Users" }

            @for user in users {
                div class="user" {
                    (UserLink(user))
                }
            }
        }))
    };

    Ok(views::layout::application(Cow::Borrowed("Users"), body, data))
}

pub fn show(user: &User, role: Role, profile: &UserProfile, data: &LayoutData, req: &mut Request) -> Result<Markup, error::FurryError> {
    let banner = try!(profile.get_banner());

    let body = html! {
        div.user_profile {
            @if let Some(image) = banner {
                div.row {
                    (Column::new(html! {
                        div.banner style=(format!("background-image: url('{}');height: {};", image.get_path(), image.height)) ""
                    }))
                }
            }

            div.row (Column::new(html! {
                div.user_info.clearfix {
                    (UserAvatar(&user, (250, 250)))
                    h1.user_name { (user.name) }
                    div.user_role {
                        strong "Role: "
                        (role.as_str())
                    }
                }
            }))

            div.row (Column::new(html! {
                div.user_actions {
                    @if req.current_user_can(authorization::LoggedIn) {
                        @if req.current_user_can(authorization::SameUserAuth) {
                            a.btn.btn-info href=(url!(format!("/users/{}/edit", user.id))) "Edit"
                            a.btn.btn-info href=(url!(format!("/users/{}/profile/edit", user.id))) "Edit Profile"
                        }
                    }
                    a.btn.btn-info href=(url!(format!("/users/{}/submissions", user.id))) "Gallery"
                }
            }))

            div.row (Column::new(html! {
                div.user_bio {
                    (views::markdown::parse(&profile.bio))
                }
            }))
        }
    };

    Ok(views::layout::application(Cow::Owned(format!("User: {}", user.name)), body, data))
}

pub fn edit(user: &User, errors: Option<UserError>, data: &LayoutData) -> Result<Markup, error::FurryError> {
    let body = html! {
        div.row (Column::custom(6, 3, html! {
            h1 { "Edit User " (user.name) }
            (PreEscaped(Form::new(FormMethod::Post, &format!("/users/{}", user.id))
              .with_encoding("multipart/form-data")
              .with_fields(&[
                   &Input::new("Name", "user_name")
                        .with_value(&user.name)
                        .with_errors(errors.as_ref().map(|x| &x.name)),
                   &Input::new("Email", "user_email")
                        .with_value(&user.email)
                        .with_errors(errors.as_ref().map(|x| &x.email)),
                   &Input::new("Password", "user_password")
                        .with_type("password")
                        .with_errors(errors.as_ref().map(|x| &x.password)),
                   &Input::new("Avatar", "user_avatar")
                        .with_type("file")
                        .with_errors(errors.as_ref().map(|x| &x.profile_image)),
                   &Input::new("", "")
                        .with_value("Update")
                        .with_type("submit")
                        .with_class("btn btn-primary")
              ])))
        }))
    };

    Ok(views::layout::application(Cow::Borrowed("Register"), body, data))
}
