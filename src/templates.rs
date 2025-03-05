use askama::Template;
use actix_web::http::header::ContentType;
use actix_web::{get, web, HttpResponse};

use crate::config::structs::{Config, HostConfig, HostInputKind};
use crate::errors::{throw, ErrorKind as EK, ServerError};
use crate::config::methods::{load_config, load_hosts};

#[derive(Template)]
#[template(path = "form.html")]
pub struct FormTemplate<'a> {
    config: &'a Config,
    hostname: &'a str,
    host: &'a HostConfig,
}

#[get("/{host}/gen_tpl")]
pub async fn gen_tpl(form_host: web::Path<String>) -> Result<HttpResponse, ServerError> {
    let config = load_config();
    let hosts = load_hosts();
    let host = form_host.into_inner();

    // check if the host exists
    let (host_name, host_config) = hosts
        .get_key_value(&host)
        .ok_or_else(|| throw(EK::UnknownHost, format!("no host named {host}")))?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(FormTemplate { config: &config, hostname: &host_name, host: &host_config }.render().expect("TODO"))
    )

}
