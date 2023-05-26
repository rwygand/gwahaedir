use rocket_dyn_templates::Template;
use rocket::get;
use rocket::response::Redirect;
use rocket::response::status::NotFound;
use rocket_db_pools::Connection;
use crate::{database, RedisPool};

#[get("/characters")]
pub async fn get_all(db: Connection<RedisPool>) -> Result<Template, NotFound<String>> {
    let roster = database::characters(db).await;
    match roster {
        Ok(roster) => Ok(Template::render("roster", roster)),
        Err(err) => Err(NotFound(format!("Error: {}", err.to_string())))
    }
}

#[get("/character/<char_name>")]
pub async fn get(db: Connection<RedisPool>, char_name: &str) -> Result<Template, NotFound<String>> {
    let char = database::character(db, char_name).await;
    match char {
        Ok(cd) => Ok(Template::render("character", cd)),
        Err(err) => Err(NotFound(format!("Error: {}", err.to_string())))
    }
}

// backwards bookmark compatibility
#[get("/roster")]
pub fn roster() -> Redirect {
    Redirect::to(uri!(get_all()))
}
