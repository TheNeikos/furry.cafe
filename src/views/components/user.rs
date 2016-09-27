use std::fmt::{self, Display, Formatter};

use maud::PreEscaped;

use models::user::User;

pub struct UserLink<'a>(pub &'a User);

impl<'a> Display for UserLink<'a> {
    fn fmt(&self, mut f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(&html!(
            a.user_link href=(format!("/users/{}", self.0.id)) alt=(format!("{}'s Profile", self.0.name)) span {
                (PreEscaped(UserAvatar(&self.0)))
                (self.0.name)
            }
        ).into_string())
    }
}

static DEFAULT_AVATAR : &'static str = "/assets/imgs/default_avatar.png";

pub struct UserAvatar<'a>(pub &'a User);

impl<'a> Display for UserAvatar<'a> {
    fn fmt(&self, mut f: &mut Formatter) -> Result<(), fmt::Error> {
        match self.0.get_avatar() {
            Ok(Some(t)) => {
                f.write_str(&html!(
                    img.user_avatar src=(t.get_path()) alt=(format!("{}'s Avatar", self.0.name)) /
                ).into_string())
            },
            _ => {
                Ok(())
            }
        }
    }
}

