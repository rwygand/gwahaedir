use rocket_dyn_templates::Template;
use rocket::get;
use rocket::response::Redirect;
use crate::{database, RedisPool, errors::AppError};

#[get("/characters")]
pub async fn get_all(db: &RedisPool) -> Result<Template,  AppError> {
    let roster = database::characters(db.get().await?).await?;

    Ok(Template::render("roster", roster))
}

#[get("/character/<char_name>")]
pub async fn get(db: &RedisPool, char_name: &str) -> Result<Template, AppError> {
    let mut char = database::character(db.get().await?, char_name).await?;
    let mut pl = database::periods(db.get().await?).await?;
    char.professions = database::char_profs(db.get().await?, char_name).await?;

    pl.periods.retain(|x| x.region.to_lowercase() == "us");
    let period_start = pl.periods[0].current.start;
    let period_end = pl.periods[0].current.end;
    for r in char.mythic_plus_recent_runs.iter_mut() {
        r.current = r.completed_at > period_start && r.completed_at < period_end;
    }
    Ok(Template::render("character", char))
}

#[get("/character/refresh/<char_name>")]
pub async fn refresh(db: &RedisPool, char_name: &str) -> Result<Redirect, AppError> {
    database::clear_character(db.get().await?, char_name).await?;
    Ok(Redirect::to(uri!(get(char_name))))
}

// backwards bookmark compatibility
#[get("/roster")]
pub fn roster() -> Redirect {
    Redirect::to(uri!(get_all()))
}
