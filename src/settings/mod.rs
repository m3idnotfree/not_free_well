use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub server: Server,
    pub twitch: TwitchSettings,
}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub url: String,
    pub port: String,
}

#[derive(Deserialize, Debug)]
pub struct TwitchSettings {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
}

pub fn settings() -> Result<Settings, ConfigError> {
    let settings = Config::builder()
        .add_source(config::File::with_name("settings.toml"))
        .add_source(config::File::with_name("twitch.toml"))
        .build()?;

    settings.try_deserialize::<Settings>()
}
