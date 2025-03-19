use config::{Config, ConfigError, File};
use serde::Deserialize;
//use serde_derive::Deserialize;
use std::sync::OnceLock;

// #[derive(Debug, Deserialize)]
// #[allow(unused)]
// struct Global {
    
// }

static CONFIG: OnceLock<Settings> = OnceLock::new();


#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Browser {
   pub enabled: bool,
    start_url: String,
    start_cmd: String,

}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Streambot {
    enabled: bool,
    channel: String,
    username: String,
    command_symbol: String,
    client_id: String,
    access_token: String,
    refresh_token: String,
}


impl Streambot {
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    
    pub fn channel(&self) -> &str {
        &self.channel
    }
    
    pub fn username(&self) -> &str {
        &self.username
    }
    
    pub fn command_symbol(&self) -> &str {
        &self.command_symbol
    }
    
    pub fn client_id(&self) -> &str {
        &self.client_id
    }
    
    pub fn access_token(&self) -> &str {
        &self.access_token
    }
    
    pub fn refresh_token(&self) -> &str {
        &self.refresh_token
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
 //   pub(crate) global: Global,
    pub(crate) browser: Browser,
    pub(crate) streambot: Streambot,
}

impl Settings {
pub fn new() -> &'static Settings {
    CONFIG.get_or_init(|| 
        Config::builder()
        .add_source(File::with_name("config"))
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap()
    )
}

pub fn streambot(&self) -> &Streambot {
    &self.streambot
}


}

