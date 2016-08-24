use iron::prelude::*;
use iron::status::Status;
use iron_login::User as U;

use error::UnauthorizedError;
use models::user::{self, User};
use middleware::authorization::UserRequirement;

#[derive(Clone, Debug)]
pub struct LoggedIn;

impl UserRequirement for LoggedIn {
    fn check(&self, user: Option<&User>, req: &mut Request) -> bool {
        user.is_some()
    }
}

