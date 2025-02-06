use std::sync::OnceLock;
use std::fs;
use std::path::Path;
use toml::Value;

static CONFIG: OnceLock<Value> = OnceLock::new();

fn load_config() -> &'static Value {
    CONFIG.get_or_init(|| {
        let config_content = fs::read_to_string(Path::new("config.toml"))
            .expect("Failed to read config.toml");
        config_content.parse::<Value>()
            .expect("Failed to parse config.toml")
    })
}

pub fn get_config() -> &'static Value {
    load_config()

}



