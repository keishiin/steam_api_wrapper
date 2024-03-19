use anyhow::Ok;
use serde::{Deserialize, Serialize};

use crate::{generics::{BASE_URL, GET_PLAYER_ACHIEVEMENTS, ISTEAM_USER_STATS, VERSION_V2}, helpers::make_api_call::{api_call, FunctionResult}, Steam};


#[derive(Debug, Deserialize, Serialize)]
pub struct Achievement {
    pub apiname: String,
    pub achieved: u32,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerStats {
    #[serde(rename = "steamID")]
    pub steam_id: u64,
    #[serde(rename = "gameName")]
    pub game_name: String,
    pub achievements: Vec<Achievement>,
    pub success: bool,
}



impl Steam {
    pub async fn get_player_acheivements(&self, steam_id: u64, app_id: u32) -> Result<PlayerStats, anyhow::Error> {
        let query = format!("?key={}&steamid={}&appid={}", self.api_key, steam_id, app_id);
        let url = format!(
            "{}/{}/{}/{}/{}",
            BASE_URL, ISTEAM_USER_STATS, GET_PLAYER_ACHIEVEMENTS, VERSION_V2, query
        );

        match api_call::<PlayerStats>(url).await {
            FunctionResult::Success(response) => Ok(response),
            FunctionResult::Error(err) => Err(err)
        }
    }
}