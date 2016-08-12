use std::borrow::Cow;

use views;
use views::layout::LayoutData;
use models::user::UserError;

pub fn login(errors: Option<UserError>, data: &LayoutData) -> Result<String, ::std::fmt::Error> {
    let mut buffer = String::new();
    let mut partial = String::new();
    try!(html!(partial,
        h1 "Login"

        form method="post" action="/login" {
            div {
                label for="user_email" "Email:"
                input type="text" id="user_email" name="user_email" ""
                @if let &Some(ref errors) = &errors {
                    @for err in &errors.email {
                        p class="error" ^err
                    }
                }
            }
            div {
                label for="user_password" "Password:"
                input type="password" id="user_password" name="user_password" ""
                @if let &Some(ref errors) = &errors {
                    @for err in &errors.password {
                        p class="error" ^err
                    }
                }
            }

            input type="submit" /
        }
    ));

    try!(views::layout::application(&mut buffer, Cow::Borrowed("Login"), Cow::Owned(partial), data));
    Ok(buffer)
}

pub fn logout(data: &LayoutData) -> Result<String, ::std::fmt::Error> {
    let mut buffer = String::new();
    let mut partial = String::new();

    try!(html!(partial,
        h1 "Logout"

        form method="post" action="/logout" {
            input type="submit" value="Logout" /
        }
    ));

    try!(views::layout::application(&mut buffer, Cow::Borrowed("Logout"), Cow::Owned(partial), data));
    Ok(buffer)
}
