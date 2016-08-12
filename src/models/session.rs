use diesel;

use models::schema::sessions;
use models::user::User;
use database;
use models;
use error;

#[derive(Queryable, Identifiable)]
#[belongs_to(User)]
pub struct Session {
    pub id: i64,
    pub user_id: i64,
    pub created_at: diesel::data_types::PgTimestamp,
    pub updated_at: diesel::data_types::PgTimestamp,
}

#[insertable_into(sessions)]
pub struct NewSession {
    pub user_id: i64,
}

impl NewSession {
    fn new(user: &User) -> NewSession {
        NewSession {
            user_id: user.id,
        }
    }
}

pub fn find(uid: i64) -> Result<Option<Session>, error::DatabaseError> {
    use diesel::prelude::*;
    use models::schema::sessions::dsl::*;

    sessions.limit(1).filter(id.eq(uid))
         .get_result::<models::session::Session>(&*database::connection().get().unwrap()).optional().map_err(|e| e.into())
}
