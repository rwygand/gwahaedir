use rocket_db_pools::Database;
use std::error::Error;
use crate::raider_io::*;
use rocket_db_pools::Connection;
use deadpool_redis::redis::AsyncCommands;
use crate::models::{CharacterInfo, CharacterList};

#[derive(Database)]
#[database("gwahaedir")]
pub struct RedisPool(deadpool_redis::Pool);

/// Fetches the guild roster from redis. If it's not there, fetches it from
/// RaiderIO. Deserialized the result into a GuildRoster
pub async fn characters(mut db: Connection<RedisPool>) -> Result<CharacterList, Box<dyn Error>> {
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

    let roster: GuildRoster;

    if data_s.is_empty() {
        roster = RaiderIO::new().get_roster().await?;
        // best effort cache write
        let data = serde_json::to_string(&roster)?;
        db.set_ex("guild_roster", &data, 60 * 60 * 24).await?;
        println!("Wrote guild_roster to cache!");
    } else {
        roster = serde_json::from_str(data_s.as_str())?
    }
    Ok(CharacterList::from(roster))
}

/// Fetches a character detail record from redis. If it's not there, fetches it from
/// RaiderIO. Deserializes the result into a CharacterDetail
pub async fn character(mut db: Connection<RedisPool>, char_name: &str) -> Result<CharacterInfo, Box<dyn Error>> {
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

    Ok(CharacterInfo::from(chr))
}

pub async fn periods(mut db: Connection<RedisPool>) -> Result<PeriodList, Box<dyn Error>> {
    let cache_data = db.get("periods").await;
    let data_s: String = match cache_data {
        Ok(s) => {
            println!("Found periods in cache!");
            s
        },
        Err(_err) => {
            println!("periods not found in cache!");
            "".to_string()
        }
    };

    let p;

    if data_s.is_empty() {
        p = RaiderIO::new().get_periods().await?;
        // best effort write to cache
        let data = serde_json::to_string(&p)?;
        db.set_ex("periods", &data, 60 * 60 * 4).await?;
        println!("Wrote periods to cache!");
    } else {
        p = serde_json::from_str(data_s.as_str())?;
    }

    Ok(p)
}