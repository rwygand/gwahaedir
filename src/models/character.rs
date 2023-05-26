use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::raider_io::{GuildRoster, Member};

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterList {
    pub name: String,
    pub faction: String,
    pub region: String,
    pub realm: String,
    pub characters: Vec<Character>
}

impl From<GuildRoster> for CharacterList {
    fn from(g: GuildRoster) -> Self {
        let mut characters = vec![];
        for m in g.members.iter() {
            characters.push(Character::from(m));
        }
        CharacterList {
            name: g.name.clone(),
            faction: g.faction.clone(),
            region: g.region.clone(),
            realm: g.realm.clone(),
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

impl From<&Member> for Character {
    fn from(m: &Member) -> Self {
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