use crate::config::global::{CONFIG_VERSION, CONFIG_FILE, CONFIG, STARS, STARS_FILE};
use crate::config::structs::{Config, Stars};

use std::collections::HashMap;
use std::fs::File;
use std::fmt;
use std::io::Read;

// Initialize CONFIG
pub fn read_config() {
    CONFIG
        .set(Config::init())
        .ok()
        .expect("could not load config");
   STARS
        .set(Stars::init())
        .ok()
        .expect("could not load stars");
}

impl Config {
    pub fn init() -> Self {
        toml::from_str(&init_from_file(CONFIG_FILE)).unwrap()
    }

    pub fn global() -> &'static Config {
        CONFIG.get().expect("TODO config not initialized")
    }

    pub fn check(&self) {
        // check config version
        if self.config_version != CONFIG_VERSION {
            eprintln!("Your configuration file is obsolete! Please update it using config.toml.sample and update its version to {}.", CONFIG_VERSION);
            panic!();
        }
    }
}

fn init_from_file(filename: &'static str) -> String {
    let mut conffile = File::open(filename).expect(&format!(
        r#"File {} not found. This software will now exit."#,
        filename
    ));
    let mut confstr = String::new();
    conffile
        .read_to_string(&mut confstr)
        .expect(&format!(
                "Couldn't read {} to string",
                filename
        ));
    confstr
}

impl Stars {
    pub fn init() -> Self {
        serde_json::from_str(&init_from_file(STARS_FILE)).unwrap()
    }
    pub fn global() -> &'static Stars {
        STARS.get().expect("Stars not initialized?!")
    }
}
