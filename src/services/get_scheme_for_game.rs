use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};

use crate::{
    generics::{BASE_URL, GET_SCHEMA_FOR_GAME, ISTEAM_USER_STATS, VERSION_V2},
    helpers::make_api_call::{api_call, FunctionResult},
    Steam,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Achievement {
    pub name: String,
    pub defaultvalue: u32,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub hidden: u32,
    pub description: String,
    pub icon: String,
    pub icongray: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AvailableGameStats {
    pub achievements: Vec<Achievement>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    #[serde(rename = "gameName")]
    pub game_name: String,
    #[serde(rename = "gameVersion")]
    pub game_version: String,
    #[serde(rename = "availableGameStats")]
    pub available_game_stats: AvailableGameStats,
}

impl Steam {
    pub async fn get_schema_for_game(&self, app_id: u32) -> Result<Game, Error> {
        let query = format!("?key={}&appid={}", self.api_key, app_id);
        let url = format!(
            "{}/{}/{}/{}/{}",
            BASE_URL, ISTEAM_USER_STATS, GET_SCHEMA_FOR_GAME, VERSION_V2, query
        );

        match api_call::<Game>(url).await {
            FunctionResult::Success(response) => Ok(response),
            FunctionResult::Error(err) => Err(err)
        }
    }
}
