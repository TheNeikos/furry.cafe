use diesel;
use diesel::ExpressionMethods;

use models::schema::user_roles;
use models::user::User;
use database;
use models;
use error;

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum Role {
    Banned, Member, Moderator, Admin
}

impl Role {
    pub fn from(u: i32) -> Role {
        match u {
            0 => Role::Banned,
            1 => Role::Member,
            2 => Role::Moderator,
            3 => Role::Admin,
            _ => panic!("Got {} for role, only 0..3 is accepted", u)
        }
    }
}

#[derive(Queryable, Identifiable)]
#[belongs_to(User)]
pub struct UserRole {
    pub id: i64,
    pub user_id: i64,
    pub role: i32,
    pub created_at: diesel::data_types::PgTimestamp,
    pub updated_at: diesel::data_types::PgTimestamp,
}

impl UserRole {
    pub fn create_from(nur: NewUserRole) -> Result<(), error::DatabaseError> {
        use diesel;
        use diesel::prelude::*;
        use models::schema::user_roles::dsl::*;
        diesel::insert(&nur)
            .into(user_roles).execute(&*database::connection().get().unwrap()).map_err(|e| e.into()).map(|_| ())
    }

    pub fn update(&mut self, role_: Role) -> Result<(), error::DatabaseError> {
        use diesel;
        use diesel::prelude::*;
        use models::schema::user_roles::dsl::*;
        diesel::update(user_roles.filter(id.eq(self.id))).set(&UpdateUserRole {
            role: role_ as i32,
        }).execute(&*database::connection().get().unwrap()).map_err(|e| e.into()).map(|_| ())
    }
}

#[insertable_into(user_roles)]
pub struct NewUserRole {
    pub user_id: i64,
    pub role: i32,
}

impl NewUserRole {
    pub fn new(user: &User, role: Role) -> NewUserRole {
        NewUserRole {
            user_id: user.id,
            role: role as i32,
        }
    }
}

#[changeset_for(user_roles)]
pub struct UpdateUserRole {
    role: i32,
}

pub fn find(uid: i64) -> Result<Option<UserRole>, error::DatabaseError> {
    use diesel::prelude::*;
    use models::schema::user_roles::dsl::*;

    user_roles.limit(1).filter(id.eq(uid))
         .get_result::<models::user_role::UserRole>(&*database::connection().get().unwrap()).optional().map_err(|e| e.into())
}

pub fn find_by_user_id(uid: i64) -> Result<Option<UserRole>, error::DatabaseError> {
    use diesel::prelude::*;
    use models::schema::user_roles::dsl::*;

    user_roles.limit(1).filter(user_id.eq(uid))
         .get_result::<models::user_role::UserRole>(&*database::connection().get().unwrap()).optional().map_err(|e| e.into())
}
