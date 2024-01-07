use crate::config::structs::MailTemplate;
use crate::errors::{throw, ErrorKind, ServerError};
use crate::Config;

use lettre::address::AddressError;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn send(
    tpl: &MailTemplate,
    dest_address: &str,
    dest_name: &str,
    dest_link: &str,
) -> Result<(), ServerError> {
    let config = Config::global();

    let mut msg_body = tpl.body.clone().unwrap();

    msg_body = if dest_name.is_empty() {
        msg_body.replace("{{NAME}}", "")
    } else {
        msg_body.replace("{{NAME}}", &format!(" {dest_name}"))
    };

    msg_body = msg_body.replace("{{LINK}}", dest_link);

    let m =
        Message::builder()
            .from(
                config.mail.send_as.parse().map_err(|e: AddressError| {
                    throw(ErrorKind::EmailFromParseFail, e.to_string())
                })?,
            )
            .to(dest_address
                .parse()
                .map_err(|e: AddressError| throw(ErrorKind::EmailToParseFail, e.to_string()))?)
            .subject(&tpl.title)
            .header(ContentType::TEXT_PLAIN)
            .body(msg_body)
            .map_err(|e| throw(ErrorKind::EmailBodyParseFail, e.to_string()))?;

    let creds = Credentials::new(
        config.mail.sender_email.clone(),
        config.mail.sender_password.clone(),
    );

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&config.mail.mailserver_address)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    mailer
        .send(&m)
        .map_err(|e| throw(ErrorKind::EmailSendFail, e.to_string()))?;

    Ok(())
}
