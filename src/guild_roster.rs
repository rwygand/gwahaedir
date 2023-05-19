use std::error::Error;
use crate::{raider_io, RedisPool};
use raider_io::RaiderIO;
use serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;
use deadpool_redis::redis::AsyncCommands;

#[derive(Debug, Deserialize, Serialize)]
pub struct GuildRoster {
    pub name: String,
    pub faction: String,
    pub region: String,
    pub realm: String,
    pub last_crawled_at: String,
    pub profile_url: String,
    pub members: Vec<Member>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Member {
    pub rank: i32,
    pub character: Character,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Character {
    pub name: String,
    pub race: String,
    pub class: String,
    pub active_spec_name: Option<String>,
    pub active_spec_role: Option<String>,
    pub gender: String,
    pub faction: String,
    achievement_points: u32,
    honorable_kills: u32,
    region: String,
    realm: String,
    last_crawled_at: String,
    profile_url: String,
    profile_banner: String
}

/// Fetches the guild roster from redis. If it's not there, fetches it from
/// RaiderIO. Deserialized the result into a GuildRoster
pub async fn fetch(mut db: Connection<RedisPool>) -> Result<GuildRoster, Box<dyn Error>> {
    let mut cache_data = db.get("guild_roster").await;

    let mut data_s: String = match cache_data {
        Ok(s) => {
            println!("Found guild_roster in cache!");
            s
        },
        Err(err) => {
            println!("guild_roster not found in cache!");
            "".to_string()
        }
    };


    if data_s.is_empty() {
        let rio_client = RaiderIO::new();
        data_s = rio_client.get_roster().await?;
        let foo = db.set("guild_roster", &data_s).await?;
        println!("Wrote guild_roster to cache!");

    }

    let roster = serde_json::from_str(data_s.as_str())?;
    Ok(roster)
}