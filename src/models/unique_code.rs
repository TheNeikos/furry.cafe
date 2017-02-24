use rand::{thread_rng, Rng};
use diesel;
use diesel::ExpressionMethods;

use models::schema::unique_codes;
use models::user::{User, self};
use database;
use models;
use error;

#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum UniqueCodeType {
    PasswordReset,
}

impl UniqueCodeType {
    pub fn from_i32(i: i32) -> UniqueCodeType {
        match i {
            0 => UniqueCodeType::PasswordReset,
            _ => panic!("tried to use out of bound image type")
        }
    }
}


#[derive(Queryable, Identifiable, Debug, Associations)]
#[belongs_to(User)]
pub struct UniqueCode {
    pub id: i64,
    pub user_id: Option<i64>,
    pub created_at: diesel::data_types::PgTimestamp,
    pub updated_at: diesel::data_types::PgTimestamp,
    pub code: String,
    pub typ: i32,
}

impl UniqueCode {
    pub fn create_from(nup: NewUniqueCode) -> Result<i64, error::FurryError> {
        use diesel;
        use diesel::prelude::*;
        use models::schema::unique_codes::dsl::*;
        diesel::insert(&nup).into(unique_codes).returning(id)
            .get_result(&*database::connection().get().unwrap()).map_err(|e| e.into())
    }

    pub fn update(&self, update: &UpdateUniqueCode) -> Result<usize, error::FurryError> {
        use diesel;
        use diesel::prelude::*;
        use models::schema::unique_codes::dsl::*;
        diesel::update(unique_codes.filter(id.eq(self.id))).set(update)
            .execute(&*database::connection().get().unwrap()).map_err(|e| e.into())
    }

    pub fn get_user(&self) -> Result<Option<User>, error::FurryError> {
        match self.user_id {
            Some(id) => user::find(id),
            None => Ok(None)
        }
    }
}

#[derive(Clone, Debug)]
#[derive(Insertable)]
#[table_name="unique_codes"]
pub struct NewUniqueCode {
    pub code: String,
    pub typ: i32,
}

impl NewUniqueCode {
    pub fn new(typ: UniqueCodeType) -> NewUniqueCode {
        let mut rng = thread_rng();
        let code = rng.gen_ascii_chars().take(50).collect::<String>();

        NewUniqueCode {
            code: code,
            typ: typ as i32,
        }
    }
}

#[derive(Clone, Debug)]
#[derive(AsChangeset)]
#[table_name="unique_codes"]
pub struct UpdateUniqueCode {
    pub id: i64,
    pub user_id: i64,
}

impl UpdateUniqueCode {
    pub fn create_for(inv: &UniqueCode, id: i64) -> UpdateUniqueCode {
        UpdateUniqueCode {
            id: inv.id,
            user_id: id,
        }
    }
}

pub fn find_by_id(uid: i64) -> Result<UniqueCode, error::FurryError> {
    use diesel::prelude::*;
    use models::schema::unique_codes::dsl::*;

    unique_codes.limit(1).filter(id.eq(uid))
         .get_result::<models::unique_code::UniqueCode>(&*database::connection().get().unwrap()).map_err(|e| e.into())
}

pub fn find_by_user_id(uid: i64) -> Result<Option<UniqueCode>, error::FurryError> {
    use diesel::prelude::*;
    use models::schema::unique_codes::dsl::*;

    unique_codes.limit(1).filter(user_id.eq(uid)).order(created_at.desc())
         .get_result::<models::unique_code::UniqueCode>(&*database::connection().get().unwrap()).optional().map_err(|e| e.into())
}

pub fn find_by_code(c: &str, t: UniqueCodeType) -> Result<Option<UniqueCode>, error::FurryError> {
    use diesel::prelude::*;
    use models::schema::unique_codes::dsl::*;

    unique_codes.limit(1).filter(code.eq(c).and(typ.eq(t as i32)))
         .get_result::<models::unique_code::UniqueCode>(&*database::connection().get().unwrap()).optional().map_err(|e| e.into())
}

pub fn find_by_code_free(c: &str, t: UniqueCodeType) -> Result<Option<UniqueCode>, error::FurryError> {
    use diesel::prelude::*;
    use models::schema::unique_codes::dsl::*;

    let null : Option<i64> = None;
    unique_codes.limit(1).filter(code.eq(c)).filter(user_id.eq(null)).filter(typ.eq(t as i32))
         .get_result::<models::unique_code::UniqueCode>(&*database::connection().get().unwrap()).optional().map_err(|e| e.into())
}

pub fn all() -> Result<Vec<UniqueCode>, error::FurryError> {
    use diesel::prelude::*;
    use models::schema::unique_codes::dsl::*;

    unique_codes.order(created_at.desc())
         .get_results::<models::unique_code::UniqueCode>(&*database::connection().get().unwrap()).map_err(|e| e.into())
}

