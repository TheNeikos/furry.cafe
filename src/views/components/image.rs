use maud::Render;

use models::image;

static DEFAULT_AVATAR : &'static str = "/assets/imgs/default_avatar.png";

pub struct Image<'a> {
    pub img: &'a image::Image,
    pub size: Option<(i32, i32)>,
    pub class: Option<&'a str>,
}

impl<'a> Image<'a> {
    pub fn new(img: &'a image::Image) -> Image<'a> {
        Image {
            img: img,
            size: None,
            class: None,
        }
    }

    pub fn with_class(&mut self, cl: &'a str) -> &mut Image<'a> {
        self.class = Some(cl);
        self
    }

    pub fn with_size(&mut self, size: (i32, i32)) -> &mut Image<'a> {
        self.size = Some(size);
        self
    }
}

impl<'a> Render for Image<'a> {
    fn render_to(&self, f: &mut String) {

        let mut img = |i: &str| {
                f.push_str(&html!(
                    img src=(i) class=(self.class.unwrap_or(""))/
                ).into_string())
        };

        match self.size {
            Some((width, height)) => {
                match self.img.get_with_size(width, height) {
                    Ok(i) => img(&i.get_path()[..]),
                    Err(e) => {
                        error!("Could not get image resized: {:?}", e);
                        img("/not_found.png")
                    }
                }
            }
            None => {
                img(&self.img.get_path())
            }
        }
    }
}

