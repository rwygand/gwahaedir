use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use crate::raider_io;

#[derive(Debug, Deserialize, Serialize)]
pub struct MythicPlusRun {
    pub name: String,
    pub mythic_level: u8,
    pub completed_at: DateTime<Utc>,
    pub clear_time_ms: u64,
    pub par_time_ms: u64,
    pub num_keystone_upgrades: u8,
    pub zone_id: u32,
    pub score: f64,
    pub url: String,
    pub current: bool,
}

impl From<raider_io::MythicRun> for MythicPlusRun {
    fn from(r: raider_io::MythicRun) -> Self {
        MythicPlusRun {
            name: r.dungeon.clone(),
            mythic_level: r.mythic_level,
            completed_at: r.completed_at,
            clear_time_ms: r.clear_time_ms,
            par_time_ms: r.par_time_ms,
            num_keystone_upgrades: r.num_keystone_upgrades.unwrap_or(0),
            zone_id: r.zone_id,
            score: r.score,
            url: r.url,
            current: false, // default to false
        }
    }
}