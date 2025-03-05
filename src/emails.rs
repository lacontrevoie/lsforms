use crate::config::structs::{Config, HostConfig, HostInputKind};
use crate::config::methods::load_templates;
use crate::errors::{throw, ErrorKind as EK, ServerError};

use lettre::address::AddressError;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, AsyncSmtpTransport, Tokio1Executor, AsyncTransport};
use std::collections::HashMap;


pub async fn deliver(
    config: &Config,
    client_ip: &str,
    host_name: &str,
    host_config: &HostConfig,
    form_values: &HashMap<String, String>
) -> Result<(), ServerError> {

    let mail_body = load_templates().get(&host_config.language)
        .ok_or_else(|| throw(EK::EmailLanguageNotFound, format!("defined language not found for {host_name} ({:?})", &host_config.language)))?
        .to_string();

    let form_data = fill_form_data(host_config, form_values)?;

    let subject = mail_body.lines().next()
        .ok_or_else(|| throw(EK::EmailLinesFail, format!("Could not read next line for {host_name}, {mail_body:?}")))?;
    
    let mail_body = mail_body
        .replace(subject, "")
        .replace("{{HOST}}", host_name)
        .replace("{{IP}}", client_ip)
        .replace("{{FORMDATA}}", &form_data)
        .replace("{{CONTACT}}", &config.mail.contact_spam)
        .replace("{{MAIL_SIGNATURE}}", &config.mail.mail_signature);

    let subject = subject
        .replace("{{SUBJECT}}", "")
        .replace("{{/SUBJECT}}", "");

    // build the message body and headers
    let m =
        Message::builder()
            .from(
                config.mail.send_as.parse().map_err(|e: AddressError| {
                    throw(EK::EmailFromParseFail, e.to_string())
                })?,
            )
            .to(host_config.recipient
                .parse()
                .map_err(|e: AddressError| throw(EK::EmailToParseFail, e.to_string()))?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(mail_body)
            .map_err(|e| throw(EK::EmailBodyParseFail, e.to_string()))?;

    // build the mail server credentials
    let creds = Credentials::new(
        config.mail.sender_email.clone(),
        config.mail.sender_password.clone(),
    );

    // Open a remote connection to mail server
    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay(&config.mail.mailserver_address)
            .map_err(|e| throw(EK::EmailHostnameReadFail, format!("Could not read mailserver hostname {e}")))?
            .credentials(creds)
            .build();
    
    // Send the email
    mailer
        .send(m)
        .await
        .map_err(|e| throw(EK::EmailSendFail, e.to_string()))?;

    Ok(())
}

/*pub fn deliver(
    config: &Config,
    client_ip: &str,
    host_name: &str,
    host_config: &HostConfig,
    form_values: &HashMap<String, String>
) -> Result<(), ServerError> {
    
    let mail_body = load_templates().get(&host_config.language)
        .ok_or_else(|| throw(EK::EmailLanguageNotFound, format!("defined language not found for {host_name} ({:?})", &host_config.language)))?
        .to_string();

    let form_data = fill_form_data(host_config, form_values)?;

    let subject = mail_body.lines().next()
        .ok_or_else(|| throw(EK::EmailLinesFail, format!("Could not read next line for {host_name}, {mail_body:?}")))?;
    
    let mail_body = mail_body
        .replace(subject, "")
        .replace("{{HOST}}", host_name)
        .replace("{{IP}}", client_ip)
        .replace("{{FORMDATA}}", &form_data)
        .replace("{{CONTACT}}", &config.mail.contact_spam)
        .replace("{{MAIL_SIGNATURE}}", &config.mail.mail_signature);

    let subject = subject
        .replace("{{SUBJECT}}", "")
        .replace("{{/SUBJECT}}", "");


    //mail_body = mail_body.replace("{{LINK}}", "aaa");
    // build the message body and headers
    let m =
        Message::builder()
            .from(
                config.mail.send_as.parse().map_err(|e: AddressError| {
                    throw(EK::EmailFromParseFail, e.to_string())
                })?,
            )
            .to(host_config.recipient
                .parse()
                .map_err(|e: AddressError| throw(EK::EmailToParseFail, e.to_string()))?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(mail_body)
            .map_err(|e| throw(EK::EmailBodyParseFail, e.to_string()))?;

    // build the mail server credentials
    let creds = Credentials::new(
        config.mail.sender_email.clone(),
        config.mail.sender_password.clone(),
    );

    // Open a remote connection to mail server
    let mailer = SmtpTransport::relay(&config.mail.mailserver_address)
        .map_err(|e| throw(EK::EmailHostnameReadFail, format!("Could not read mailserver hostname {e}")))?
        .credentials(creds)
        .build();

    // Send the email
    mailer
        .send(&m)
        .map_err(|e| throw(EK::EmailSendFail, e.to_string()))?;

    Ok(())
}*/
    
fn fill_form_data(host_config: &HostConfig, form_values: &HashMap<String, String>) -> Result<String, ServerError> {
    let mut form_data = String::new();
    for field in &host_config.inputs {
        let field_value = form_values.get(&field.name).cloned().unwrap_or_default();

        if field.kind == HostInputKind::Select {
            if field_value == String::default() {
                form_data.push_str(&format!("{}: {}\n\n", field.display_name, field_value));
            } else {
                let select_options = field.settings.options.clone()
                    .ok_or_else(|| throw(EK::FieldSelectNoOptions, format!("select {} has no options", field.name)))?;

                let selected_index = field_value.parse::<usize>().map_err(|e| {
                    throw(EK::FieldSelectWrongType, format!("field {} is not an unsigned integer: {}", field.name, e))
                })?;

                let real_value = select_options.get(selected_index - 1)
                    .ok_or_else(|| throw(EK::FieldSelectOutOfRange, format!("field not found: {}", field.name)))?;

                form_data.push_str(&format!("{}: {}\n\n", field.display_name, real_value));
            }
        } else {
            form_data.push_str(&format!("{}: {}\n\n", field.display_name, field_value));
        }
    }
    Ok(form_data)
}
