use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};

use crate::{
    generics::{BASE_URL, GET_OWNED_GAMES, IPLAYER_SERVICE, VERSION_V1},
    Steam,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OwnedGame {
    #[serde(rename = "appid")]
    pub app_id: u64,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "playtime_2weeks")]
    pub recent_play_time: Option<u64>,

    #[serde(rename = "playtime_forever")]
    pub total_play_time: Option<u64>,

    pub img_icon_url: Option<String>,

    pub img_logo_url: Option<String>,

    pub has_community_visible_stats: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OwnedGames {
    pub game_count: u32,
    pub games: Vec<OwnedGame>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GetOwnedGamesResponse {
    response: OwnedGames,
}

impl Steam {
    pub async fn get_owned_games(
        &self,
        steam_id: u64,
        include_app_info: bool,
        include_played_free_games: bool,
    ) -> Result<OwnedGames> {
        let query = format!(
            "?key={}&steamid={}&include_appinfo={}&include_played_free_games={}",
            &self.api_key, steam_id, include_app_info, include_played_free_games
        );
        let url = format!(
            "{}/{}/{}/{}/{}",
            BASE_URL, IPLAYER_SERVICE, GET_OWNED_GAMES, VERSION_V1, query
        );

        let resp = reqwest::get(url).await?;

        match resp.status() {
            reqwest::StatusCode::OK => {
                let json_response: Value = resp.json().await?;
                println!("{:?}", json_response);
                let response: GetOwnedGamesResponse = match from_value(json_response.to_owned()) {
                    Ok(res) => res,
                    Err(err) => {
                        return Err(anyhow!(
                            "Failed to parse response into GetOwnedGamesResponse: {}",
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

