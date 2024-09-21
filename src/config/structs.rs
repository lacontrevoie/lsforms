use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConfGeneral {
    pub listening_address: String,
    pub listening_port: u16,
    pub hostname: String,
    pub database_url: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub config_version: u8,
    pub general: ConfGeneral,
}
