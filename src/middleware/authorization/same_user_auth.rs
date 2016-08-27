use iron::prelude::*;

use models::user::{self, User};
use middleware::authorization::UserRequirement;

#[derive(Clone, Debug)]
pub struct SameUserAuth;

impl UserRequirement for SameUserAuth {
    fn check(&self, user: Option<&User>, req: &mut Request) -> bool {
        if user.is_none() { return false };
        let user = user.unwrap();

        // FIXME: Don't do DB stuff in here...
        let other_user = {
            use router::Router;
            let id = match req.extensions.get::<Router>().unwrap().find("id") {
                    Some(t) => match t.parse::<_>() {
                                Ok(t) => t,
                                Err(_) => return true
                            },
                    None => return true, // For now we just pass through...
            };
            match user::find(id) {
                Ok(t) => t,
                Err(e) => {
                    error!("Could not find user: {}", e);
                    return false;
                }
            }
        };

        other_user.map(|x| x.id == user.id).unwrap_or(false)
    }
}

