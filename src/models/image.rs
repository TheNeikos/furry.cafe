use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;

use diesel;
use image::{DynamicImage, GenericImage, ImageFormat};

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
    pub fn create_from(new: NewImage) -> Result<i64, error::DatabaseError> {
        use diesel;
        use diesel::prelude::*;
        use models::schema::images::dsl::*;
        diesel::insert(&new).into(images)
            .returning(id).get_result(&*database::connection().get().unwrap()).map_err(|e| e.into())
    }

    pub fn get_path(&self) -> String {
        format!("{}", self.path)
    }
}

#[derive(Clone, Debug)]
#[insertable_into(images)]
pub struct NewImage {
    host_type: i32,
    path: String,
}

impl NewImage {
    pub fn new(typ: ImageType, path: &str) -> NewImage {
        NewImage {
            host_type: typ as i32,
            path: path.to_string(),
        }
    }

    // TODO: Better Error handling
    pub fn create_from_dynamic_image(img: &DynamicImage, suffix: &str) -> Result<NewImage, ::std::io::Error> {
        let dims = img.dimensions();
        let path = format!("./assets/uploads/{}_{}-{}-{}.png", dims.0, dims.1, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(), suffix);
        let mut file = try!(File::create(&path));
        if let Err(e) = img.save(&mut file, ImageFormat::PNG) {
            error!("Could not save image... {}", e);
        };
        Ok(NewImage {
            path: String::from(&path[1..]),
            host_type: ImageType::Local as i32,
        })
    }
}

pub fn find(uid: i64) -> Result<Option<Image>, error::DatabaseError> {
    use diesel::prelude::*;
    use models::schema::images::dsl::*;

    images.limit(1).filter(id.eq(uid))
         .get_result::<models::image::Image>(&*database::connection().get().unwrap()).optional().map_err(|e| e.into())
}

