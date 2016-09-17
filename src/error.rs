#![allow(dead_code)]

use iron::prelude::*;
use iron::status;
use bcrypt::BcryptError;

use diesel;
use std::error::Error;
use models::user::User;
use std::fmt;

quick_error! {
    #[derive(Debug)]
    pub enum FurratoriaError {
        NotImplemented(err: String) {
            description(err)
        }
        Template(err: Box<Error + Send>) {
            cause(&**err)
            description(err.description())
            from(e: fmt::Error) -> (Box::new(e))
        }
        Database(err: Box<Error + Send>) {
            cause(&**err)
            description(err.description())
            from(e: diesel::result::Error) -> (Box::new(e))
        }
        Login(err: Box<Error + Send>) {
            cause(&**err)
            description(err.description())
            //from(e: FurratoriaError) -> (Box::new(e))
        }
        Bcrypt(err: BcryptError) {
            from()
        }
        Internal(err: Box<Error + Send>) {
            cause(&**err)
            description(err.description())
        }
        Unauthorized(user: Option<User>) {}
        NotFound {}
        BadFormatting { }
    }
}

impl From<FurratoriaError> for IronError {
    fn from(e: FurratoriaError) -> IronError {
        IronError::new(Box::new(e), status::InternalServerError)
    }
}
