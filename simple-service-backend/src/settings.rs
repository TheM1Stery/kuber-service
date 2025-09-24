use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Server {
    pub address: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Settings {
    pub server: Server,
}

impl Settings {
    pub fn try_new() -> anyhow::Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("config/default").required(false))
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()?;

        config.try_deserialize()
    }
}
