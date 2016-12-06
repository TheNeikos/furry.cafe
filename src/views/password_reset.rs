use maud::Markup;

use views;
use error;
use views::layout::LayoutData;
use views::components::form::*;
use views::components::Column;

pub fn ask_reset(data: &LayoutData) -> Result<Markup, error::FurryError> {
    let body = html! {
        div.row (Column::custom(6, 3, html! {
            h1 "Request Password Reset"

            (Form::new(FormMethod::Post, "/password_reset")
              .with_fields(&[
                   &Input::new("Email", "user_email"),
                   &Input::new("", "")
                        .with_value("Login")
                        .with_type("Submit")
                        .with_class("btn btn-primary")
              ]))
        }))
    };

    Ok(views::layout::application("Login", body, data))
}

pub fn reset_sent(email: Option<&str>, data: &LayoutData) -> Result<Markup, error::FurryError> {
    let body = html! {
        div.row (Column::custom(6, 3, html! {
            h1 "Request Password Reset"

            @if let Some(email) = email {
                div.alert.warning {
                    "We've sent an email to "
                    strong (email)
                    " if an account is associated with it, you will see "
                }
            } @else {
                div.alert.error {
                    "Could not make out the email, please try again"
                }
            }
        }))
    };

    Ok(views::layout::application("Login", body, data))
}

