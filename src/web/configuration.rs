use std::env;
use std::path::PathBuf;

use config::{Config as oConfig, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Application {
    pub host: String,
    pub port: usize,
}

#[derive(Debug, Deserialize)]
pub struct Postgres {
    pub host: String,
    pub port: usize,
    pub username: String,
    pub password: String,
    pub db: String,
}

impl Postgres {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.db
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct Authentication {
    pub secret: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub application: Application,
    pub database: Postgres,
    pub authentication: Authentication,
}

impl Config {
    pub fn new(base_path: PathBuf) -> Result<Self, ConfigError> {
        // init logger
        log4rs::init_file(base_path.join("configuration/logger.yml"), Default::default()).unwrap();

        info!("logger init finished");

        let mut s = oConfig::new();

        // Start off by merging in the "default" configuration file
        let path = base_path.join("configuration/base");
        s.merge(File::from(path))?;

        // Detect the running environment
        let environment = env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".into());

        // Add in environment-specific settings (optional)
        let path = base_path.join(&format!("configuration/{}", environment));
        s.merge(File::from(path).required(false))?;

        // Add in settings from environment variables (with a prefix of APP and '_' as separator)
        // Eg.. `APP_APPLICATION_PORT=5001 would set `Settings.application.port`
        s.merge(Environment::with_prefix("app").separator("_"))?;

        info!("configuration init finished");

        // Deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}
