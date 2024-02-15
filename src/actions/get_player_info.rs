use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};

use crate::{generics::{INTERFACE, VERSION, METHOD, BASE_URL}, Steam};

// player profile object returned by api
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    #[serde(rename = "steamid")]
    pub steam_id: String,

    #[serde(rename = "communityvisibilitystate")]
    pub community_visibility_state: u8,

    #[serde(rename = "profilestate")]
    pub profile_state: u8,

    #[serde(rename = "personaname")]
    pub persona_name: String,

    #[serde(rename = "lastlogoff")]
    pub last_logoff:  Option<u64>,

    #[serde(rename = "commentpermission")]
    pub comment_permission: Option<u8>,

    #[serde(rename = "profileurl")]
    pub profile_url: String,

    #[serde(rename = "avatar")]
    pub avatar: String,

    #[serde(rename = "avatarhash")]
    pub avatar_hash: String,

    #[serde(rename = "avatarmedium")]
    pub avatar_medium: String,

    #[serde(rename = "avatarfull")]
    pub avatar_full: String,

    #[serde(rename = "personastate")]
    pub persona_state:  Option<u8>,

    #[serde(rename = "realname")]
    pub real_name:  Option<String>,

    #[serde(rename = "primaryclanid")]
    pub primary_clan_id: String,

    #[serde(rename = "timecreated")]
    pub time_created:  Option<u64>,

    #[serde(rename = "personastateflags")]
    pub persona_state_flags: u8,

    #[serde(rename = "gameextrainfo")]
    pub game_extra_info: Option<String>,

    #[serde(rename = "gameid")]
    pub game_id: Option<String>,

    #[serde(rename = "loccountrycode")]
    pub loc_country_code: Option<String>,

    #[serde(rename = "locstatecode")]
    pub loc_state_code: Option<String>,

    #[serde(rename = "loccityid")]
    pub loc_city_id: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Players {
    players: Vec<Player>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerResponse {
    response: Players
}

impl Steam {
    pub async fn get_player_summaries(&self, steam_id: u64) -> Result<Vec<Player>> {
        let query = format!("?key={}&steamids={}", &self.api_key, steam_id);
        let url = format!("{}/{}/{}/{}/{}", BASE_URL, INTERFACE, METHOD, VERSION, query);

        let resp = reqwest::get(url).await?;

        match resp.status() {
            reqwest::StatusCode::OK => {
                let json_response: Value = resp.json().await?;
                println!("{:?}", json_response);
                let response: PlayerResponse = match from_value(json_response.to_owned()) {
                    Ok(res) => res,
                    Err(err) => {
                        return Err(anyhow!(
                            "Failed to parse response into PlayerResponse: {}",
                            err
                        ));
                    }
                };
                Ok(response.response.players)
            }
            status_code => {
                Err(anyhow!("Expected 200 Status, got {}", status_code))
            }
        }
    }
}