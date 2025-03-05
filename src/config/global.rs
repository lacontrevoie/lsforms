use crate::config::structs::{Config, Hosts, MailTemplates};

use once_cell::sync::OnceCell;
use regex::Regex;

pub const CONFIG_FILE: &str = "./config.toml";

pub const CONFIG_VERSION: u8 = 1;

pub static CONFIG: OnceCell<Config> = OnceCell::new();

pub const HOSTS_FILE: &str = "./hosts.json";

pub static HOSTS: OnceCell<Hosts> = OnceCell::new();

pub const MAIL_TEMPLATES_FOLDER: &str = "./mails/";

pub static MAIL_TEMPLATES: OnceCell<MailTemplates> = OnceCell::new();

#[cfg(feature = "static-files")]
pub const ASSETS_FOLDER: &str = "./assets/";

// BEHOLD!! the evil email regex. From here, actually:
// https://html.spec.whatwg.org/multipage/input.html#email-state-(type=email)
pub static RE_EMAIL: OnceCell<Regex> = OnceCell::new();
