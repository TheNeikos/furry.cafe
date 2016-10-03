use std::marker::PhantomData;

use iron::prelude::*;
use router::Router;

use models::HasOwner;
use models::user::User;
use middleware::authorization::UserRequirement;

#[derive(Clone, Debug)]
pub struct IsOwner<T: HasOwner + Clone> {
    _data: PhantomData<T>,
}

impl<T: HasOwner + Clone> IsOwner<T> {
    pub fn new() -> IsOwner<T> {
        IsOwner {
            _data: PhantomData,
        }
    }
}

impl<T: HasOwner + Clone> UserRequirement for IsOwner<T> {
    fn check(&self, user: Option<&User>, req: &mut Request) -> bool {
        if user.is_none() { return false };
        let id = match req.extensions.get::<Router>().unwrap().find("id") {
                Some(t) => match t.parse::<_>() {
                            Ok(t) => t,
                            Err(_) => return false
                        },
                None => return false, // For now we just pass through...
        };
        let other_user = match T::get_owner(id) {
            Ok(Some(u)) => u,
            Ok(None) => {
                error!("Could not find owner of: {}", id);
                return false;
            }
            Err(e) => {
                error!("Could not fetch owner: {}", e);
                return false;
            }
        };

        user.unwrap().id == other_user.id
    }
}

