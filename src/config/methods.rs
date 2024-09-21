use crate::config::global::{CONFIG, CONFIG_FILE, CONFIG_VERSION};
use crate::config::structs::{Config};
use crate::errors::{throw, ErrorKind, ServerError};

use std::fs::File;
use std::io::Read;

// Initialize CONFIG
pub fn read_config() {
    CONFIG
        .set(Config::init())
        .ok()
        .expect("could not load config");
}

impl Config {
    pub fn init() -> Self {
        let mut config: Config = toml::from_str(&init_from_file(CONFIG_FILE).unwrap()).unwrap();
        config
    }

    pub fn global() -> &'static Config {
        CONFIG.get().expect("TODO config not initialized")
    }

    pub fn check(&self) {
        // check config version
        if self.config_version != CONFIG_VERSION {
            eprintln!("Your configuration file is obsolete! Please update it using config.toml.sample and update its version to {CONFIG_VERSION}.");
            panic!();
        }
    }
}

pub fn init_from_file(filename: &str) -> Result<String, ServerError> {
    let mut conffile =
        File::open(filename).map_err(|e| throw(ErrorKind::FileNotFound, e.to_string()))?;

    let mut confstr = String::new();
    conffile
        .read_to_string(&mut confstr)
        .map_err(|e| throw(ErrorKind::FileReadFail, e.to_string()))?;

    Ok(confstr)
}
