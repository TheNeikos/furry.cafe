use maud::Render;

use models::user::User;
use views::components::Image;

pub struct UserLink<'a>(pub &'a User);

impl<'a> Render for UserLink<'a> {
    fn render_to(&self, mut f: &mut String) {
        f.push_str(&html!(
            a.user_link href=(format!("/users/{}", self.0.id)) alt=(format!("{}'s Profile", self.0.name)) span {
                (UserAvatar(&self.0, (50, 50)))
                (self.0.name)
            }
        ).into_string())
    }
}

static DEFAULT_AVATAR : &'static str = "/assets/images/default_avatar.png";

pub struct UserAvatar<'a>(pub &'a User, pub (i32, i32));

impl<'a> Render for UserAvatar<'a> {
    fn render_to(&self, mut f: &mut String) {
        match self.0.get_avatar() {
            Ok(Some(t)) => {
                Image::new(&t).with_size(self.1).with_class("user_avatar").render_to(f);
            },
            Ok(None) => {
                f.push_str(&format!("<img class='user_avatar' src='{}' />", DEFAULT_AVATAR))
            }
            _ => { }
        }
    }
}

