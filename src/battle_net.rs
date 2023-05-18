use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct OAuthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GuildRoster {
    pub _links: Value,
    pub guild: Guild,
    pub members: Vec<CharacterRank>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Guild {
    key: Value,
    pub name: String,
    id: u64,
    pub realm: Realm,
    faction: Value
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Realm {
    key: Value,
    pub name: String,
    id: u64,
    slug: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterRank {
    pub character: Character,
    pub rank: i32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Character {
    pub key: Value,
    pub name: String,
    pub id: u64,
    pub realm: Value,
    pub level: u32,
    pub playable_class: Value,
    pub playable_race: Value,
}

/// To retrieve a token, you need to provide your client_id and client_secret as well as a region (US, EU, APAC or CN)
///
/// ```rust
/// let token = battle_net_oauth::get_oauth_token("client_id", "client_secret", "region");
/// ```
pub async fn get_oauth_token(
    client_id: &str,
    client_secret: &str,
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


pub async fn get_roster(access_token: String) -> Result<GuildRoster, Box<dyn std::error::Error + Send + Sync>> {
    let header = format!("Bearer {}", access_token);
    let url = "https://us.api.blizzard.com/data/wow/guild/proudmoore/power-word-taint/roster?namespace=profile-us&locale=en_US";

    let resp = reqwest::Client::new()
        .get(url)
        .header(reqwest::header::AUTHORIZATION, header)
        .send()
        .await?
        .json()
        .await?;

    Ok(resp)
}