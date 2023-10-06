use once_cell::sync::OnceCell;

use crate::config::structs::{Config, Stars};

pub const CONFIG_FILE: &str = "./config.toml";
pub const STARS_FILE: &str = "./static/data/stars.json";

pub const CONFIG_VERSION: u8 = 1;

// initializing configuration
pub static CONFIG: OnceCell<Config> = OnceCell::new();

// initializing stars.json file
pub static STARS: OnceCell<Stars> = OnceCell::new();
