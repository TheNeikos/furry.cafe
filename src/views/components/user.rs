use std::fmt::{self, Display, Formatter};

use maud::PreEscaped;

use models::user::User;
use views::components::Image;

pub struct UserLink<'a>(pub &'a User);

impl<'a> Display for UserLink<'a> {
    fn fmt(&self, mut f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(&html!(
            a.user_link href=(format!("/users/{}", self.0.id)) alt=(format!("{}'s Profile", self.0.name)) span {
                (PreEscaped(UserAvatar(&self.0, (50, 50))))
                (self.0.name)
            }
        ).into_string())
    }
}

static DEFAULT_AVATAR : &'static str = "/assets/imgs/default_avatar.png";

pub struct UserAvatar<'a>(pub &'a User, pub (i32, i32));

impl<'a> Display for UserAvatar<'a> {
    fn fmt(&self, mut f: &mut Formatter) -> Result<(), fmt::Error> {
        match self.0.get_avatar() {
            Ok(Some(t)) => {
                f.write_str(&Image::new(&t).with_size(self.1).with_class("user_avatar").to_string()[..])
            },
            _ => {
                Ok(())
            }
        }
    }
}

