use std::borrow::Cow;
use maud::PreEscaped;

use views;
use error;
use views::layout::LayoutData;
use views::components::user::UserLink;
use views::components::form::*;
use models::invite::Invite;

pub fn index(invites: &[Invite], data: &LayoutData) -> Result<String, error::FurratoriaError> {
    let mut buffer = String::new();
    let mut partial = String::new();
    try!(html!(partial,
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
                                ^(PreEscaped(UserLink(&user)))
                            } @else {
                                "<NONE>"
                            }
                        }

                        td {
                            ^(inv.invite_key)
                        }
                    }
                }
            }
        }

        ^(PreEscaped(Form::new(FormMethod::Post, "/admin/invites")
          .with_fields(&[
               &Input::new("", "")
                    .with_value("Create")
                    .with_type("submit")
                    .with_class("btn btn-primary")
          ])))
    ));

    try!(views::layout::application(&mut buffer, Cow::Borrowed("Invites"), Cow::Owned(partial), data));

    Ok(buffer)
}
