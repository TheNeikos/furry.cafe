use iron::prelude::*;
use iron::BeforeMiddleware;
use iron::status::Status;
use iron_login::User as U;

use error::UnauthorizedError;
use models::user::User;

pub struct Authorizer<T: Send + Sync> {
    reqs: Vec<T>
}

impl<T: UserRequirement + Send + Sync + 'static> BeforeMiddleware for Authorizer<T> {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let user = User::get_login(req).get_user();

        let results = self.reqs.iter().map(|x| x.check(user.as_ref())).all(|x| x);

        if results {
            Ok(())
        } else {
            Err(IronError::new(UnauthorizedError::new(user), Status::Unauthorized))
        }
    }
}

pub trait UserRequirement {
    fn check(&self, user: Option<&User>) -> bool;
}
