use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::Deserialize;

pub const ID_CONFIG_FILE_NAME: &str = "id_config.toml";

#[derive(Deserialize, Debug)]
pub struct IDConfig {
    #[serde(default = "default_true")]
    pub vanilla: bool,

    #[serde(default)]
    pub data: IDConfigData,
    #[serde(default)]
    pub options: IDConfigOptions,
}

#[derive(Deserialize, Debug, Default)]
pub struct IDConfigData {
    #[serde(default = "default_path")]
    pub id_data: PathBuf,
}

#[derive(Deserialize, Debug, Default)]
pub struct IDConfigOptions {
    #[serde(default = "default_width")]
    pub width: usize,
    #[serde(default = "default_height")]
    pub height: usize,
    #[serde(default = "default_true")]
    pub fullscreen: bool,
}

fn default_width() -> usize {
    320
}

fn default_height() -> usize {
    200
}

fn default_path() -> PathBuf {
    let mut path = PathBuf::new();
    path.push("./");
    path
}

fn default_true() -> bool {
    true
}

pub fn read_id_config() -> Result<IDConfig, String> {
    let conf_file = Path::new(ID_CONFIG_FILE_NAME);
    if conf_file.exists() {
        let content = fs::read_to_string(conf_file).map_err(|e| e.to_string())?;
        let config: IDConfig = toml::from_str(&content).map_err(|e| e.to_string())?;
        Ok(config)
    } else {
        default_id_config()
    }
}

pub fn default_id_config() -> Result<IDConfig, String> {
    toml::from_str("vanilla = true").map_err(|e| e.to_string())
}
