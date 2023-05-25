#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_dyn_templates::Template;
use rocket_db_pools::Database;
use rocket::{launch, get};
use rocket::fs::FileServer;
use rocket::response::Redirect;
use rocket::serde::{Serialize, Deserialize};

mod guild_roster;
mod character;
mod raider_io;
mod routes;
mod models;
mod database;

#[derive(Database)]
#[database("gwahaedir")]
pub struct RedisPool(deadpool_redis::Pool);

#[derive(Debug, Deserialize, Serialize)]
pub struct Error(String);

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!(routes::characters::get_all()))
}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .attach(RedisPool::init())
        .mount("/public", FileServer::from("./static"))
        .mount("/", routes![
            routes::characters::get,
            routes::characters::get_all,
            routes::characters::roster,
            index
        ])
}