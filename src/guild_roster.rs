use std::error::Error;
use crate::RedisPool;
use crate::raider_io::*;
use rocket_db_pools::Connection;
use deadpool_redis::redis::AsyncCommands;

/// Fetches the guild roster from redis. If it's not there, fetches it from
/// RaiderIO. Deserialized the result into a GuildRoster
pub async fn fetch(mut db: Connection<RedisPool>) -> Result<GuildRoster, Box<dyn Error>> {
    let cache_data = db.get("guild_roster").await;
    let data_s: String = match cache_data {
        Ok(s) => {
            println!("Found guild_roster in cache!");
            s
        },
        Err(_err) => {
            println!("guild_roster not found in cache!");
            "".to_string()
        }
    };

    let roster;

    if data_s.is_empty() {
        roster = RaiderIO::new().get_roster().await?;
        // best effort cache write
        let data = serde_json::to_string(&roster)?;
        db.set_ex("guild_roster", &data, 60 * 60 * 24).await?;
        println!("Wrote guild_roster to cache!");
    } else {
        roster = serde_json::from_str(data_s.as_str())?
    }

    Ok(roster)
}