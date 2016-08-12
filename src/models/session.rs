
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
