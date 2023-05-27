use rocket::get;
use rocket_db_pools::Connection;
use crate::{database, RedisPool};
use crate::raider_io::PeriodList;

#[get("/periods")]
pub async fn periods(db: Connection<RedisPool>) -> String {
    let result = database::periods(db.into_inner()).await;
    let x = result.unwrap_or(PeriodList::default());
    let json = serde_json::to_string(&x);
    json.unwrap_or("".to_string())
}