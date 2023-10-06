use crate::errors::{ErrorKind, ServerError, throw};
use crate::config::structs::Config;

use actix_web::{get, post, http, http::Method, HttpRequest, HttpResponse, web::{Bytes}};


#[post("/callback/helloasso")]
pub async fn callback_helloasso(bytes: Bytes) -> Result<HttpResponse, ServerError> {

    if let Ok(s) = String::from_utf8(bytes.to_vec()) {
        println!("{:#?}", s);
    } else {
        println!("Deserialization failed");
    }

    Ok(HttpResponse::Ok().body("OK"))
    // throw(kind, msgstring) -> ServerError
}

