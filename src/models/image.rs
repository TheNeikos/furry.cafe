use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;

use diesel;
use image::{DynamicImage, GenericImage, ImageFormat, self};

use models::schema::images;
use database;
use models;
use error;

#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum ImageType {
    Local
}

impl ImageType {
    pub fn from_i32(i: i32) -> ImageType {
        match i {
            0 => ImageType::Local,
            _ => panic!("tried to use out of bound image type")
        }
    }
}

#[derive(Queryable, Identifiable, Clone)]
pub struct Image {
    pub id: i64,
    pub created_at: diesel::data_types::PgTimestamp,
    pub updated_at: diesel::data_types::PgTimestamp,
    host_type: i32,
    path: String,
    pub width: i32,
    pub height: i32,
    pub parent_id: Option<i64>,
}

impl Image {
    pub fn create_from(new: NewImage) -> Result<i64, error::FurratoriaError> {
        use diesel;
        use diesel::prelude::*;
        use models::schema::images::dsl::*;
        diesel::insert(&new).into(images)
            .returning(id).get_result(&*database::connection().get().unwrap()).map_err(|e| e.into())
    }

    pub fn get_path(&self) -> String {
        format!("{}", self.path)
    }

    pub fn get_with_size(&self, width: i32, height: i32) -> Result<Option<Image>, error::FurratoriaError> {
        if self.width > width || self.height > height {
            match find_from_image(self.id, width, height) {
                Ok(Some(i)) => Ok(Some(i)),
                Ok(None) => {
                    let new_image = try!(NewImage::create_from_image_with_size(self, width, height));
                    let img_id = try!(Image::create_from(new_image));
                    find(img_id)
                }
                Err(e) => Err(e),
            }
        } else {
            Ok(Some(self.clone()))
        }
    }
}

#[derive(Clone, Debug)]
#[insertable_into(images)]
pub struct NewImage {
    host_type: i32,
    path: String,
    width: i32,
    height: i32,
    parent_id: Option<i64>
}

impl NewImage {
    pub fn new(typ: ImageType, path: &str) -> NewImage {
        NewImage {
            host_type: typ as i32,
            path: path.to_string(),
            width: 0,
            height: 0,
            parent_id: None,
        }
    }

    pub fn create_from_image_with_size(img: &Image, width: i32, height: i32) -> Result<NewImage, error::FurratoriaError> {
        let image = {
            match ImageType::from_i32(img.host_type) {
                ImageType::Local => {
                    try!(image::open(&format!(".{}", img.get_path())[..]))
                }
            }
        };

        let mut image = try!(NewImage::create_from_dynamic_image(&image.resize(width as u32, height as u32, image::FilterType::Lanczos3), &format!("orig_{}", img.id)[..]));
        image.parent_id = Some(img.id);
        Ok(image)
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
            width: dims.0 as i32,
            height: dims.1 as i32,
            parent_id: None,
        })
    }
}

pub fn find(uid: i64) -> Result<Option<Image>, error::FurratoriaError> {
    use diesel::prelude::*;
    use models::schema::images::dsl::*;

    images.limit(1).filter(id.eq(uid))
         .get_result::<models::image::Image>(&*database::connection().get().unwrap()).optional().map_err(|e| e.into())
}

pub fn find_from_image(uid: i64, w: i32, h: i32) -> Result<Option<Image>, error::FurratoriaError> {
    use diesel::prelude::*;
    use models::schema::images::dsl::*;

    images.limit(1).filter(parent_id.eq(uid)).filter(width.le(w).and(height.le(h)))
         .get_result::<models::image::Image>(&*database::connection().get().unwrap()).optional().map_err(|e| e.into())
}

