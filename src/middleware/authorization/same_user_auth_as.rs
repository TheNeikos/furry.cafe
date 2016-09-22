use iron::prelude::*;

use models::user::User;
use middleware::authorization::UserRequirement;

#[derive(Clone, Debug)]
pub struct SameUserAuthAs<'a>(pub &'a User);

impl<'a> UserRequirement for SameUserAuthAs<'a> {
    fn check(&self, user: Option<&User>, _req: &mut Request) -> bool {
        if user.is_none() { return false };
        let user = user.unwrap();

        user.id == self.0.id
    }
}

