#![allow(dead_code)]

use iron::prelude::*;
use iron::status;
use bcrypt::BcryptError;

use diesel;
use std::error::Error;
use std::fmt::{self, Debug};

#[derive(Debug)]
pub struct NotImplemented {
    route: String,
}

impl NotImplemented {
    pub fn new(req: &Request) -> NotImplemented {
        NotImplemented {
            route: req.url.clone().into_generic_url().into_string()
        }
    }
}

impl fmt::Display for NotImplemented {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for NotImplemented {
    fn description(&self) -> &str { &self.route }
}


#[derive(Debug)]
pub struct TemplateError {
    cause: Option<Box<Error + Send>>,
}

impl TemplateError {
    pub fn new() -> TemplateError {
        TemplateError {
            cause: None,
        }
    }
}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for TemplateError {
    fn description(&self) -> &str { "Template could not be parsed." }
}

impl From<fmt::Error> for TemplateError {
    fn from(other: fmt::Error) -> Self {
        TemplateError {
            cause: Some(Box::new(other)),
        }
    }
}

impl From<TemplateError> for IronError {
    fn from(e: TemplateError) -> IronError {
        IronError::new(Box::new(e), status::InternalServerError)
    }
}

#[derive(Debug)]
pub struct DatabaseError {
    cause: Option<Box<Error + Send>>,
}

impl DatabaseError {
    pub fn new() -> DatabaseError {
        DatabaseError {
            cause: None,
        }
    }
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for DatabaseError {
    fn description(&self) -> &str { "Database error." }
}

impl From<diesel::result::Error> for DatabaseError {
    fn from(other: diesel::result::Error) -> Self {
        DatabaseError {
            cause: Some(Box::new(other)),
        }
    }
}

impl From<DatabaseError> for IronError {
    fn from(e: DatabaseError) -> IronError {
        IronError::new(Box::new(e), status::InternalServerError)
    }
}

#[derive(Debug)]
pub struct BadFormattingError {
    cause: Option<Box<Error + Send>>,
}

impl BadFormattingError {
    pub fn new() -> BadFormattingError {
        BadFormattingError {
            cause: None,
        }
    }
}

impl fmt::Display for BadFormattingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for BadFormattingError {
    fn description(&self) -> &str { "BadFormatting error." }
}

impl From<diesel::result::Error> for BadFormattingError {
    fn from(other: diesel::result::Error) -> Self {
        BadFormattingError {
            cause: Some(Box::new(other)),
        }
    }
}

impl From<BadFormattingError> for IronError {
    fn from(e: BadFormattingError) -> IronError {
        IronError::new(Box::new(e), status::InternalServerError)
    }
}

#[derive(Debug)]
pub struct LoginError {
    cause: Option<Box<Error + Send>>,
}

impl LoginError {
    pub fn new() -> LoginError {
        LoginError {
            cause: None,
        }
    }
}

impl fmt::Display for LoginError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for LoginError {
    fn description(&self) -> &str { "Login error." }
}

impl From<diesel::result::Error> for LoginError {
    fn from(other: diesel::result::Error) -> Self {
        LoginError {
            cause: Some(Box::new(other)),
        }
    }
}

impl From<LoginError> for IronError {
    fn from(e: LoginError) -> IronError {
        IronError::new(Box::new(e), status::InternalServerError)
    }
}

impl From<BcryptError> for LoginError {
    fn from(_other: BcryptError) -> Self {
        LoginError {
            // FIXME: We shouldn't  just discard it...
            cause: None,
        }
    }
}

impl From<DatabaseError> for LoginError {
    fn from(other: DatabaseError) -> Self {
        LoginError {
            cause: Some(Box::new(other)),
        }
    }
}
