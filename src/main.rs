mod raider_io;
mod guild_roster;
use serde::{Deserialize, Serialize};

#[macro_use] extern crate rocket;
use std::collections::HashMap;
use rocket_dyn_templates::Template;
use rocket_db_pools::Database;
use rocket::{launch, get};
use rocket::response::status::NotFound;
use rocket_db_pools::Connection;

#[derive(Database)]
#[database("gwahaedir")]
pub struct RedisPool(deadpool_redis::Pool);

#[derive(Debug, Deserialize, Serialize)]
pub struct Error(String);

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> =  HashMap::new();
    Template::render("index", context)
}

#[get("/roster")]
async fn roster(db: Connection<RedisPool>) -> Result<Template, NotFound<String>> {
    let roster = guild_roster::fetch(db).await;
    match roster {
        Ok(roster) => Ok(Template::render("roster", roster)),
        Err(err) => Err(NotFound(format!("Error: {}", err.to_string())))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(RedisPool::init())
        .mount("/", routes![index])
        .mount("/", routes![roster])
        .attach(Template::fairing())
}