use std::borrow::Cow;
use maud::{Markup, PreEscaped};

use views;
use error;
use views::layout::LayoutData;
use views::components::form::*;
use views::components::Column;
use models::user::User;
use models::user_profile::NewUserProfile;

// TODO: Add profile errors
pub fn edit(user: &User, profile: &NewUserProfile, _errors: Option<()>, data: &LayoutData) -> Result<Markup, error::FurratoriaError> {
    let body = html! {
        div.row (Column::custom(6, 3, html! {
            h1 { "Edit User Profile: " (user.name) }
            (PreEscaped(Form::new(FormMethod::Post, &format!("/users/{}/profile", user.id))
              .with_fields(&[
                   &Textarea::new("Profile", "user_bio")
                        .with_value(&profile.bio)
                        .with_errors(None),
                   &Input::new("Banner", "banner_image")
                        .with_value(&format!("{}", &profile.banner_image.as_ref().unwrap_or(&0)))
                        .with_type("number"),
                   &Input::new("", "")
                        .with_value("Update")
                        .with_type("submit")
                        .with_class("btn btn-primary"),
              ])))
        }))
    };

    Ok(views::layout::application(Cow::Borrowed("Register"), body, data))
}
