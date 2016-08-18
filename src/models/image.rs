use diesel;

use models::schema::images;
use database;
use models;
use error;

#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum ImageType {
    Local
}

#[derive(Queryable, Identifiable)]
pub struct Image {
    pub id: i64,
    pub created_at: diesel::data_types::PgTimestamp,
    pub updated_at: diesel::data_types::PgTimestamp,
    host_type: i32,
    path: String
}

impl Image {
    pub fn create_from(new: NewImage) -> Result<(), error::DatabaseError> {
        use diesel;
        use diesel::prelude::*;
        use models::schema::images::dsl::*;
        diesel::insert(&new)
            .into(images).execute(&*database::connection().get().unwrap()).map_err(|e| e.into()).map(|_| ())
    }
}

#[derive(Clone, Debug)]
#[insertable_into(images)]
pub struct NewImage<'a> {
    host_type: i32,
    path: &'a str
}

impl<'a> NewImage<'a> {
    pub fn new(typ: ImageType, path: &'a str) -> NewImage<'a> {
        NewImage {
            host_type: typ as i32,
            path: path,
        }
    }
}

pub fn find_by_id(uid: i64) -> Result<Option<Image>, error::DatabaseError> {
    use diesel::prelude::*;
    use models::schema::images::dsl::*;

    images.limit(1).filter(id.eq(uid))
         .get_result::<models::image::Image>(&*database::connection().get().unwrap()).optional().map_err(|e| e.into())
}
