use iron::prelude::*;
use iron::status::Status;
use iron_login::User as U;
use iron::BeforeMiddleware;

use error::UnauthorizedError;
use models::user::User;
use middleware::authorization::UserRequirement;

#[derive(Clone, Debug)]
pub struct Authorizer<T: Send + Sync> {
    reqs: Vec<T>
}

impl<T: Send + Sync> Authorizer<T> {
    pub fn new(r: Vec<T>) -> Authorizer<T> {
        Authorizer {
            reqs: r
        }
    }
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

