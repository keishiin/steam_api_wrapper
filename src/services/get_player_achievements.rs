/*
    This function currently only gets the achievements for one game at aa time

    TODO:
        make it so the fucntions can do batch requests
*/

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    generics::{BASE_URL, GET_PLAYER_ACHIEVEMENTS, ISTEAM_USER_STATS, VERSION_V1},
    helpers::make_api_call::{api_call, FunctionResult},
    Steam,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Achievements {
    #[serde(rename = "apiname")]
    pub api_name: String,

    pub achieved: u8,

    pub name: Option<String>,

    pub description: Option<String>,

    #[serde(rename = "unlocktime")]
    pub unlock_time: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerAchievementInfo {
    #[serde(rename = "gameName")]
    pub game_name: Option<String>,

    #[serde(rename = "steamID")]
    pub steam_id: Option<String>,

    pub achievements: Option<Vec<Achievements>>,

    pub success: bool,

    pub error: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerStats {
    #[serde(rename = "playerstats")]
    player_stats: PlayerAchievementInfo,
}

impl Steam {
    pub async fn get_player_achievements(
        &self,
        steam_id: u64,
        app_id: u64,
    ) -> Result<PlayerAchievementInfo> {
        let query = format!(
            "?key={}&steamid={}&appid={}",
            &self.api_key, steam_id, app_id
        );
        let url = format!(
            "{}/{}/{}/{}/{}",
            BASE_URL, ISTEAM_USER_STATS, GET_PLAYER_ACHIEVEMENTS, VERSION_V1, query
        );

        // Call the api_call function
        match api_call::<PlayerStats>(url).await {
            FunctionResult::Success(response) => Ok(response.player_stats),
            FunctionResult::Error(err) => Err(err),
        }
    }
}
