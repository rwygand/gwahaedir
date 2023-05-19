mod raider_io;

#[macro_use] extern crate rocket;
use std::collections::HashMap;
use deadpool_redis::redis::{AsyncCommands, RedisResult};
use rocket_dyn_templates::Template;
use rocket_db_pools::Database;
use rocket::{launch, get};
use rocket_db_pools::Connection;

#[derive(Database)]
#[database("gwahaedir")]
pub struct RedisPool(deadpool_redis::Pool);

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> =  HashMap::new();
    Template::render("index", context)
}

#[get("/roster")]
async fn roster(mut db: Connection<RedisPool>) -> Template {
    let roster: RedisResult<String> = db.get("guild_roster").await;
    let rio_client = raider_io::RaiderIO::new();
    let res = rio_client.get_roster();
    let res = res.await;
    Template::render("roster", res.unwrap())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(RedisPool::init())
        .mount("/", routes![index])
        .mount("/", routes![roster])
        .attach(Template::fairing())
}