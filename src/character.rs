use std::error::Error;
use crate::{raider_io, RedisPool};
use raider_io::RaiderIO;
use serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;
use deadpool_redis::redis::AsyncCommands;

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterDetail {
    pub name: String,
    pub race: String,
    pub class: String,
    pub active_spec_name: String,
    pub active_spec_role: String,
    pub gender: String,
    pub faction: String,
    pub achievement_points: u64,
    pub honorable_kills: u64,
    pub thumbnail_url: String,
    pub region: String,
    pub realm: String,
    pub last_crawled_at: String,
    pub profile_url: String,
    pub profile_banner: String,
    pub mythic_plus_recent_runs: Vec<MythicRun>,
    pub mythic_plus_best_runs: Vec<MythicRun>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MythicRun {
    pub dungeon: String,
    pub short_name: String,
    pub mythic_level: u8,
    pub completed_at: String,
    pub clear_time_ms: u64,
    pub par_time_ms: u64,
    pub num_keystone_upgrades: Option<u8>,
    pub map_challenge_mode_id: u32,
    pub zone_id: u32,
    pub score: f64,
    pub affixes: Vec<Affix>,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Affix {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub wowhead_url: String
}

/// Fetches a character detail record from redis. If it's not there, fetches it from
/// RaiderIO. Deserializes the result into a CharacterDetail
pub async fn fetch(mut db: Connection<RedisPool>, char_name: &str) -> Result<CharacterDetail, Box<dyn Error>> {
    let cache_data = db.get(char_name).await;
    let mut data_s: String = match cache_data {
        Ok(s) => {
            println!("Found character {} in cache!", char_name);
            s
        },
        Err(_err) => {
            println!("character {} not found in cache!", char_name);
            "".to_string()
        }
    };


    if data_s.is_empty() {
        let rio_client = RaiderIO::new();
        data_s = rio_client.get_character(char_name).await?;
        // best effort write to cache
        db.set_ex(char_name, &data_s, 60 * 60 * 4).await?;
        println!("Wrote character {} to cache!", char_name);
    }

    let roster = serde_json::from_str(data_s.as_str())?;
    Ok(roster)
}