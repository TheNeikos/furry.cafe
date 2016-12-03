use params::File;

use diesel::{self, ExpressionMethods};
use image::{self, GenericImage, DynamicImage};

use models::schema::submissions;
use models::user::User;
use models::image::{Image, NewImage};
use models::filter_settings::FilterSettings;
use models::HasOwner;
use database;
use models;
use error;

fn convert_image(mut img: DynamicImage, fmt: image::ImageFormat) -> Option<i64> {
    use image::FilterType;
    match img.dimensions() {
        (x, y) if x > 3000 || y > 3000 => {
            img = img.resize(3000, 3000, FilterType::CatmullRom);
        }
        _ => (),
    }

    let new_image = match NewImage::create_from_dynamic_image(&img, "submission", fmt) {
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

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Visibility {
    Public, Friends, Private, Unpublished
}

impl Visibility {
    pub fn try_from_i32(i: i32) -> Result<Visibility, error::FurryError> {
        match i {
            0 => Ok(Visibility::Public),
            1 => Ok(Visibility::Friends),
            2 => Ok(Visibility::Private),
            3 => Ok(Visibility::Unpublished),
            _ => Err(error::FurryError::Parse(None)),
        }
    }

    pub fn from_i32(i: i32) -> Visibility {
        match i {
            0 => Visibility::Public,
            1 => Visibility::Friends,
            2 => Visibility::Private,
            3 => Visibility::Unpublished,
            _ => panic!("Could not convert {} to visibility", i),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
            Visibility::Public => "0",
            Visibility::Friends => "1",
            Visibility::Private => "2",
            Visibility::Unpublished => "3",
        }
    }

    pub fn human(&self) -> &'static str {
        match *self {
            Visibility::Public => "Public",
            Visibility::Friends => "Friends Only",
            Visibility::Private => "Private - Link Only",
            Visibility::Unpublished => "Unpublished",
        }
    }
}

#[derive(Queryable, Identifiable, Clone, Associations)]
#[belongs_to(User)]
pub struct Submission {
    pub id: i64,
    pub user_id: i64,
    pub created_at: diesel::data_types::PgTimestamp,
    pub updated_at: diesel::data_types::PgTimestamp,
    image: Option<i64>,
    pub title: String,
    pub description: String,
    pub published_at: Option<diesel::data_types::PgTimestamp>,
    visibility: i32,
}

impl Submission {
    pub fn create_from(nup: NewSubmission) -> Result<i64, error::FurryError> {
        use diesel;
        use diesel::prelude::*;
        use models::schema::submissions::dsl::*;
        diesel::insert(&nup).into(submissions)
            .returning(id).get_result(&*database::connection().get().unwrap()).map_err(|e| e.into())
    }

    pub fn update(&self, update: &UpdateSubmission) -> Result<usize, error::FurryError> {
        use diesel;
        use diesel::prelude::*;
        use models::schema::submissions::dsl::*;
        diesel::update(submissions.filter(id.eq(self.id))).set(update)
            .execute(&*database::connection().get().unwrap()).map_err(|e| e.into())
    }

    pub fn delete(self) -> Result<usize, error::FurryError> {
        use diesel;
        use diesel::prelude::*;
        use models::schema::submissions::dsl::*;
        diesel::delete(submissions.filter(id.eq(self.id)))
            .execute(&*database::connection().get().unwrap()).map_err(|e| e.into())
    }

    pub fn get_image(&self) -> Result<Option<Image>, error::FurryError> {
        match self.image {
            Some(id) => models::image::find(id),
            None => Ok(None),
        }
    }

    pub fn has_image(&self) -> bool {
        self.image.is_some()
    }

    pub fn get_submitter(&self) -> Result<User, error::FurryError> {
        match models::user::find(self.user_id) {
            Ok(None) => Err(error::FurryError::NotFound),
            Ok(Some(u)) => Ok(u),
            Err(e) => Err(e)
        }
    }

    pub fn get_visibility(&self) -> Visibility {
        Visibility::from_i32(self.visibility)
    }

    pub fn full_id(&self) -> String {
        format!("{}-{}", self.id, self.title.to_lowercase()
                .replace(|x| !char::is_alphanumeric(x) && x != ' ', "")
                .replace(char::is_whitespace, "-"))
    }
}

#[derive(Clone, Debug)]
#[derive(Insertable)]
#[table_name="submissions"]
pub struct NewSubmission<'a> {
    pub user_id: i64,
    pub title: &'a str,
    pub description: &'a str,
    image: Option<i64>,
    visibility: i32,
}

impl<'a> NewSubmission<'a> {
    pub fn new(user: &User) -> Result<NewSubmission, SubmissionError> {
        Ok(NewSubmission {
            user_id: user.id,
            title: "",
            description: "",
            image: None,
            visibility: Visibility::Unpublished as i32
        })
    }

    pub fn get_visibility(&self) -> Visibility {
        Visibility::from_i32(self.visibility)
    }
}

#[derive(AsChangeset)]
#[table_name="submissions"]
pub struct UpdateSubmission<'a> {
    title: Option<&'a str>,
    description: Option<&'a str>,
    image: Option<i64>,
    published_at: Option<diesel::data_types::PgTimestamp>,
    visibility: Option<i32>,
}

impl<'a> UpdateSubmission<'a> {
    pub fn has_image(&self) -> bool {
        self.image.is_some()
    }

    pub fn get_visibility(&self) -> Option<Visibility> {
        self.visibility.map(|x| Visibility::from_i32(x))
    }

    pub fn new(sub: &Submission, mut image: Option<&File>, title: Option<&'a str>, desc: Option<&'a str>, vis: Option<i32>)
        -> Result<UpdateSubmission<'a>, (UpdateSubmission<'a>, SubmissionError)>
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

        if let Some(vis) = vis {
            if vis > 2 {
                se.visibility.push("Wrong input")
            }
        }

        let mut to_be_converted = None;

        if image.is_some() && image.as_ref().map(|x| x.size) == Some(0) {
            image = None;
        }

        if let Some(image) = image {
            if image.size > 3 * 1024 * 1024 { // 3 Megabytes
                se.image.push("Image is too big (limit is 3MiB)");
            }

            if let Ok(mut f) = image.open() {
                use std::io::Read;
                let mut buffer = Vec::new();
                if f.read_to_end(&mut buffer).is_err() {
                    se.image.push("Image is not in a valid format");
                } else {
                    to_be_converted = match image::guess_format(&buffer) {
                        Ok(image::PNG) | Ok(image::JPEG) | Ok(image::GIF) => {
                            match image::load_from_memory(&buffer) {
                                Ok(t) => {
                                    Some((t, image::guess_format(&buffer).unwrap()))
                                }
                                Err(e) => {
                                    info!("Could not load image {}", e);
                                    se.image.push("Image is not in a valid format");
                                    None
                                }
                            }
                        }
                        Ok(t) => {
                            info!("Could not load image format {:?}", t);
                            se.image.push("Image is not in a valid format, valid formats are: JPEG, PNG or GIF");
                            None
                        }
                        Err(t) => {
                            info!("Could not load image format {}", t);
                            se.image.push("Image is not in a valid format, valid formats are: JPEG, PNG or GIF");
                            None
                        }
                    }
                }
            } else {
                se.image.push("Could not use this image, please try again")
            }
        } else if !sub.has_image() {
            se.image.push("Image cannot be empty");
        }

        let image = to_be_converted.and_then(|(x, fmt)| convert_image(x, fmt));

        let us = UpdateSubmission {
            title: title,
            description: desc,
            image: image,
            published_at: None,
            visibility: vis
        };

        if se.has_any_errors() {
            return Err((us, se));
        }
        Ok(us)
    }
}


#[derive(Debug)]
pub struct SubmissionError {
    pub title: Vec<&'static str>,
    pub description: Vec<&'static str>,
    pub image: Vec<&'static str>,
    pub visibility: Vec<&'static str>,
}

impl SubmissionError {
    pub fn new() -> SubmissionError {
        SubmissionError {
            title: vec![],
            description: vec![],
            image: vec![],
            visibility: vec![],
        }
    }
    fn has_any_errors(&self) -> bool {
        !(self.title.is_empty()
          && self.description.is_empty()
          && self.image.is_empty()
          && self.visibility.is_empty())
    }
}

pub fn find(uid: i64) -> Result<Option<Submission>, error::FurryError> {
    use diesel::prelude::*;
    use models::schema::submissions::dsl::*;

    submissions.limit(1).filter(id.eq(uid))
         .get_result::<models::submission::Submission>(&*database::connection().get().unwrap()).optional().map_err(|e| e.into())
}

pub fn find_by_user_id(uid: i64) -> Result<Option<Submission>, error::FurryError> {
    use diesel::prelude::*;
    use models::schema::submissions::dsl::*;

    submissions.limit(1).filter(user_id.eq(uid)).order(created_at.desc())
         .get_result::<models::submission::Submission>(&*database::connection().get().unwrap()).optional().map_err(|e| e.into())
}

pub fn last(amt: i64) -> Result<Vec<Submission>, error::FurryError> {
    use diesel::prelude::*;
    use models::schema::submissions::dsl::*;

    submissions.limit(amt).order(created_at.desc()).filter(visibility.eq(0))
         .get_results::<models::submission::Submission>(&*database::connection().get().unwrap()).map_err(|e| e.into())
}


pub struct SubmissionFilter<'a> {
    settings: FilterSettings,
    submitter: Option<&'a User>,
    viewer: Option<&'a User>,
    title: Option<&'a str>,
    description: Option<&'a str>,
    unpublished: bool,
}

impl<'a> SubmissionFilter<'a> {
    pub fn new(settings: Option<FilterSettings>) -> SubmissionFilter<'a> {
        SubmissionFilter {
            settings: settings.unwrap_or_else(|| FilterSettings::default()),
            viewer: None,
            submitter: None,
            title: None,
            description: None,
            unpublished: false,
        }
    }

    pub fn with_submitter(mut self, user: &'a User) -> SubmissionFilter {
        self.submitter = Some(user);
        self
    }

    pub fn with_viewer(mut self, user: Option<&'a User>) -> SubmissionFilter {
        self.viewer = user;
        self
    }

    pub fn with_title(mut self, s: &'a str) -> SubmissionFilter {
        self.title = Some(s);
        self
    }

    pub fn with_decription(mut self, d: &'a str) -> SubmissionFilter {
        self.description = Some(d);
        self
    }

    pub fn with_unpublished(mut self) -> SubmissionFilter<'a> {
        self.unpublished = true;
        self
    }

    pub fn run(self) -> Result<Vec<Submission>, error::FurryError> {
        use diesel::prelude::*;
        use models::schema::submissions::dsl::*;

        let mut query = submissions.limit(self.settings.limit).into_boxed();

        if let Some(user) = self.submitter {
            query = query.filter(user_id.eq(user.id));
        }

        if let Some(n) = self.title {
            query = query.filter(title.like(n));
        }

        if let Some(desc) = self.description {
            query = query.filter(description.like(desc));
        }

        query = query.filter(visibility.eq(Visibility::Public as i32)
                                       .or(user_id.eq(self.viewer.map(|x| x.id).unwrap_or(-1))));
                                        // Either it is public, or the user is the owner

        if !self.unpublished {
            query = query.filter(visibility.ne(Visibility::Unpublished as i32));
        }

        query = query.order(created_at.desc());

        query.get_results::<models::submission::Submission>(&*database::connection().get().unwrap()).map_err(|e| e.into())
    }
}

impl HasOwner for Submission {
    fn get_owner(id: i64) -> Result<Option<User>, error::FurryError> {
        match find(id) {
            Ok(Some(sub)) => models::user::find(sub.user_id),
            Ok(None) => Err(error::FurryError::NotFound),
            Err(e) => Err(e)
        }
    }
}
