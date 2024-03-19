use anyhow::Result;
use serde::__private::Ok;
use serde::{Deserialize, Serialize};

use crate::{
    generics::{BASE_URL, GET_RECENTLY_PLAYED_GAMES, IPLAYER_SERVICE, VERSION_V1},
    helpers::make_api_call::{api_call, FunctionResult},
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

        // Call the api_call function
        match api_call::<RecentlyPlayedSummary>(url).await {
            FunctionResult::Success(response) => Ok(response.response),
            FunctionResult::Error(err) => Err(err),
        }
    }
}
