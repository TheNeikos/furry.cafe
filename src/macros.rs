

#[macro_export]
macro_rules! resource {
    ($name:ident) => {{
        use $crate::router::Router;
        let mut router = Router::new();
        router.get("/",         $name::index);
        router.get("/new",      $name::new);
        router.post("/",        $name::create);
        router.get("/:id",      $name::show);
        router.get("/:id/edit", $name::edit);
        router.put("/:id",      $name::update);
        router.post("/:id",      $name::update);
        router.delete("/:id",   $name::delete);
        router
    }}
}

#[macro_export]
macro_rules! temp_redirect {
    ($url:expr) => {
        (status::SeeOther, Redirect(url!($url)))
    }
}


lazy_static! {
    pub static ref URL_PATH: String = {
        use std::env;
        env::var("FULL_URL")
        .expect("COOKIE_SECRET must be set")
    };
}

#[macro_export]
macro_rules! url {
    ($url:expr) => {{
        use iron::Url;
        let s: &String = &$crate::macros::URL_PATH;
        Url::parse(&(format!("{}{}", s, $url)[..])).unwrap()
    }}
}

#[macro_export]
macro_rules! find_by_id {
    ($req:ident, $name:expr, $module:ident) => {{
        use iron::prelude::*;
        use iron::status;
        use router::Router;

        let id = match $req.extensions.get::<Router>().unwrap().find($name) {
            Some(t) => {
                let matches : Vec<_> = t.splitn(2, '-').collect();
                if let Some(id) = matches.get(0) {
                    match id.parse::<_>() {
                        Ok(t) => Ok(t),
                        Err(_) => Err(IronError::new(error::FurryError::BadFormatting, temp_redirect!("/")))
                    }
                } else {
                    Err(IronError::new(error::FurryError::BadFormatting, temp_redirect!("/")))
                }
            }
            None => {
                Err(IronError::new(error::FurryError::BadFormatting, temp_redirect!("/")))
            }
        };

        id.and_then(|id| {
            match $module::find(id) {
                Err(e) => {
                    Err(e.into())
                },
                Ok(Some(u)) => Ok(u),
                Ok(None) => {
                    Err(IronError::new(error::FurryError::NotFound, status::NotFound))
                }
            }
        })
    }}
}
