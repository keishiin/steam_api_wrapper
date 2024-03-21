use crate::{
    generics::{BASE_URL, GET_STEAM_LEVEL, IPLAYER_SERVICE, VERSION_V1},
    helpers::make_api_call::{api_call, FunctionResult},
    Steam,
};
use anyhow::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SteamPlayerLevel {
    pub player_level: u16,
}

#[derive(Deserialize, Serialize)]
pub struct SteamPlayerLevelRespone {
    response: SteamPlayerLevel,
}

impl Steam {
    pub async fn get_steam_level(&self, steam_id: u64) -> Result<SteamPlayerLevel, Error> {
        let query = format!("?key={}&steamid={}", self.api_key, steam_id);

        let url = format!(
            "{}/{}/{}/{}/{}",
            BASE_URL, IPLAYER_SERVICE, GET_STEAM_LEVEL, VERSION_V1, query
        );

        match api_call::<SteamPlayerLevelRespone>(url).await {
            FunctionResult::Success(response) => Ok(response.response),
            FunctionResult::Error(err) => Err(err),
        }
    }
}
