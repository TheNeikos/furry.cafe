use iron::prelude::*;
use iron::status::Status;
use iron_login::User as U;
use iron::BeforeMiddleware;

use error::FurratoriaError;
use models::user::User;
use middleware::authorization::UserRequirement;

#[derive(Clone, Debug)]
pub struct Authorizer<T: UserRequirement + Send + Sync> {
    reqs: Vec<T>
}

impl<T: UserRequirement + Send + Sync> Authorizer<T> {
    pub fn new(r: Vec<T>) -> Authorizer<T> {
        Authorizer {
            reqs: r
        }
    }
}

impl<T: UserRequirement + Send + Sync> Authorizer<T> {
    pub fn do_check(&self, req: &mut Request, user: Option<&User>) -> bool {
        self.reqs.iter().map(|x| x.check(user, req)).all(|x| x)
    }
}

impl<T: UserRequirement + Send + Sync + 'static> BeforeMiddleware for Authorizer<T> {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let user = User::get_login(req).get_user();

        let results = self.do_check(req, user.as_ref());
        if results {
            Ok(())
        } else {
            Err(IronError::new(FurratoriaError::Unauthorized(user), Status::Unauthorized))
        }
    }
}


