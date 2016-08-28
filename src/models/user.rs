use std::str::FromStr;

use bcrypt::{self, hash, DEFAULT_COST};
use diesel::{self, ExpressionMethods};
use iron_login;
use iron::Request;
use params::File;
use image;

use models::schema::users;
//use models::session;
use database;
use models;
use error;
use models::user_role::{self, Role, UserRole, NewUserRole};
use models::user_profile::{self, UserProfile, NewUserProfile};
use models::image::{Image, NewImage};

#[derive(Queryable, Identifiable, Debug)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub created_at: diesel::data_types::PgTimestamp,
    pub updated_at: diesel::data_types::PgTimestamp,
    profile_image: Option<i64>
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

        if let Ok(Some(_)) = find_by_email(email) {
            ue.push("E-Mail is already in use")
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

    pub fn create_from(nu: NewUser) -> Result<(), error::DatabaseError> {
        use diesel;
        use diesel::prelude::*;
        use models::schema::users::dsl::*;
        let user_id = try!(database_try!(diesel::insert(&nu)
                                    .into(users)
                                    .returning(id)
                                    .get_result(&*database::connection().get().unwrap())
                                    ));
        let user = try!(find(user_id)).unwrap();
        user.set_role(Role::Member)
    }

    pub fn set_role(&self, role: Role) -> Result<(), error::DatabaseError> {
        match try!(user_role::find_by_user_id(self.id)) {
            Some(mut x) => {
                if x.role == role as i32 { return Ok(()); }

                x.update(role)
            }
            None => {
                UserRole::create_from(NewUserRole::new(self, role))
            }
        }
    }

    pub fn get_role(&self) -> Result<Role, error::DatabaseError> {
        match try!(user_role::find_by_user_id(self.id)) {
            Some(x) => Ok(Role::from(x.role)),
            None => {
                error!("Could not find role for user, adding...: {}", self.id);
                try!(self.set_role(Role::Member));
                return Ok(Role::Member);
            }
        }
    }

    pub fn set_profile(&self, profile: NewUserProfile) -> Result<(), error::DatabaseError> {
        UserProfile::create_from(profile)
    }

    pub fn get_profile(&self) -> Result<UserProfile, error::DatabaseError> {
        match try!(user_profile::find_by_user_id(self.id)) {
            Some(x) => Ok(x),
            None => {
                error!("Could not find profile for user, adding...: {}", self.id);
                try!(self.set_profile(NewUserProfile {
                    user_id: self.id,
                    bio: "",
                }));
                return self.get_profile(); // TODO: Is this safe??
            }
        }
    }

    pub fn get_avatar(&self) -> Result<Option<Image>, error::DatabaseError> {
        match self.profile_image {
            Some(id) => models::image::find(id),
            None => Ok(None)
        }
    }
}

impl iron_login::User for User {
    fn from_user_id(_req: &mut Request, user_id: &str) -> Option<User> {
        let id = match i64::from_str(user_id) {
            Ok(i) => i,
            Err(_) => return None,
        };


        let user = match find(id) {
            Ok(Some(u)) => Some(u),
            Ok(None) => return None,
            Err(e) => {
                error!("Could not find user: {}", e);
                return None;
            }
        };

        return user;
    }

    fn get_user_id(&self) -> String {
        self.id.to_string()
    }
}

#[changeset_for(users)]
pub struct UpdateUser<'a> {
    name: Option<&'a str>,
    password_hash: Option<String>,
    profile_image: Option<i64>
}

impl<'a> UpdateUser<'a> {
    pub fn new<'b>(name: Option<&'b str>, mut password: Option<&'b str>, file: Option<&File>)
        -> Result<UpdateUser<'b>, UserError>
{
        let mut ue = UserError::new();

        if let Some(name) = name {
            ue.name.append(&mut User::verify_name(name));
        }

        if let Some(pass) = password {
            if pass.is_empty() {
                password = None;
            }
        }

        let mut to_be_converted = None;

        if let Some(f) = file {
            if f.size() > 300 * 1024 { // 300 Kilobytes
                ue.profile_image.push("Image is too big (limit is 300KiB)");
            }

            if let Ok(mut f) = f.open() {
                use std::io::Read;
                let mut buffer = Vec::new();
                if f.read_to_end(&mut buffer).is_err() {
                    ue.profile_image.push("Image is not in a valid format");

                } else {
                    to_be_converted = match image::load_from_memory(&buffer) {
                        Ok(t) => {
                            Some(t)
                        }
                        Err(e) => {
                            info!("Could not load image: {}", e);
                            ue.profile_image.push("Image is not in a valid format");
                            None
                       }
                    }
                }
            } else {
                ue.profile_image.push("Could not use this image")
            }
        }

        if ue.has_any_errors() {
            return Err(ue);
        }

        let password_hash = password.map(|password| {
            hash(password, DEFAULT_COST).expect("Could not hash password!")
        });

        let img = to_be_converted.and_then(|img| {
            use image::FilterType;
            let img = img.resize(350, 350, FilterType::CatmullRom);

            let new_image = match NewImage::create_from_dynamic_image(&img, "avatar") {
                Ok(t) => t,
                Err(e) => {
                    error!("Could not create from dynamic image {}", e);
                    return None;
                }
            };

            match Image::create_from(new_image) {
                Ok(t) => Some(t),
                Err(e) => {
                    error!("Could not save image {}", e);
                    return None;
                }
            }
        });

        Ok(UpdateUser {
            name: name,
            password_hash: password_hash,
            profile_image: img,
        })
    }
}

#[insertable_into(users)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password_hash: String,
    pub name: &'a str,
}

impl<'a> NewUser<'a> {
    pub fn new<'b>(name: Option<&'b str>, email: Option<&'b str>, password: Option<&'b str>)
        -> Result<NewUser<'b>, (UserError, NewUser<'b>)>
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

        let mut nu = NewUser {
            email: email.unwrap(),
            name: name.unwrap(),
            password_hash: String::new(),
        };

        if ue.has_any_errors() {
            return Err((ue, nu));
        }

        let password_hash = hash(password.unwrap(), DEFAULT_COST).expect("Could not hash password!");

        nu.password_hash = password_hash;

        Ok(nu)
    }
}

#[derive(Debug)]
pub struct UserError {
    pub email: Vec<&'static str>,
    pub password: Vec<&'static str>,
    pub name: Vec<&'static str>,
    pub profile_image: Vec<&'static str>,
}

impl UserError {
    pub fn new() -> UserError {
        UserError { email: vec![], password: vec![], name: vec![], profile_image: vec![] }
    }
    fn has_any_errors(&self) -> bool {
        !(self.email.is_empty()
          && self.name.is_empty()
          && self.password.is_empty()
          && self.profile_image.is_empty())
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

pub fn find_by_email(email_addr: &str) -> Result<Option<User>, error::DatabaseError> {
    use diesel::prelude::*;
    use models::schema::users::dsl::*;

    users.limit(1).filter(email.eq(email_addr))
         .get_result::<models::user::User>(&*database::connection().get().unwrap()).optional().map_err(|e| e.into())
}

pub fn with_email_password(email: &str, password: &str) -> Result<Option<User>, error::LoginError> {
    let user = try!(find_by_email(email));

    if let None = user {
        return Ok(None);
    }

    let user = user.unwrap();

    let correct = try!(bcrypt::verify(password, &user.password_hash));

    if correct {
        Ok(Some(user))
    } else  {
        Ok(None)
    }
}


