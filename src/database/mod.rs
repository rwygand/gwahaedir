pub struct RaiderIO {
    client: reqwest::Client
}

impl RaiderIO {
    pub fn new() -> RaiderIO {
        RaiderIO {
            client: reqwest::Client::new()
        }
    }

    pub async fn get_roster(&self) -> Result<String, Box<dyn std::error::Error>> {
        let url = "https://raider.io/api/v1/guilds/profile?region=us&realm=proudmoore&name=power%20word%20taint&fields=members";

        let resp = self.client.get(url)
            .send()
            .await?
            .text()
            .await?;

        Ok(resp)
    }

    pub async fn get_character(&self, char_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!(
            "https://raider.io/api/v1/characters/profile?region=us&realm=proudmoore&name={}&fields=mythic_plus_recent_runs%2Cmythic_plus_best_runs",
            char_name
        );

        let resp = self.client.get(url)
            .send()
            .await?
            .text()
            .await?;

        Ok(resp)
    }
}