use crate::config::global::{CONFIG, CONFIG_FILE, CONFIG_VERSION, HOSTS, HOSTS_FILE, MAIL_TEMPLATES, MAIL_TEMPLATES_FOLDER, RE_EMAIL};
use crate::config::structs::{Config, MailTemplates, Language, Hosts};
use crate::errors::{throw, ErrorKind, ServerError};

use std::fs::File;
use std::io::Read;
use regex::Regex;

// Initialize CONFIG
pub fn read_config() {

    println!("Loading configuration file {CONFIG_FILE}...");
    CONFIG
        .set(toml::from_str(&init_from_file(CONFIG_FILE).unwrap()).unwrap())
        .ok()
        .expect("could not load TOML config file");

    println!("Loading hosts file {HOSTS_FILE}...");
    HOSTS 
        .set(serde_json::from_str(&init_from_file(HOSTS_FILE).unwrap()).unwrap())
        .ok()
        .expect("could not load hosts file");
    
    println!("Loading mail templates in {MAIL_TEMPLATES_FOLDER}...");
    MAIL_TEMPLATES 
        .set(init_mail_template(MAIL_TEMPLATES_FOLDER).unwrap())
        .expect("could not load config");

    let email_regex = Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap();
    RE_EMAIL
        .set(email_regex)
        .expect("could not load email regex");
}

fn init_mail_template(tpl_folder: &str) -> Result<MailTemplates, ServerError> {
    let mut tpl_list: MailTemplates = MailTemplates::new();

    tpl_list.insert(Language::Fr, init_from_file(&format!("{}{}.txt", tpl_folder, "fr"))?);
    tpl_list.insert(Language::En, init_from_file(&format!("{}{}.txt", tpl_folder, "en"))?);

    Ok(tpl_list)
}

pub fn load_config() -> &'static Config {
    CONFIG.get().expect("config not initialized")
}

pub fn load_hosts() -> &'static Hosts {
    HOSTS.get().expect("hosts not initialized")
}

pub fn load_templates() -> &'static MailTemplates {
    MAIL_TEMPLATES.get().expect("mail templates not initialized")
}

pub fn load_re_email() -> &'static Regex {
    RE_EMAIL.get().expect("hosts not initialized")
}
impl Config {
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
