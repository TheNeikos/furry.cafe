use iron::prelude::*;
use iron::status;

use lettre::transport::smtp::{SmtpTransport, SmtpTransportBuilder, SecurityLevel};
use lettre::transport::smtp::authentication::Mechanism;
use lettre::transport::smtp::SUBMISSION_PORT;
use lettre::transport::EmailTransport;
use lettre::email::EmailBuilder;

use views;
use models::user;
use models::unique_code::{self, NewUniqueCode, UniqueCodeType, UniqueCode};
use views::layout::LayoutData;

pub fn ask_reset(req: &mut Request) -> IronResult<Response> {
    let data = LayoutData::from_request(req);
    let resp = Response::with((status::Ok, try!(views::password_reset::ask_reset(&data))));
    Ok(resp)
}

pub fn get_reset(req: &mut Request) -> IronResult<Response> {
    use params::{Params, Value};

    let data = LayoutData::from_request(req);

    let map = req.get_ref::<Params>().unwrap();

    let email = match map.get("user_email") {
        Some(&Value::String(ref email)) => Some(&email[..]),
        _ => None
    };

    if let Some(email) = email {
        if let Some(user) =  try!(user::find_by_email(email)) {

            let password_code = try!(UniqueCode::create_from(NewUniqueCode::new(UniqueCodeType::PasswordReset)));
            let password_code = try!(unique_code::find_by_id(password_code));

            let send_email = EmailBuilder::new()
                .to(email)
                .from("password@furry.cafe")
                .text(&views::password_reset::email::text(&user, &password_code))
                .html(&views::password_reset::email::html(&user, &password_code))
                .subject("Password Reset Request")
                .build().unwrap();

            lazy_static! {
                static ref MAILGUN_NAME : String = {
                    ::std::env::var("MAILGUN_NAME").expect("MAILGUN_NAME must be set")
                };
                static ref MAILGUN_DOMAIN : String = {
                    ::std::env::var("MAILGUN_DOMAIN").expect("MAILGUN_DOMAIN must be set")
                };
                static ref MAILGUN_PASSWORD : String = {
                    ::std::env::var("MAILGUN_PASSWORD").expect("MAILGUN_PASSWORD must be set")
                };
            };

            let mut mailer = SmtpTransportBuilder::new(("smtp.mailgun.org", SUBMISSION_PORT)).unwrap()
                .hello_name(&MAILGUN_DOMAIN)
                .credentials(&MAILGUN_NAME, &MAILGUN_PASSWORD)
                .security_level(SecurityLevel::AlwaysEncrypt)
                .smtp_utf8(true)
                .authentication_mechanism(Mechanism::Plain)
                .connection_reuse(true).build();

            try!(mailer.send(send_email).map_err(|e| ::error::FurryError::from(e)));
            mailer.close();
        }
    }


    let resp = Response::with((status::Ok, try!(views::password_reset::reset_sent(email, &data))));

    return Ok(resp);
}
