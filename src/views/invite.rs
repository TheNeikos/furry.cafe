use std::borrow::Cow;
use maud::{Markup, PreEscaped};

use views;
use error;
use views::layout::LayoutData;
use views::components::user::UserLink;
use views::components::form::*;
use models::invite::Invite;

pub fn index(invites: &[Invite], data: &LayoutData) -> Result<Markup, error::FurryError> {
    let body = html! {
        h1 { "Invites" }

        table {
            thead {
                th "User"
                    th "Key"
            }
            tbody {
                @for inv in invites {
                    tr {
                        td {
                            @if let Some(user) = try!(inv.get_user()) {
                                (UserLink(&user))
                            } @else {
                                "<NONE>"
                            }
                        }

                        td {
                            (inv.invite_key)
                        }
                    }
                }
            }
        }

        (PreEscaped(Form::new(FormMethod::Post, "/admin/invites")
          .with_fields(&[
               &Input::new("", "")
                    .with_value("Create")
                    .with_type("submit")
                    .with_class("btn btn-primary")
          ])))
    };

    Ok(views::layout::application(Cow::Borrowed("Invites"), body, data))
}
