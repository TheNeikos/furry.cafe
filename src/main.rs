#![feature(plugin)]
#![plugin(maud_macros)]
#![feature(proc_macro)]
#![plugin(dotenv_macros)]

#![allow(dead_code)]


#[macro_use] extern crate iron;
extern crate router;
extern crate mount;
extern crate maud;
extern crate params;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
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
extern crate image;
#[macro_use] extern crate quick_error;
extern crate rand;
extern crate rustc_serialize;
extern crate lettre;

use std::env;
use std::path::Path;
use std::time::Duration;

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
mod helper;

fn main() {
    dotenv().ok();
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    let mut index_router = Router::new();
    index_router.get("/",      controllers::root::handler,  "index");
    index_router.get("/about", controllers::about::handler, "about");

    let user_router = {
        //---------------------------------
        //---- Normal Usage ---------------
        //---------------------------------
        let mut router = Router::new();

        router.get("/new",             controllers::user::new,         "user_new");
        router.post("/",               controllers::user::create,      "user_create");
        router.get("/:id/submissions", controllers::user::submissions, "user_submissions");
        router.get("/:id",             controllers::user::show,        "user_show");

        let auth = middleware::authorization::Authorizer::new(vec![
            middleware::authorization::LoggedIn
        ]);
        let mut chain = Chain::new(controllers::user::index);
        chain.link_before(auth.clone());
        router.get("/",         chain, "user_index");

        let auth = middleware::authorization::Authorizer::new(vec![
            middleware::authorization::SameUserAuth
        ]);
        let mut chain = Chain::new(controllers::user::edit);
        chain.link_before(auth.clone());

        router.get("/:id/edit", chain, "user_edit");

        let mut chain = Chain::new(controllers::user::update);
        chain.link_before(auth.clone());
        router.post("/:id",     chain, "user_update");


        //---------------------------------
        //---- Profile --------------------
        //---------------------------------

        let mut chain = Chain::new(controllers::user_profile::edit);
        chain.link_before(auth.clone());

        router.get("/:id/profile/edit", chain, "user_profile_edit");

        let mut chain = Chain::new(controllers::user_profile::update);
        chain.link_before(auth.clone());
        router.post("/:id/profile",     chain, "user_profile_update");

        // FIXME: Disable accounts rather than deleting them
        // router.delete("/:id",   controllers::user::delete);
        router
    };

    let mut login_router = Router::new();
    login_router.get("/", controllers::login::new, "login_new");
    login_router.post("/", controllers::login::create, "login_create");

    let mut logout_router = Router::new();
    logout_router.get("/", controllers::login::delete, "login_delete");
    logout_router.post("/", controllers::login::destroy, "login_destroy");

    let sub_router = {
        let mut router = Router::new();
        router.get("/",         controllers::submission::index, "submission_index");
        router.get("/:id",      controllers::submission::show, "submission_show");


        let auth = middleware::authorization::Authorizer::new(vec![
            middleware::authorization::LoggedIn,
        ]);

        let mut chain = Chain::new(controllers::submission::create);
        chain.link_before(auth.clone());
        router.post("/",        chain, "submission_create");

        let auth = middleware::authorization::Authorizer::new(vec![
            middleware::authorization::IsOwner::<::models::submission::Submission>::new(),
        ]);

        let mut chain = Chain::new(controllers::submission::edit);
        chain.link_before(auth.clone());
        router.get("/:id/edit", chain, "submission_edit");

        let mut chain = Chain::new(controllers::submission::update);
        chain.link_before(auth.clone());
        router.post("/:id",     chain, "submission_update");

        let mut chain = Chain::new(controllers::submission::delete);
        chain.link_before(auth.clone());
        router.post("/:id/delete",     chain, "submission_delete");

        router
    };

    let admin_chain = {
        let mut router = Router::new();

        router.get("/invites",  controllers::invite::index, "admin_invite_index");
        router.post("/invites", controllers::invite::create, "admin_invite_create");

        let auth = middleware::authorization::Authorizer::new(vec![
            middleware::authorization::HasRole(models::user_role::Role::Admin),
        ]);

        let mut chain = Chain::new(router);
        chain.link_before(auth);

        chain
    };

    let mut mount = Mount::new();
    mount.mount("/", index_router)
         .mount("/admin",      admin_chain)
         .mount("/users",       user_router)
         .mount("/login",       login_router)
         .mount("/logout",      logout_router)
         .mount("/submissions", sub_router)
         .mount("/assets/", staticfile::Static::new(Path::new("assets/")).cache(Duration::new(60 * 60 * 24 * 7, 0)));


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

