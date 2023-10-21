use crate::Config;
use crate::config::structs::MailTemplate;
use crate::config::methods::init_from_file;
use crate::errors::{ErrorKind, ServerError, throw};

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::address::AddressError;
use lettre::{Message, SmtpTransport, Transport};

pub fn send(
    tpl: &MailTemplate,
    dest_address: String,
    dest_name: String,
    dest_link: String,
    ) -> Result<(), ServerError> {
    let config = Config::global();

    let mut msg_body = init_from_file(&tpl.path)?;

    msg_body = if !dest_name.is_empty() {
        msg_body.replace("{{NAME}}", &format!(" {}", dest_name))
    } else {
        msg_body.replace("{{NAME}}", "")
    };
    
    msg_body = msg_body.replace("{{LINK}}", &dest_link);

    let m = Message::builder()
        .from(config.mail.send_as.parse().map_err(|e: AddressError| {
            throw(ErrorKind::EmailFromParseFail, e.to_string())
        })?)
        .to(dest_address.parse().map_err(|e: AddressError| {
            throw(ErrorKind::EmailToParseFail, e.to_string())
        })?)
        .subject(&tpl.name)
        .header(ContentType::TEXT_PLAIN)
        .body(msg_body).map_err(|e| {
            throw(ErrorKind::EmailBodyParseFail, e.to_string())
        })?;

    let creds = Credentials::new(config.mail.sender_email.clone(), config.mail.sender_password.clone());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&config.mail.mailserver_address)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    mailer.send(&m).map_err(|e| {
        throw(ErrorKind::EmailSendFail, e.to_string())
    })?;

    Ok(())
}
