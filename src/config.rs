use serde::Deserialize;
use serde_json;
use std::fs;
use std::{
    env,
    path::{Path, PathBuf},
};

const CONFIG_DIR: &str = ".config/fomc";
const CONFIG_FILE: &str = "config.json";

#[derive(Deserialize)]
struct Config {
    api_key: String,
}

pub fn config_path() -> PathBuf {
    let home_dir = env::var("HOME").expect("HOME environment variable not set");
    Path::new(&home_dir).join(CONFIG_DIR).join(CONFIG_FILE)
}

pub fn read_config() -> Result<String, Box<dyn std::error::Error>> {
    let path = config_path();
    let config_data = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&config_data)?;
    Ok(config.api_key)
}
