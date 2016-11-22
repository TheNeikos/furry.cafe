use diesel;

use models::schema::user_profiles;
use models::user::User;
use models::image::Image;
use database;
use models;
use error;

#[derive(Queryable, Identifiable)]
#[belongs_to(User)]
pub struct UserProfile {
    pub id: i64,
    pub user_id: i64,
    pub created_at: diesel::data_types::PgTimestamp,
    pub updated_at: diesel::data_types::PgTimestamp,
    pub bio: String,
    pub banner_image: Option<i64>,
}

impl UserProfile {
    pub fn create_from(nup: NewUserProfile) -> Result<(), error::FurryError> {
        use diesel;
        use diesel::prelude::*;
        use models::schema::user_profiles::dsl::*;
        diesel::insert(&nup)
            .into(user_profiles).execute(&*database::connection().get().unwrap()).map_err(|e| e.into()).map(|_| ())
    }

    pub fn get_banner(&self) -> Result<Option<Image>, error::FurryError> {
        match self.banner_image {
            Some(id) => models::image::find(id),
            None => Ok(None)
        }
    }
}

#[derive(Clone, Debug)]
#[derive(Insertable)]
#[table_name="user_profiles"]
pub struct NewUserProfile<'a> {
    pub user_id: i64,
    pub bio: &'a str,
    pub banner_image: Option<i64>
}

impl<'a> NewUserProfile<'a> {
    pub fn new(user: &User, bio: &'a str, image: Option<&Image>) -> NewUserProfile<'a> {
        NewUserProfile {
            user_id: user.id,
            bio: bio,
            banner_image: image.map(|x| x.id),
        }
    }

    pub fn from(profile: &'a UserProfile) -> NewUserProfile<'a> {
        NewUserProfile {
            user_id: profile.user_id,
            bio: &profile.bio,
            banner_image: profile.banner_image,
        }
    }
}

pub fn find_by_user_id(uid: i64) -> Result<Option<UserProfile>, error::FurryError> {
    use diesel::prelude::*;
    use models::schema::user_profiles::dsl::*;

    user_profiles.limit(1).filter(user_id.eq(uid)).order(created_at.desc())
         .get_result::<models::user_profile::UserProfile>(&*database::connection().get().unwrap()).optional().map_err(|e| e.into())
}
