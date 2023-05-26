use serde::{Deserialize, Serialize};
use super::MythicPlusRun;

#[derive(Debug, Deserialize, Serialize)]
struct CharacterList {
    pub characters: Vec<Character>
}

#[derive(Debug, Deserialize, Serialize)]
struct Character {
    pub name: String,
    pub class: String,
    pub race: String,
    pub recent_runs: Vec<MythicPlusRun>,
    pub best_runs: Vec<MythicPlusRun>,
    pub thumbnail_url: String,
    pub region: String,
    pub realm: String,
    pub last_crawled_at: String,
    pub profile_url: String,
}