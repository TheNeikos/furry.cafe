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
                div.alert.alert-warning {
                    "We've sent an email to "
                    strong (email)
                    " if an account is associated with it, you will get an email from us shortly"
                    " with further instructions."
                }
            } @else {
                div.alert.alert-error {
                    "Could not make out the email, please try again"
                }
            }
        }))
    };

    Ok(views::layout::application("Login", body, data))
}


pub mod email {
    use models::user::User;
    use models::unique_code::UniqueCode;

    pub fn text(user: &User, code: &UniqueCode) -> String {
        let path : &String = &::macros::URL_PATH;
        format!("
        Hello {username}
        ======{underline}

        Someone (hopefully you) has requested a password reset!
        If this was indeed you, please click on the link below:
        {reset_link}

        If it wasn't you, please do ignore this e-mail!

        Cheers
        Your Furry Café Team", username = user.name,
        reset_link = {format!("{}/reset_password?code={}", path, code.code)},
        underline = {(0..user.name.len()).map(|_| "=").collect::<Vec<_>>().join("")})
    }

    pub fn html(user: &User, code: &UniqueCode) -> String {
        let path : &String = &::macros::URL_PATH;
        let url = format!("{}/reset_password?code={}", path, code.code);
        (html! {
            h1 { "Hello " (user.name) }

            p {
                "Someone (hopefully you) has requested a password reset!"
                br/
                "If this was indeed you, please click on the link below:"
                br/
                a href=(url) "Password Reset"
                br/
                "Or copy this link into your browser: " (url)
            }

            p {
                "If it wasn't you, please do ignore this e-mail!"
                br/
                br/
                "Cheers"
                br/
                "Your Furry Café Team"
            }
        }).into_string()
    }

}
