use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    generics::{BASE_URL, GET_OWNED_GAMES, IPLAYER_SERVICE, VERSION_V1},
    helpers::make_api_call::{api_call, FunctionResult},
    Steam,
};

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct OwnedGames {
    pub game_count: u32,
    pub games: Vec<OwnedGame>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetOwnedGamesResponse {
    pub response: OwnedGames,
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

        // Call the api_call function
        match api_call::<GetOwnedGamesResponse>(url).await {
            FunctionResult::Success(response) => Ok(response.response),
            FunctionResult::Error(err) => Err(err),
        }
    }
}
