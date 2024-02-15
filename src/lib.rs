mod generics;
pub mod services;

pub struct Steam {
    api_key: String,
}

impl Steam {
    pub fn new(api_key: &str) -> Steam {
        Steam {
            api_key: api_key.to_string(),
        }
    }
}
