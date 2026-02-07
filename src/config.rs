use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub aspect_ratio: f64,
    pub image_width: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

impl Settings {
    pub fn new() -> Self {
        let config_str = fs::read_to_string("config.toml").expect("Failed to read config.toml");
        let settings: Settings = toml::from_str(&config_str).expect("Failed to parse config.toml");
        settings
    }
}
