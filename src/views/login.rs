use std::borrow::Cow;

use maud::{Markup, PreEscaped};

use views;
use error;
use views::layout::LayoutData;
use views::components::form::*;
use models::user::UserError;

pub fn login(errors: Option<UserError>, data: &LayoutData) -> Result<Markup, error::FurratoriaError> {
    let body = html! {
        div.row div class="col-sm-6 offset-sm-3" {
            h1 "Login"

            (PreEscaped(Form::new(FormMethod::Post, "/login")
              .with_fields(&[
                   &Input::new("Email", "user_email")
                        .with_errors(errors.as_ref().map(|x| &x.email)),
                   &Input::new("Password", "user_password")
                        .with_type("password")
                        .with_errors(errors.as_ref().map(|x| &x.password)),
                   &Input::new("", "")
                        .with_value("Login")
                        .with_type("Submit")
                        .with_class("btn btn-primary")
              ])))
        }
    };

    Ok(views::layout::application(Cow::Borrowed("Login"), body, data))
}

pub fn logout(data: &LayoutData) -> Result<Markup, error::FurratoriaError> {
    let body = html! {
        div.row div class="col-sm-6 offset-sm-3" {
            h1 "Logout"

            (PreEscaped(Form::new(FormMethod::Post, "/logout")
              .with_fields(&[
                    &Input::new("", "")
                        .with_value("Logout")
                        .with_type("Submit")
                        .with_class("btn btn-primary")
             ])))
        }
    };

    Ok(views::layout::application(Cow::Borrowed("Logout"), body, data))
}
