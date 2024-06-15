use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Stars(pub Vec<StarsList>);

#[derive(Serialize, Deserialize)]
pub struct StarsList {
    pub startype: i32,
    pub path: String,
    pub price: u16,
    pub size_pct: u8,
}

#[derive(Serialize, Deserialize)]
pub struct ConfGeneral {
    pub listening_address: String,
    pub listening_port: u16,
    pub hostname: String,
    pub database_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConfMail {
    pub mailserver_address: String,
    pub sender_email: String,
    pub sender_password: String,
    pub send_as: String,
    pub templates: Vec<MailTemplate>,
}

#[derive(Serialize, Deserialize)]
pub struct MailTemplate {
    pub id: i32,
    pub name: String,
    pub title: String,
    pub path: String,
    pub body: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ConfHelloAsso {
    pub callback_key: String,
    pub field_username: String,
    pub field_message: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub config_version: u8,
    pub general: ConfGeneral,
    pub mail: ConfMail,
    pub helloasso: ConfHelloAsso,
}
