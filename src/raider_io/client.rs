use std::error::Error;
use crate::raider_io::{CharacterDetail, GuildRoster};
use super::models::PeriodList;

pub struct RaiderIO {
    client: reqwest::Client
}

impl RaiderIO {
    pub fn new() -> RaiderIO {
        RaiderIO {
            client: reqwest::Client::new()
        }
    }

    pub async fn get_roster(&self) -> Result<GuildRoster, Box<dyn Error>> {
        let url = "https://raider.io/api/v1/guilds/profile?region=us&realm=proudmoore&name=power%20word%20taint&fields=members";

        let resp = self.client
            .get(url)
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }

    pub async fn get_character(&self, char_name: &str) -> Result<CharacterDetail, Box<dyn Error>> {
        let url = format!(
            "https://raider.io/api/v1/characters/profile?region=us&realm=proudmoore&name={}&fields=mythic_plus_recent_runs%2Cmythic_plus_best_runs",
            char_name
        );

        let resp = self.client.get(url)
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }

    #[allow(dead_code)]
    pub async fn get_periods(&self) -> Result<PeriodList, Box<dyn Error>> {
        let resp = self.client.get("https://raider.io/api/v1/periods")
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }

}
