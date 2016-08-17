#![feature(plugin)]
#![plugin(maud_macros)]
#![feature(custom_derive, custom_attribute)]
#![plugin(diesel_codegen, dotenv_macros)]


#[macro_use] extern crate iron;
extern crate router;
extern crate mount;
extern crate maud;
extern crate params;
#[macro_use] extern crate diesel;
extern crate dotenv;
#[macro_use] extern crate lazy_static;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate bcrypt;
extern crate iron_login;
extern crate staticfile;
#[macro_use] extern crate log;
extern crate log4rs;
extern crate pulldown_cmark;
extern crate maud_pulldown_cmark;

use std::env;
use std::path::Path;

use iron::prelude::*;
use router::Router;
use mount::Mount;
use dotenv::dotenv;

#[macro_use]
mod macros;
mod database;
mod controllers;
mod views;
mod error;
mod models;
mod logger;
mod middleware;

fn main() {
    dotenv().ok();
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    let mut index_router = Router::new();
    index_router.get("/", controllers::root::handler);
    index_router.get("/about", controllers::about::handler);

    let user_router = {
        //---------------------------------
        //---- Normal Usage ---------------
        //---------------------------------
        let mut router = Router::new();
        router.get("/",         controllers::user::index);
        router.get("/new",      controllers::user::new);
        router.post("/",        controllers::user::create);
        router.get("/:id",      controllers::user::show);

        let auth = middleware::authorization::Authorizer::new(vec![
            middleware::authorization::SameUserAuth
        ]);
        let mut chain = Chain::new(controllers::user::edit);
        chain.link_before(auth.clone());

        router.get("/:id/edit", chain);

        let mut chain = Chain::new(controllers::user::update);
        chain.link_before(auth.clone());

        router.put("/:id",      chain);

        let mut chain = Chain::new(controllers::user::update);
        chain.link_before(auth.clone());
        router.post("/:id",     chain);


        //---------------------------------
        //---- Profile --------------------
        //---------------------------------

        let mut chain = Chain::new(controllers::user_profile::edit);
        chain.link_before(auth.clone());

        router.get("/:id/profile/edit", chain);

        let mut chain = Chain::new(controllers::user_profile::update);
        chain.link_before(auth.clone());

        router.put("/:id/profile",      chain);

        let mut chain = Chain::new(controllers::user_profile::update);
        chain.link_before(auth.clone());
        router.post("/:id/profile",     chain);

        // FIXME: Disable accounts rather than deleting them
        // router.delete("/:id",   controllers::user::delete);
        router
    };

    let mut login_router = Router::new();
    login_router.get("/", controllers::login::new);
    login_router.post("/", controllers::login::create);

    let mut logout_router = Router::new();
    logout_router.get("/", controllers::login::delete);
    logout_router.post("/", controllers::login::destroy);

    let mut mount = Mount::new();
    mount.mount("/", index_router)
         .mount("/users", user_router)
         .mount("/login", login_router)
         .mount("/logout", logout_router)
         .mount("/assets/", staticfile::Static::new(Path::new("assets/")));


    let cookie_secret= env::var("COOKIE_SECRET")
        .expect("COOKIE_SECRET must be set").into_bytes();

    let mut log_chain = Chain::new(mount);
    log_chain.link_before(logger::Logger);
    log_chain.link_before(middleware::MethodOverride);

    log_chain.link_around(iron_login::LoginManager::new(cookie_secret));

    log_chain.link_after(middleware::ErrorHandler);
    log_chain.link_after(logger::Logger);
    Iron::new(log_chain).http("0.0.0.0:3000").unwrap();
}

