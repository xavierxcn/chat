use std::env;
use std::fs::File;
use serde::{Serialize, Deserialize};
use anyhow::{bail, Result};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let ret = match (
            File::open("app.yml"),
            File::open("/etc/config/app.yml"),
            env::var("CHAT_CONFIG_PATH"),
        ) {
            (Ok(file), _, _) => serde_yaml::from_reader(file),
            (_,Ok(file), _) => serde_yaml::from_reader(file),
            (_, _, Some(path)) => serde_yaml::from_reader(File::open(path.as_str())?),
            _ => bail!("Config file not found"),
        };

        Ok(ret?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}
