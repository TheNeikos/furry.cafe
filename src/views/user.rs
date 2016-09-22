use iron::Request;

use std::borrow::Cow;
use maud::PreEscaped;

use views;
use error;
use views::layout::LayoutData;
use views::components::user::{UserAvatar, UserLink};
use views::components::form::*;
use models::user::{UserError, User, NewUser};
use models::user_role::Role;
use models::user_profile::UserProfile;
use middleware::authorization::{self, UserAuthorization};

pub fn new(errors: Option<UserError>, data: &LayoutData, user: Option<&NewUser>) -> Result<String, error::FurratoriaError> {
    let mut buffer = String::new();
    let mut partial = String::new();
    try!(html!(partial,
        div.row div class="col-sm-6 offset-sm-3" {
            h1 { "Register" }

            ^(PreEscaped(Form::new(FormMethod::Post, "/users/")
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
        }
    ));

    try!(views::layout::application(&mut buffer, Cow::Borrowed("Register"), Cow::Owned(partial), data));

    Ok(buffer)
}

pub fn index(users: &[User], data: &LayoutData) -> Result<String, error::FurratoriaError> {
    let mut buffer = String::new();
    let mut partial = String::new();
    try!(html!(partial,
        h1 { "Users" }

        @for user in users {
            div class="user" {
                ^(PreEscaped(UserLink(user)))
            }
        }
    ));

    try!(views::layout::application(&mut buffer, Cow::Borrowed("Users"), Cow::Owned(partial), data));

    Ok(buffer)
}

pub fn show(user: &User, role: Role, profile: &UserProfile, data: &LayoutData, req: &mut Request) -> Result<String, error::FurratoriaError> {
    let mut buffer = String::new();
    let mut partial = String::new();

    let banner = try!(profile.get_banner());

    try!(html!(partial,
        div.user_profile {
            @if let Some(image) = banner {
                div.row div class="col-md-10 offset-md-1" {
                    div.banner style=^(format!("background-image: url('{}');height: {};", image.get_path(), image.height)) /
                }
            }

            div.row div class="col-md-10 offset-md-1" {
                div.user_info.clearfix {
                    ^PreEscaped(UserAvatar(&user))
                    h1.user_name { ^user.name }
                    div.user_role {
                        strong "Role: "
                        ^role.as_str()
                    }
                }
            }

            @if req.current_user_can(authorization::LoggedIn) {
                div.row div class="col-md-10 offset-md-1" {
                    div.user_actions {
                        @if req.current_user_can(authorization::SameUserAuth) {
                            a.btn.btn-info href=^(url!(format!("/users/{}/edit", user.id))) "Edit"
                                " "
                                a.btn.btn-info href=^(url!(format!("/users/{}/profile/edit", user.id))) "Edit Profile"
                        }
                    }
                }
            }

            div.row div class="col-md-10 offset-md-1" {
                div.user_bio {
                    ^(views::markdown::parse(&profile.bio))
                }
            }


        }
    ));

    try!(views::layout::application(&mut buffer, Cow::Owned(format!("User: {}", user.name)), Cow::Owned(partial), data));

    Ok(buffer)
}

pub fn edit(user: &User, errors: Option<UserError>, data: &LayoutData) -> Result<String, error::FurratoriaError> {
    let mut buffer = String::new();
    let mut partial = String::new();
    try!(html!(partial,
        h1 { "Edit User " ^(user.name) }
        ^(PreEscaped(Form::new(FormMethod::Post, &format!("/users/{}", user.id))
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
    ));

    try!(views::layout::application(&mut buffer, Cow::Borrowed("Register"), Cow::Owned(partial), data));

    Ok(buffer)
}

#[cfg(test)]
mod test {
    use models::user::{UserError, User};
    use super::*;

    #[test]
    fn new_user_view_content() {
        let new_string = new(None).unwrap();

        assert!(new_string.contains("user_name"));
        assert!(new_string.contains("user_email"));
        assert!(new_string.contains("user_password"));
    }

    #[test]
    fn new_user_view_errors() {
        let mut errors = UserError::new();
        errors.email.push("Error 1");
        errors.name.push("Error 2");
        errors.password.push("Error 3");
        let new_string = new(Some(errors)).unwrap();

        assert!(new_string.contains("Error 1"));
        assert!(new_string.contains("Error 2"));
        assert!(new_string.contains("Error 3"));
    }

    #[test]
    fn index_user_view() {
        let users = vec![
            User {
                id:            1,
                email:         "test@example.com".into(),
                password_hash: "asdf".into(),
                name:          "Test User #1".into(),
            },
            User {
                id:            2,
                email:         "test2@example.com".into(),
                password_hash: "asdf".into(),
                name:          "Test User #2".into(),
            },
            User {
                id:            3,
                email:         "test3@example.com".into(),
                password_hash: "asdf".into(),
                name:          "Test User #3".into(),
            },
        ];
        let index_string = index(&users).unwrap();
        for user in &users {
            assert!(index_string.contains(&user.name));
            assert!(index_string.contains(&format!("/users/{}", user.id)));
        }
    }

    #[test]
    fn show_user_view() {
        let user = User {
            id:            1,
            email:         "test@example.com".into(),
            password_hash: "asdf".into(),
            name:          "Test User #1".into(),
        };
        let show_string = show(&user).unwrap();
        assert!(show_string.contains(&user.name));
    }

    #[test]
    fn edit_user_view() {
        let user = User {
            id:            1,
            email:         "test@example.com".into(),
            password_hash: "asdf".into(),
            name:          "Test User #1".into(),
        };
        let edit_string = edit(&user, None).unwrap();
        assert!(edit_string.contains("user_name"));
        assert!(edit_string.contains("user_password"));
        assert!(!edit_string.contains("user_email"));
    }

    #[test]
    fn edit_user_view_test() {
        let user = User {
            id:            1,
            email:         "test@example.com".into(),
            password_hash: "asdf".into(),
            name:          "Test User #1".into(),
        };
        let mut errors = UserError::new();
        errors.name.push("Error 1");
        errors.password.push("Error 2");

        let edit_string = edit(&user, Some(errors)).unwrap();
        assert!(edit_string.contains("user_name"));
        assert!(edit_string.contains("user_password"));
        assert!(!edit_string.contains("user_email"));
        assert!(edit_string.contains("Error 1"));
        assert!(edit_string.contains("Error 2"));
    }
}

