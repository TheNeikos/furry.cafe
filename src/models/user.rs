use bcrypt::{hash, DEFAULT_COST};
use diesel::ExpressionMethods;

use models::schema::users;
use database;
use models;
use error;

#[derive(Queryable)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password_hash: String,
    pub name: String,
}

impl User {
    fn verify_name(name: &str) -> Vec<&'static str> {
        let mut ue = vec![];
        if name.is_empty() {
            ue.push("Username cannot be empty.");
        }

        // FIXME: This should check the graphemes instead of length...
        if name.chars().count() > 20 {
            ue.push("Username should be less than 20 characters")
        }
        return ue;
    }

    fn verify_email(email: &str) -> Vec<&'static str> {
        let mut ue = vec![];
        if email.is_empty() {
            ue.push("Email cannot be empty");
        }
        if email.find('@').is_none() {
            ue.push("A valid Email contains an @");
        }
        return ue;
    }

    pub fn update(&self, update: &UpdateUser) -> Result<usize, error::DatabaseError> {
        use diesel;
        use diesel::prelude::*;
        use models::schema::users::dsl::*;
        diesel::update(users.filter(id.eq(self.id))).set(update)
            .execute(&*database::connection().get().unwrap()).map_err(|e| e.into())
    }

    pub fn delete(self) -> Result<usize, error::DatabaseError> {
        use diesel;
        use diesel::prelude::*;
        use models::schema::users::dsl::*;
        diesel::delete(users.filter(id.eq(self.id)))
            .execute(&*database::connection().get().unwrap()).map_err(|e| e.into())
    }
}

#[changeset_for(users)]
pub struct UpdateUser<'a> {
    name: Option<&'a str>,
    password_hash: Option<String>,
}

impl<'a> UpdateUser<'a> {
    pub fn new<'b>(name: Option<&'b str>, mut password: Option<&'b str>) -> Result<UpdateUser<'b>, UserError> {
        let mut ue = UserError::new();

        if let Some(name) = name {
            ue.name.append(&mut User::verify_name(name));
        }

        if let Some(pass) = password {
            if pass.is_empty() {
                password = None;
            }
        }

        if ue.has_any_errors() {
            return Err(ue);
        }


        let password_hash = password.map(|password| {
            hash(password, DEFAULT_COST).expect("Could not hash password!")
        });

        Ok(UpdateUser {
            name: name,
            password_hash: password_hash,
        })
    }
}

#[insertable_into(users)]
pub struct NewUser<'a> {
    email: &'a str,
    password_hash: String,
    name: &'a str,
}

impl<'a> NewUser<'a> {
    pub fn new<'b>(name: Option<&'b str>, email: Option<&'b str>, password: Option<&'b str>)
        -> Result<NewUser<'b>, UserError>
    {
        let mut ue = UserError::new();

        if let Some(name) = name {
            ue.name.append(&mut User::verify_name(name));
        } else {
            ue.name.push("Username cannot be empty.");
        }

        if let Some(email) = email {
            ue.email.append(&mut User::verify_email(email));
        } else {
            ue.email.push("Email cannot be empty.");
        }

        if let Some(password) = password {
            if password.is_empty() {
                ue.password.push("Password cannot be empty");
            }
        } else {
            ue.password.push("Password cannot be empty.");
        }

        if ue.has_any_errors() {
            return Err(ue);
        }

        let password_hash = hash(password.unwrap(), DEFAULT_COST).expect("Could not hash password!");

        Ok(NewUser {
            email: email.unwrap(),
            name: name.unwrap(),
            password_hash: password_hash,
        })
    }
}

#[derive(Debug)]
pub struct UserError {
    pub email: Vec<&'static str>,
    pub password: Vec<&'static str>,
    pub name: Vec<&'static str>,
}

impl UserError {
    pub fn new() -> UserError {
        UserError { email: vec![], password: vec![], name: vec![] }
    }
    fn has_any_errors(&self) -> bool {
        !(self.email.is_empty() && self.name.is_empty() && self.password.is_empty())
    }
}


pub fn find_all() -> Result<Vec<User>, error::DatabaseError> {
    use diesel::prelude::*;
    use models::schema::users::dsl::*;

    users.get_results::<models::user::User>(&*database::connection().get().unwrap()).map_err(|e| e.into())
}

pub fn find(uid: i64) -> Result<Option<User>, error::DatabaseError> {
    use diesel::prelude::*;
    use models::schema::users::dsl::*;

    users.limit(1).filter(id.eq(uid))
         .get_result::<models::user::User>(&*database::connection().get().unwrap()).optional().map_err(|e| e.into())
}


