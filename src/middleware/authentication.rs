use iron::prelude::*;
use iron::BeforeMiddleware;
use iron::status::Status;
use iron_login::User as U;

use error::UnauthorizedError;
use models::user::{self, User};

pub struct Authorizer<T: Send + Sync> {
    reqs: Vec<T>
}

impl<T: UserRequirement + Send + Sync + 'static> BeforeMiddleware for Authorizer<T> {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let user = User::get_login(req).get_user();

        let results = self.reqs.iter().map(|x| x.check(user.as_ref(), req)).all(|x| x);

        if results {
            Ok(())
        } else {
            Err(IronError::new(UnauthorizedError::new(user), Status::Unauthorized))
        }
    }
}

pub trait UserRequirement {
    fn check(&self, user: Option<&User>, req: &mut Request) -> bool;
}

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
                Err(_) => return false,
            }
        };

        other_user.map(|x| x.id == user.id).unwrap_or(false)
    }
}
