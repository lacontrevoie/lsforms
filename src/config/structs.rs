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
    pub contact_form_dest: String,
    pub external_address: String,
    pub sender_email: String,
    pub sender_password: String,
    pub send_as: String,
    pub sender_name: String,
    pub contact_address: String,
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
