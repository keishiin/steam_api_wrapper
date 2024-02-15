use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};

use crate::{
    generics::{BASE_URL, GET_RECENTLY_PLAYED_GAMES, IPLAYER_SERVICE, VERSION_V1},
    Steam,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Game {
    #[serde(rename = "appid")]
    pub app_id: u64,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "playtime_2weeks")]
    pub recently_played: u64,

    #[serde(rename = "playtime_forever")]
    pub overall_plat_time: u64,

    pub img_icon_url: String,

    pub img_logo_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Games {
    pub total_count: u64,
    pub games: Vec<Game>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RecentlyPlayedSummary {
    response: Games,
}

impl Steam {
    pub async fn get_recently_played_games(&self, steam_id: u64) -> Result<Games> {
        let query = format!("?key={}&steamid={}", &self.api_key, steam_id);
        let url = format!(
            "{}/{}/{}/{}/{}",
            BASE_URL, IPLAYER_SERVICE, GET_RECENTLY_PLAYED_GAMES, VERSION_V1, query
        );

        let resp = reqwest::get(url).await?;

        match resp.status() {
            reqwest::StatusCode::OK => {
                let json_response: Value = resp.json().await?;
                println!("{:?}", json_response);
                let response: RecentlyPlayedSummary = match from_value(json_response.to_owned()) {
                    Ok(res) => res,
                    Err(err) => {
                        return Err(anyhow!(
                            "Failed to parse response into PlayerResponse: {}",
                            err
                        ));
                    }
                };
                Ok(response.response)
            }
            status_code => Err(anyhow!("Expected 200 Status, got {}", status_code)),
        }
    }
}
