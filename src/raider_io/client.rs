#[allow(dead_code)]
pub enum Dungeons {
    Uldaman = 13968,
    Netharus = 14011,
    Brackenhide = 13991,
    HallsOfInfusion = 14082,
    VotexPinnacle = 5035,
    Underrot = 9391,
    NeltharionsLair = 7546,
    Freehold = 9164
}

#[allow(dead_code)]
pub enum Characters {
    Bearhug = 107062664,
    Ashensmashen = 111659444,
    Jollyjump = 110756699,
    Tempelton = 109787166,
    Scrappss = 111208402,
    Ceen = 14554173,
    Belabear = 57597833,
    Kharelle = 57597847,
    Camargo = 62373030,
    Mistreant = 138262808,
    Geelun = 57589007,
    Secretseris = 132985072,
    Hunløs = 99118409,
    Morggie = 48622064,
    Valonsoturi	= 139534621,
    Strixluna = 67017013,
    Nillgoto = 50248418,
    Zarewien = 79883814,
    Tsyl = 111747996,
    Sorashana = 68114607,
    Calamistress = 78826407,
    Pookiebean = 63762445,
}

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
