use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};

use crate::{
    generics::{BASE_URL, GET_SCHEMA_FOR_GAME, ISTEAM_USER_STATS, VERSION_V2},
    helpers::make_api_call::{api_call, FunctionResult},
    Steam,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Achievement {
    #[serde(rename = "name")]
    pub achievement_name: String,
    #[serde(rename = "defaultvalue")]
    pub default_value: u32,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub hidden: u32,
    pub description: Option<String>,
    pub icon: String,
    pub icongray: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AvailableGameStats {
    pub achievements: Vec<Achievement>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Game {
    #[serde(rename = "gameName")]
    pub game_name: String,
    #[serde(rename = "gameVersion")]
    pub game_version: String,
    #[serde(rename = "availableGameStats")]
    pub available_game_stats: AvailableGameStats,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Root {
    pub game: Game,
}

impl Steam {
    pub async fn get_schema_for_game(&self, app_id: u32) -> Result<Root, Error> {
        let query = format!("?key={}&appid={}", self.api_key, app_id);
        let url = format!(
            "{}/{}/{}/{}/{}",
            BASE_URL, ISTEAM_USER_STATS, GET_SCHEMA_FOR_GAME, VERSION_V2, query
        );

        match api_call::<Root>(url).await {
            FunctionResult::Success(response) => Ok(response),
            FunctionResult::Error(err) => Err(err),
        }
    }
}
