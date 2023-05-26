use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::models::MythicPlusRun;
use crate::raider_io;

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterList {
    pub name: String,
    pub faction: String,
    pub region: String,
    pub realm: String,
    pub characters: Vec<Character>
}

impl From<raider_io::GuildRoster> for CharacterList {
    fn from(g: raider_io::GuildRoster) -> Self {
        let mut characters = vec![];
        for m in g.members.iter() {
            characters.push(Character::from(m));
        }
        CharacterList {
            name: g.name.clone(),
            faction: g.faction.clone(),
            region: g.region.clone(),
            realm: g.realm,
            characters,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Character {
    pub rank: i32,
    pub name: String,
    pub class: String,
    pub race: String,
    pub region: String,
    pub realm: String,
    pub last_crawled_at: DateTime<Utc>,
    pub profile_url: String,
}

impl From<&raider_io::Member> for Character {
    fn from(m: &raider_io::Member) -> Self {
        Character {
            rank: m.rank,
            name: m.character.name.clone(),
            class: m.character.class.clone(),
            race: m.character.race.clone(),
            region: m.character.region.clone(),
            realm: m.character.realm.clone(),
            last_crawled_at: m.character.last_crawled_at,
            profile_url: m.character.profile_url.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterInfo {
    pub name: String,
    pub race: String,
    pub class: String,
    pub gender: String,
    pub faction: String,
    pub achievement_points: u64,
    pub honorable_kills: u64,
    pub thumbnail_url: String,
    pub region: String,
    pub realm: String,
    pub last_crawled_at: DateTime<Utc>,
    pub profile_url: String,
    pub mythic_plus_recent_runs: Vec<MythicPlusRun>,
    pub mythic_plus_best_runs: Vec<MythicPlusRun>
}

impl From<raider_io::CharacterDetail> for CharacterInfo {
    fn from(c: raider_io::CharacterDetail) -> Self {
        CharacterInfo {
            name: c.name.clone(),
            race: c.race.clone(),
            class: c.class.clone(),
            gender: c.gender.clone(),
            faction: c.faction.clone(),
            achievement_points: c.achievement_points,
            honorable_kills: c.honorable_kills,
            thumbnail_url: c.thumbnail_url.clone(),
            region: c.region.clone(),
            realm: c.realm.clone(),
            last_crawled_at: c.last_crawled_at,
            profile_url: c.profile_url.clone(),
            mythic_plus_recent_runs: c.mythic_plus_recent_runs
                .into_iter()
                .map(MythicPlusRun::from)
                .collect(),
            mythic_plus_best_runs: c.mythic_plus_best_runs
                .into_iter()
                .map(MythicPlusRun::from)
                .collect(),
        }
    }
}