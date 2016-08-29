use iron::prelude::*;

use models::user::User;
use middleware::authorization::UserRequirement;

#[derive(Clone, Debug)]
pub struct LoggedIn;

impl UserRequirement for LoggedIn {
    fn check(&self, user: Option<&User>, _req: &mut Request) -> bool {
        user.is_some()
    }
}

