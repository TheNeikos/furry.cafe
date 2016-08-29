use params::File;

use diesel::{self, ExpressionMethods};
use image::{self, GenericImage, DynamicImage};

use models::schema::submissions;
use models::user::User;
use models::image::{Image, NewImage};
use database;
use models;
use error;

fn convert_image(mut img: DynamicImage) -> Option<i64> {
    use image::FilterType;
    match img.dimensions() {
        (x, y) if x > 3000 || y > 3000 => {
            img = img.resize(3000, 3000, FilterType::CatmullRom);
        }
        _ => (),
    }

    let new_image = match NewImage::create_from_dynamic_image(&img, "submission") {
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
}

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
    pub fn create_from(nup: NewSubmission) -> Result<i64, error::DatabaseError> {
        use diesel;
        use diesel::prelude::*;
        use models::schema::submissions::dsl::*;
        diesel::insert(&nup).into(submissions)
            .returning(id).get_result(&*database::connection().get().unwrap()).map_err(|e| e.into())
    }

    pub fn update(&self, update: &UpdateSubmission) -> Result<usize, error::DatabaseError> {
        use diesel;
        use diesel::prelude::*;
        use models::schema::submissions::dsl::*;
        diesel::update(submissions.filter(id.eq(self.id))).set(update)
            .execute(&*database::connection().get().unwrap()).map_err(|e| e.into())
    }

    pub fn delete(self) -> Result<usize, error::DatabaseError> {
        use diesel;
        use diesel::prelude::*;
        use models::schema::submissions::dsl::*;
        diesel::delete(submissions.filter(id.eq(self.id)))
            .execute(&*database::connection().get().unwrap()).map_err(|e| e.into())
    }

    pub fn get_image(&self) -> Result<Option<Image>, error::DatabaseError> {
        models::image::find(self.image)
    }

    pub fn get_submitter(&self) -> Result<Option<User>, error::DatabaseError> {
        models::user::find(self.user_id)
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
    pub fn new(user: &User, image: Option<&File>, title: Option<&'a str>, desc: Option<&'b str>)
        -> Result<NewSubmission<'a, 'b>, (SubmissionError, NewSubmission<'a, 'b>)>
    {
        let mut se = SubmissionError::new();

        if let Some(title) = title {
            if title.chars().count() > 50 {
                se.title.push("Title cannot have more than 50 characters");
            }

            if title.chars().count() == 0 {
                se.title.push("Title cannot be empty");
            }
        } else {
            se.title.push("Title cannot be empty");
        }

        if let Some(desc) = desc {
            if desc.chars().count() > 150_000 {
                se.description.push("Description cannot be longer than 150 000 characters");
            }
        }

        let mut to_be_converted = None;

        if let Some(image) = image {
            if image.size() > 2 * 1024 * 1024 { // 2 Megabytes
                se.image.push("Image is too big (limit is 2MiB)");
            }

            if let Ok(mut f) = image.open() {
                use std::io::Read;
                let mut buffer = Vec::new();
                if f.read_to_end(&mut buffer).is_err() {
                    se.image.push("Image is not in a valid format");
                } else {
                    to_be_converted = match image::load_from_memory(&buffer) {
                        Ok(t) => {
                            Some(t)
                        }
                        Err(e) => {
                            info!("Could not load image {}", e);
                            se.image.push("Image is not in a valid format");
                            None
                        }
                    }
                }
            } else {
                se.image.push("Could not use this image, please try again")
            }
        };

        let image = to_be_converted.and_then(convert_image);

        if image.is_none() {
            se.image.push("Could not convert image");
        }

        let ns = NewSubmission {
            user_id: 0,
            title: title.unwrap_or(&""),
            description: desc.unwrap_or(&""),
            image: 0,
        };

        if se.has_any_errors() {
            return Err((se, ns));
        }

        let image = image.unwrap();

        Ok(NewSubmission {
            user_id: user.id,
            title: title.unwrap(),
            description: desc.unwrap_or(&""),
            image: image,
        })
    }
}

#[changeset_for(submissions)]
pub struct UpdateSubmission<'a> {
    title: Option<&'a str>,
    description: Option<&'a str>,
    image: Option<i64>
}

impl<'a> UpdateSubmission<'a> {
    pub fn new(mut image: Option<&File>, title: Option<&'a str>, desc: Option<&'a str>)
        -> Result<UpdateSubmission<'a>, SubmissionError>
    {
        let mut se = SubmissionError::new();

        if let Some(title) = title {
            if title.chars().count() > 50 {
                se.title.push("Title cannot have more than 50 characters");
            }

            if title.chars().count() == 0 {
                se.title.push("Title cannot be empty");
            }
        } else {
            se.title.push("Title cannot be empty");
        }

        if let Some(desc) = desc {
            if desc.chars().count() > 150_000 {
                se.description.push("Description cannot be longer than 150 000 characters");
            }
        }

        let mut to_be_converted = None;

        if image.is_some() && image.as_ref().map(|x| x.size()) == Some(0) {
            image = None;
        }

        if let Some(image) = image {
            if image.size() > 2 * 1024 * 1024 { // 2 Megabytes
                se.image.push("Image is too big (limit is 2MiB)");
            }

            if let Ok(mut f) = image.open() {
                use std::io::Read;
                let mut buffer = Vec::new();
                if f.read_to_end(&mut buffer).is_err() {
                    se.image.push("Image is not in a valid format");
                } else {
                    to_be_converted = match image::load_from_memory(&buffer) {
                        Ok(t) => {
                            Some(t)
                        }
                        Err(e) => {
                            info!("Could not load image {}", e);
                            se.image.push("Image is not in a valid format");
                            None
                        }
                    }
                }
            } else {
                se.image.push("Could not use this image, please try again")
            }
        };

        let image = to_be_converted.and_then(convert_image);

        if se.has_any_errors() {
            return Err(se);
        }

        Ok(UpdateSubmission {
            title: title,
            description: desc,
            image: image,
        })
    }
}


#[derive(Debug)]
pub struct SubmissionError {
    pub title: Vec<&'static str>,
    pub description: Vec<&'static str>,
    pub image: Vec<&'static str>,
}

impl SubmissionError {
    pub fn new() -> SubmissionError {
        SubmissionError { title: vec![], description: vec![], image: vec![]}
    }
    fn has_any_errors(&self) -> bool {
        !(self.title.is_empty()
          && self.description.is_empty()
          && self.image.is_empty())
    }
}

pub fn find(uid: i64) -> Result<Option<Submission>, error::DatabaseError> {
    use diesel::prelude::*;
    use models::schema::submissions::dsl::*;

    submissions.limit(1).filter(id.eq(uid))
         .get_result::<models::submission::Submission>(&*database::connection().get().unwrap()).optional().map_err(|e| e.into())
}

pub fn find_by_user_id(uid: i64) -> Result<Option<Submission>, error::DatabaseError> {
    use diesel::prelude::*;
    use models::schema::submissions::dsl::*;

    submissions.limit(1).filter(user_id.eq(uid)).order(created_at.desc())
         .get_result::<models::submission::Submission>(&*database::connection().get().unwrap()).optional().map_err(|e| e.into())
}

pub fn last(amt: i64) -> Result<Vec<Submission>, error::DatabaseError> {
    use diesel::prelude::*;
    use models::schema::submissions::dsl::*;

    submissions.limit(amt).order(created_at.desc())
         .get_results::<models::submission::Submission>(&*database::connection().get().unwrap()).map_err(|e| e.into())
}


