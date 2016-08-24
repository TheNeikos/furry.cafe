use diesel;

use models::schema::submissions;
use models::user::User;
use models::image::Image;
use database;
use models;
use error;

#[derive(Queryable, Identifiable)]
#[belongs_to(User)]
pub struct Submission {
    pub id: i64,
    pub user_id: i64,
    pub created_at: diesel::data_types::PgTimestamp,
    pub updated_at: diesel::data_types::PgTimestamp,
    image: i64,
    pub title: String,
    pub description: String,
}

impl Submission {
    pub fn create_from(nup: NewSubmission) -> Result<(), error::DatabaseError> {
        use diesel;
        use diesel::prelude::*;
        use models::schema::submissions::dsl::*;
        diesel::insert(&nup)
            .into(submissions).execute(&*database::connection().get().unwrap()).map_err(|e| e.into()).map(|_| ())
    }
}

#[derive(Clone, Debug)]
#[insertable_into(submissions)]
pub struct NewSubmission<'a, 'b> {
    pub user_id: i64,
    pub title: &'a str,
    pub description: &'b str,
    image: i64,
}

impl<'a, 'b> NewSubmission<'a, 'b> {
    pub fn new(user: &User, image: &Image, title: &'a str, desc: &'b str) -> NewSubmission<'a, 'b> {
        NewSubmission {
            user_id: user.id,
            title: title,
            description: desc,
            image: image.id,
        }
    }
}

pub fn find_by_user_id(uid: i64) -> Result<Option<Submission>, error::DatabaseError> {
    use diesel::prelude::*;
    use models::schema::submissions::dsl::*;

    submissions.limit(1).filter(user_id.eq(uid)).order(created_at.desc())
         .get_result::<models::submission::Submission>(&*database::connection().get().unwrap()).optional().map_err(|e| e.into())
}
