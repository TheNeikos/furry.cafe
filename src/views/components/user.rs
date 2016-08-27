use std::fmt::{self, Display, Formatter};

use maud::PreEscaped;

use models::user::User;

pub struct UserLink<'a>(pub &'a User);

impl<'a> Display for UserLink<'a> {
    fn fmt(&self, mut f: &mut Formatter) -> Result<(), fmt::Error> {
        html!(f,
            a.user_link href=^(format!("/users/{}", self.0.id)) alt=^(format!("{}'s Profile", self.0.name)) span {
                ^(PreEscaped(UserAvatar(&self.0)))
                ^(self.0.name)
            }
        )
    }
}

static DEFAULT_AVATAR : &'static str = "/assets/imgs/default_avatar.png";

pub struct UserAvatar<'a>(pub &'a User);

impl<'a> Display for UserAvatar<'a> {
    fn fmt(&self, mut f: &mut Formatter) -> Result<(), fmt::Error> {

        let avatar = match self.0.get_avatar() {
                Ok(Some(t)) => t.get_path(),
                _ => String::from(DEFAULT_AVATAR),
        };

        html!(f,
            img.user_avatar src=^(avatar) alt=^(format!("{}'s Avatar", self.0.name)) /
        )
    }
}

