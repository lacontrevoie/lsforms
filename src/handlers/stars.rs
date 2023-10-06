use crate::errors::{ErrorKind, ServerError, throw};
use crate::config::structs::Config;

use actix_web::{get, post, http, http::Method, HttpRequest, HttpResponse, web};


#[get("/helloworld")]
pub async fn helloworld() -> Result<HttpResponse, ServerError> {
    Ok(HttpResponse::Ok().body("OK"))
    // throw(kind, msgstring) -> ServerError
}


#[get("/api/stars/own")]
pub async fn get_stars_own() -> Result<HttpResponse, ServerError> {
    Ok(HttpResponse::Ok().body("OK"))
    // throw(kind, msgstring) -> ServerError
}

#[post("/api/stars/own")]
pub async fn post_stars_own() -> Result<HttpResponse, ServerError> {
    Ok(HttpResponse::Ok().body("OK"))
    // throw(kind, msgstring) -> ServerError
}


#[get("/api/stars/global")]
pub async fn get_stars_global() -> Result<HttpResponse, ServerError> {
    Ok(HttpResponse::Ok().body("OK"))
    // throw(kind, msgstring) -> ServerError
}


