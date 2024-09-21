use once_cell::sync::OnceCell;
use crate::Config;

pub const CONFIG_FILE: &str = "./config.toml";

pub const CONFIG_VERSION: u8 = 1;

// initializing configuration
pub static CONFIG: OnceCell<Config> = OnceCell::new();
