#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate core;

use rocket_dyn_templates::Template;
use rocket_db_pools::Database;
use rocket::{launch, get};
use rocket::fs::FileServer;
use rocket::response::Redirect;

mod raider_io;
mod routes;
mod models;
mod database;

use database::RedisPool;

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!(routes::characters::get_all()))
}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .attach(database::RedisPool::init())
        .mount("/public", FileServer::from("./static"))
        .mount("/", routes![
            routes::characters::get,
            routes::characters::get_all,
            routes::characters::roster,
            index
        ])
}