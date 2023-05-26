use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Period {
    pub period: i64,
    pub start: DateTime<Utc>,
    pub end: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Periods {
    pub region: String,
    pub previous: Period,
    pub current: Period,
    pub next: Period,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PeriodList {
    pub periods: Vec<Periods>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
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

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MythicRun {
    pub dungeon: String,
    pub short_name: String,
    pub mythic_level: u8,
    pub completed_at: DateTime<Utc>,
    pub clear_time_ms: u64,
    pub par_time_ms: u64,
    pub num_keystone_upgrades: Option<u8>,
    pub map_challenge_mode_id: u32,
    pub zone_id: u32,
    pub score: f64,
    pub affixes: Vec<Affix>,
    pub url: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Affix {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub wowhead_url: String
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct GuildRoster {
    pub name: String,
    pub faction: String,
    pub region: String,
    pub realm: String,
    pub last_crawled_at: String,
    pub profile_url: String,
    pub members: Vec<Member>
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Member {
    pub rank: i32,
    pub character: Character,
}

#[derive(Serialize, Deserialize, Default, Debug)]
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