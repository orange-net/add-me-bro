use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct LogConfig {
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
struct ServerConfig {
    host: String,
    port: i32,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ServiceConfig {
    log: LogConfig,
    server: ServerConfig,
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
