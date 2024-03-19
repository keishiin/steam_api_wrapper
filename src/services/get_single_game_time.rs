/*
need to find more information on this METHOD

need:
    request -> structure
    response -> structure
*/

use crate::{
    generics::{BASE_URL, GET_SINGLE_GAME_PLAYTIME, IPLAYER_SERVICE, VERSION_V1},
    Steam,
};

impl Steam {
    pub async fn get_single_game_time(&self, steam_id: u64, app_id: u32) {
        let query = format!(
            "?key={}&steamid={}&appid={}",
            self.api_key, steam_id, app_id
        );
        let url = format!(
            "{}/{}/{}/{}/{}",
            BASE_URL, IPLAYER_SERVICE, GET_SINGLE_GAME_PLAYTIME, VERSION_V1, query
        );

        println!("{}", url);

        //match api_call(url) {}
        todo!()
    }
}
