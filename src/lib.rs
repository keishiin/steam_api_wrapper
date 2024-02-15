
pub mod actions;

const BASE_URL: &str = "https://api.steampowered.com";

pub struct Steam {
    api_key: String
}

impl Steam {
    pub fn new(api_key: &str) -> Steam {
        Steam {
            api_key: api_key.to_string(),
        }
    }
}