use std::fs;
use serde::{ Serialize, Deserialize };
use toml;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscordConfig {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub uri: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameConfig {
    pub tps: u32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub discord: DiscordConfig,
    pub database: DatabaseConfig,
    pub game: GameConfig
}


impl Config {
    pub fn new() -> Self {
        let result = fs::read_to_string("./Config.toml");
        let config_toml: Config;

        match result {
            Ok(r) => config_toml = toml::from_str(&r).unwrap_or_else(|e| panic!("Problem with reading config.toml! {e}")),
            Err(e) => panic!("Problem with finding config.toml: {e}")
        }

        config_toml
    }
}
