use iron::prelude::*;

use models::user::User;
use models::user_role::Role;
use middleware::authorization::UserRequirement;

#[derive(Clone, Debug)]
pub struct HasRole(pub Role);

impl UserRequirement for HasRole {
    fn check(&self, user: Option<&User>, _req: &mut Request) -> bool {
        if user.is_none() { return false };

        user.map(|x| x.get_role().map(|x| x == self.0).unwrap_or(false)).unwrap_or(false)
    }
}

