use std::borrow::Cow;

use maud::PreEscaped;

use views;
use views::layout::LayoutData;
use views::components::form::*;
use models::user::UserError;

pub fn login(errors: Option<UserError>, data: &LayoutData) -> Result<String, ::std::fmt::Error> {
    let mut buffer = String::new();
    let mut partial = String::new();
    try!(html!(partial,
        div.row div class="col-sm-6 offset-sm-3" {
            h1 "Login"


            ^(PreEscaped(Form::new(FormMethod::Post, "/login")
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
    ));

    try!(views::layout::application(&mut buffer, Cow::Borrowed("Login"), Cow::Owned(partial), data));
    Ok(buffer)
}

pub fn logout(data: &LayoutData) -> Result<String, ::std::fmt::Error> {
    let mut buffer = String::new();
    let mut partial = String::new();

    try!(html!(partial,
        div.row div class="col-sm-6 offset-sm-3" {
            h1 "Logout"

            form method="post" action="/logout" {
                input.btn.btn-primary type="submit" value="Logout" /
            }
        }
    ));

    try!(views::layout::application(&mut buffer, Cow::Borrowed("Logout"), Cow::Owned(partial), data));
    Ok(buffer)
}
