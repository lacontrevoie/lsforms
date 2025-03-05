use serde::{Deserialize};
use std::collections::HashMap;

pub type MailTemplates = HashMap<Language, String>;
pub type Hosts = HashMap<String, HostConfig>;

/* config.toml settings */

#[derive(Deserialize)]
pub struct ConfGeneral {
    pub listening_address: String,
    pub listening_port: u16,
    #[cfg(feature = "templates")]
    pub hostname: String,
    pub verbose_level: VerboseLevel,
}

#[derive(Deserialize)]
pub struct ConfCaptcha {
    pub secret: String,
    pub complexity: u64,
    pub expires: i64,
}

#[derive(Deserialize)]
pub struct ConfMail {
    pub mailserver_address: String,
    pub sender_email: String,
    pub sender_password: String,
    pub send_as: String,
    pub contact_spam: String,
    pub mail_signature: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub config_version: u8,
    pub general: ConfGeneral,
    pub captcha: ConfCaptcha,
    pub mail: ConfMail,
}

/* hosts.json settings */

#[derive(Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HostInputKind {
    Text, TextArea, Email, Select
}

#[derive(Deserialize)]
pub struct HostSettings {
    pub maxlength: Option<usize>,
    pub options: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct HostInput {
    pub display_name: String,
    pub name: String,
    pub kind: HostInputKind,
    pub required: bool,
    pub settings: HostSettings,
}

#[derive(Deserialize, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Fr, En
}

#[derive(Deserialize)]
pub struct HostConfig {
    pub language: Language,
    pub recipient: String,
    pub enable_captcha: bool,
    pub inputs: Vec<HostInput>,
}

/* verbosity settings */

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VerboseLevel {
    Info,
    Notice,
    Warn,
    Crit,
}
