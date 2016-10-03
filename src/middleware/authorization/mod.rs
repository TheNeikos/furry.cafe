use iron::Request;
use iron_login::User as U;

use models::user::User;

mod authorizer;
mod same_user_auth;
mod same_user_auth_as;
mod logged_in;
mod has_role;
mod is_owner;

pub trait UserRequirement {
    fn check(&self, user: Option<&User>, req: &mut Request) -> bool;
}

pub trait UserAuthorization {
    fn current_user_can<T: UserRequirement + Send + Sync>(&mut self, auth: T) -> bool;
}

pub use self::authorizer::Authorizer;
pub use self::same_user_auth::SameUserAuth;
pub use self::same_user_auth_as::SameUserAuthAs;
pub use self::logged_in::LoggedIn;
pub use self::has_role::HasRole;
pub use self::is_owner::IsOwner;

impl<'a, 'b> UserAuthorization for Request<'a, 'b> {
    fn current_user_can<T: UserRequirement + Send + Sync>(&mut self, auth: T) -> bool {
        let user = User::get_login(self).get_user();
        Authorizer::new(vec![auth]).do_check(self, user.as_ref())
    }
}
