use serde::{Deserialize, Serialize};
use chrono::prelude::*;

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
}