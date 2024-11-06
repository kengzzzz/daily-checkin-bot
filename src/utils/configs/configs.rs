use std::{error::Error, fs};

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Config {
    pub server_utc: i32,
    pub delay_minute: i64,
    pub randomize: bool,
    pub random_range: u64,
    pub scheduler_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server_utc: 8,
            delay_minute: 0,
            randomize: false,
            random_range: 3600,
            scheduler_name: "HoyolabCheckInBot".to_string(),
        }
    }
}

pub fn read_cookie() -> Result<String, Box<dyn Error>> {
    let f = fs::read_to_string("cookie.txt")?;
    Ok(f.trim().to_string())
}

pub fn read_config() -> Result<Config, Box<dyn Error>> {
    let config_content = fs::read_to_string("config.json").unwrap_or_else(|_| "{}".to_string());
    let config: Config = serde_json::from_str(&config_content)?;
    Ok(config)
}
