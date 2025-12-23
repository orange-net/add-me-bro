use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct LogConfig {
    /*
     * LOG LEVEL mapping
     * 10 -> verbose
     * 20 -> debug
     * 30 -> info
     * 40 -> warn
     * 50 -> error
     * 60 -> critical
     * 70 -> Fatal
     */
    level: i32,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u32,
    pub username: String,
    pub password: String,
    pub database_name: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ServiceConfig {
    pub log: LogConfig,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

impl ServiceConfig {
    // TODO: Add run mode configuraton and based on choose which file to override with

    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::with_name("conf.default.toml"))
            .add_source(config::File::with_name("local-conf.toml").required(false))
            .add_source(config::Environment::with_prefix("ADD_ME_BRO_SERVICE").separator("_"))
            .build()?;

        let cfg = s.try_deserialize::<Self>()?;
        println!("Config {cfg:?}");

        Ok(cfg)
    }
}
