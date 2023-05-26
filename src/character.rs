use std::error::Error;
use crate::RedisPool;
use crate::raider_io::*;
use rocket_db_pools::Connection;
use deadpool_redis::redis::AsyncCommands;

/// Fetches a character detail record from redis. If it's not there, fetches it from
/// RaiderIO. Deserializes the result into a CharacterDetail
pub async fn fetch(mut db: Connection<RedisPool>, char_name: &str) -> Result<CharacterDetail, Box<dyn Error>> {
    let cache_data = db.get(char_name).await;
    let data_s: String = match cache_data {
        Ok(s) => {
            println!("Found character {} in cache!", char_name);
            s
        },
        Err(_err) => {
            println!("character {} not found in cache!", char_name);
            "".to_string()
        }
    };

    let chr;

    if data_s.is_empty() {
        chr = RaiderIO::new().get_character(char_name).await?;
        // best effort write to cache
        let data = serde_json::to_string(&chr)?;
        db.set_ex(char_name, &data, 60 * 60 * 4).await?;
        println!("Wrote character {} to cache!", char_name);
    } else {
        chr = serde_json::from_str(data_s.as_str())?;
    }

    Ok(chr)
}