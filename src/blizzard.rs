use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::errors::AppError;

#[derive(Debug, Deserialize, Serialize)]
pub struct OAuthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: usize,
}

pub async fn get_oauth_token(
    client_id: &String,
    client_secret: &String,
    region: &str,
) -> Result<OAuthToken, Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();
    let url = if region.to_lowercase() == "cn" {
        "https://www.battlenet.com.cn/oauth/token".to_string()
    } else {
        format!("https://{}.battle.net/oauth/token", region.to_lowercase())
    };

    let resp: OAuthToken = client
        .post(&url)
        .basic_auth(client_id, Some(client_secret))
        .form(&[("grant_type", "client_credentials")])
        .send()
        .await?
        .json()
        .await?;

    Ok(resp)
}

#[allow(dead_code)]
async fn get_roster(access_token: String) -> Result<String, reqwest::Error> {
    let header = format!("Bearer {}", access_token);
    let url = "https://us.api.blizzard.com/data/wow/guild/proudmoore/power-word-taint/roster?namespace=profile-us&locale=en_US";

    let resp = reqwest::Client::new()
        .get(url)
        .header(reqwest::header::AUTHORIZATION, header)
        .send()
        .await?
        .text()
        .await?;

    Ok(resp)
}

pub async fn get_professions(access_token: String, name: &str) -> Result<CharacterProfessions, AppError> {
    let header = format!("Bearer {}", access_token);
    let url = format!("https://us.api.blizzard.com/profile/wow/character/proudmoore/{}/professions?namespace=profile-us&locale=en_US", name.to_lowercase());

    println!("Fetching: {}", url);
    let x = reqwest::Client::new()
        .get(url)
        .header(reqwest::header::AUTHORIZATION, header)
        .send()
        .await?
        .text()
        .await?;

    println!("text body: {}", x);

    let res: CharacterProfessions = serde_json::from_str(x.as_str())?;
    Ok(res)

}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Specialization {
  pub name: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Tier {
  pub name: String,
  pub id: i64,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ProfessionTier {
  pub skill_points: i64,
  pub max_skill_points: i64,
  pub tier: Tier,
  pub known_recipes: Option<Vec<Profession>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ProfessionInfo {
  pub key: Value,
  pub name: String,
  pub id: i64,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Profession {
  pub profession: Option<ProfessionInfo>,
  pub tiers: Option<Vec<ProfessionTier>>,
  pub specialization: Option<Specialization>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CharacterProfessions {
  pub _links: Option<Value>,
  pub character: Option<Value>,
  pub primaries: Vec<Profession>,
  pub secondaries: Vec<Profession>,
}