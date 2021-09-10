use std::env;
use config::{ConfigError, Config, File, Environment};

const ENV_VAR_PREFIX: &str = "app";

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct App {
    pub http_port: u16,
    https_port: u32,
    http_workers: u32,
    pub version: String,
    pub environment: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Settings {
    pub app: App
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();
        s.merge(File::with_name("config/default.json"))?;
        let env = env::var("APP_INSTANCE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("config/{}.json", env)).required(false))?;
        s.merge(File::with_name("config/local.json").required(false))?;
        s.merge(Environment::with_prefix(ENV_VAR_PREFIX))?;
        s.try_into()
    }
}