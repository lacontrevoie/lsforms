use actix_web::{get, post, web, HttpRequest, HttpResponse};
use serde_json::Value;
use altcha_lib_rs::{create_challenge, ChallengeOptions, verify_json_solution};
use chrono::Utc;
use std::collections::HashMap;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use regex::Regex;
use serde::Serialize;

use crate::config::structs::{HostInput, HostInputKind};
use crate::config::methods::{load_config, load_hosts, load_re_email};
use crate::errors::{throw, ErrorKind as EK, ServerError};
use crate::emails::deliver;

#[derive(Serialize)]
pub struct ServerStatus {
    pub status: String,
    pub message: Option<String>,
}

#[get("/{host}/captcha")]
pub async fn get_captcha(form_host: web::Path<String>) -> Result<HttpResponse, ServerError> {

    let config = load_config();
    let hosts = load_hosts();
    let host = form_host.into_inner();

    // check if the host exists
    let (host_name, host_config) = hosts
        .get_key_value(&host)
        .ok_or_else(|| throw(EK::UnknownHost, format!("no host named {host}")))?;

    // check if captcha is enabled for this host
    true.then_some(host_config.enable_captcha)
        .ok_or_else(|| throw(EK::CaptchaDisabled, format!("captcha is disabled for host {host_name}")))?;

    // create a captcha challenge
    let challenge = create_challenge(ChallengeOptions {
        hmac_key: &config.captcha.secret,
        expires: Some(Utc::now() + chrono::TimeDelta::minutes(config.captcha.expires)),
        max_number: Some(config.captcha.complexity),
        ..Default::default()
    }).map_err(|_| throw(EK::CaptchaGenerationFailed, format!("could not generate challenge for {host_name}")))?;

    Ok(HttpResponse::Ok().json(challenge))
}

#[post("/{host}/send")]
pub async fn post_form(form_host: web::Path<String>, form: web::Form<Value>, req: HttpRequest) -> Result<HttpResponse, ServerError> {

    let config = load_config();
    let hosts = load_hosts();
    let host = form_host.into_inner();
    
    // check if the host exists
    let (host_name, host_config) = hosts
        .get_key_value(&host)
        .ok_or_else(|| throw(EK::UnknownHost, format!("no host named {host}")))?;

    let form_values = form_to_hashmap(&form.into_inner())?;

    // check captcha if necessary
    if host_config.enable_captcha {
        let altcha_form_entry = form_values.get("altcha").ok_or_else(|| {
            throw(EK::CaptchaFieldMissing, format!("no captcha field for {host_name}"))
        })?;
        let decoded_payload = BASE64_STANDARD.decode(altcha_form_entry).map_err(|e| {
            throw(EK::CaptchaPayloadB64Fail, format!("payload base64 decode failed for {host_name}: {e}"))
        })?;
        let string_payload = std::str::from_utf8(decoded_payload.as_slice()).map_err(|e| {
            throw(EK::CaptchaPayloadUtf8Fail, format!("payload utf8 decoding failed for {host_name}: {e}"))
        })?;
        verify_json_solution(string_payload, &config.captcha.secret, true).map_err(|_| {
            throw(EK::CaptchaResultInvalid, format!("captcha result is invalid for {host_name}"))
        })?;
    } else if form_values.contains_key("altcha") {
        // if captcha is disabled but form values contains captcha… it’s weird
        return Err(throw(EK::CaptchaFoundButDisabled, format!("contains captcha but captcha is disabled for {host_name}")));
    }

    let re_email = load_re_email();
    // check inputs
    for field in &host_config.inputs {
        form_input_check(re_email, field, &form_values)?;
    }

    // send the email
    let client_connection = req.connection_info();
    let client_ip = client_connection.realip_remote_addr()
        .ok_or_else(|| throw(EK::NoClientIP, format!("could not get client IP for {host}")))?;
    deliver(config, client_ip, host_name, host_config, &form_values).await?;
    
    Ok(HttpResponse::Ok().json(ServerStatus { status: "ok".to_string(), message: None }))
}

fn form_input_check(re_email: &Regex, host_field: &HostInput, form_values: &HashMap<String, String>) -> Result<(), ServerError> {

    // check if field exists.
    // May return an empty value for an existing and not required field.
    let form_field = match form_values.get(&host_field.name) {
        // if form field is empty and field is required => throw.
        Some(f) if f.is_empty() && host_field.required => return Err(throw(EK::FieldRequiredButEmpty, format!("field {} is empty", host_field.name))),
        // if field is not empty => ok.
        Some(f) => f,
        // if field does not exist and is not required => early return, validate the field
        None if !host_field.required => return Ok(()),
        // if field does not exist and is required => throw.
        None => return Err(throw(EK::FieldRequiredButMissing, format!("missing field {}", host_field.name))),
    };

    // check max length
    // text/textarea/email: must be a string of maximum of {maxlength.or(10000)} chars
    if (host_field.kind == HostInputKind::Text
        || host_field.kind == HostInputKind::TextArea
        || host_field.kind == HostInputKind::Email)
    && form_field.len() > host_field.settings.maxlength.unwrap_or(10000) {
        return Err(throw(EK::FieldTooLong, format!("user has written a novel for field {}", host_field.name)));
    }

    // email: must be an email…
    if host_field.kind == HostInputKind::Email && !re_email.is_match(form_field) {
        return Err(throw(EK::FieldEmailWrongType, format!("field {} is not an email", host_field.name)));
    }
    
    // select: check options
    if host_field.kind == HostInputKind::Select {
        let select_options = host_field.settings.options.clone()
            .ok_or_else(|| throw(EK::FieldSelectNoOptions, format!("select {} has no options", host_field.name)))?;

        let selected_index = form_field.parse::<usize>().map_err(|e| {
            throw(EK::FieldSelectWrongType, format!("field {} is not an unsigned integer: {}", host_field.name, e))
        })?;

        if selected_index == 0 && host_field.required {
            return Err(throw(EK::FieldRequiredButEmpty, format!("field {} is required but empty", host_field.name)));
        }

        if selected_index >= select_options.len() {
            return Err(throw(EK::FieldSelectOutOfRange, format!("field {} is out of range", host_field.name)));
        }
    }

    Ok(())
}

fn form_to_hashmap(form_data: &Value) -> Result<HashMap<String, String>, ServerError> {

    let mut form_values: HashMap<String, String> = HashMap::new();

    let form_data_obj = form_data.as_object()
        .ok_or_else(|| throw(EK::FormDataNotObject, format!("Received form data is not convertible to object: {form_data:?}")))?;

    for (key, value) in form_data_obj {
        let value_str = value.as_str()
        .ok_or_else(|| throw(EK::FormParamNotString, format!("Could not read form param as string: {form_data:?}")))?
        .to_string();

        form_values.insert(key.to_string(), value_str);
    }
    Ok(form_values)
}

