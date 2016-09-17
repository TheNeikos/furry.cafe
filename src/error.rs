#![allow(dead_code)]

use iron::prelude::*;
use iron::status;
use bcrypt::BcryptError;

use diesel;
use std::error::Error;
use std::fmt::{self, Debug};
use models::user::User;

quick_error! {
    #[derive(Debug)]
    pub enum FurratoriaError {
        NotImplemented(err: String) {
            description(err)
        }
        Template(err: Box<Error + Send>) {
            cause(&**err)
            description(err.description())
            from()
        }
        Database(err: Box<Error + Send>) {
            cause(&**err)
            description(err.description())
            from(diesel::result::Error)
        }
        BadFormatting(err: Box<Error + Send>) {
            cause(&**err)
            description(err.description())
            from(diesel::result::Error)
        }
        Login(err: Box<Error + Send>) {
            cause(&**err)
            description(err.description())
            from(diesel::result::Error)
            from(FurratoriaError::Internal)
            from(FurratoriaError::Database)
        }
        Internal(err: Box<Error + Send>) {
            cause(&**err)
            description(err.description())
            from(BcryptError)
        }
        Unauthorized(user: User) {
            cause(&**err)
            description(err.description())
        }
        NotFound {}
    }
}

impl From<FurratoriaError> for IronError {
    fn from(e: FurratoriaError) -> IronError {
        IronError::new(Box::new(e), status::InternalServerError)
    }
}
