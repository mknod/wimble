use config::{Config, File};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct GlobalConfig {
    pub placeholder: bool,
}

#[derive(Debug, Deserialize)]
pub struct StreambotConfig {
    pub enabled: bool,
    pub channel: String,
    pub username: String,
    pub command_symbol: String,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ElementConfig {
    pub element: String,  // XPath query
    pub attribute: String, // Attribute to extract
    pub iframe: Option<String>,  // Optional iframe selector (by id or CSS)

}

#[derive(Debug, Deserialize)]
pub struct BrowserConfig {
    pub enabled: bool,
    pub start_url: String,
    pub goto: HashMap<String, String>,
    pub elements: HashMap<String, ElementConfig>, // Maps names to ElementConfig

}


#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub global: GlobalConfig,
    pub streambot: StreambotConfig,
    pub browser: BrowserConfig,
}

pub fn load_config() -> Result<AppConfig, config::ConfigError> {
    let settings = Config::builder()
        .add_source(File::with_name("config"))
        .build()?;
    
    settings.try_deserialize::<AppConfig>()
}