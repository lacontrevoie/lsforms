use crate::errors::{ErrorKind, ServerError, throw};
use crate::db::methods::get_conn;
use crate::db::models::{PublicStar, OwnStar};
use crate::webmodels::{GenericId, OwnToken, ClientStatus};
use crate::DbPool;

use actix_web::{get, post, delete, HttpResponse, web};

#[get("/api/stars/own")]
pub async fn get_stars_own(
    dbpool: web::Data<DbPool>,
    web::Query(query): web::Query<OwnToken>,
    ) -> Result<HttpResponse, ServerError> {
    
    let mut conn = get_conn(&dbpool)?;

    let ownstar = OwnStar::get_from_token(&mut conn, query.token)?;

    if let Some(o) = ownstar {
        Ok(HttpResponse::Ok().json(o))
    } else {

        let c_err = ClientStatus {
            code: 3001,
            message: "Token invalid or already used".to_string()
        };
        Ok(HttpResponse::Ok().json(c_err))
    }

    // throw(kind, msgstring) -> ServerError
}

#[post("/api/stars/own")]
pub async fn post_stars_own() -> Result<HttpResponse, ServerError> {

    Ok(HttpResponse::Ok().body("OK"))
    // throw(kind, msgstring) -> ServerError
}

#[get("/api/stars/global")]
pub async fn get_stars_global(dbpool: web::Data<DbPool>) -> Result<HttpResponse, ServerError> {
    let mut conn = get_conn(&dbpool)?;

    let starlist: Vec<PublicStar> = PublicStar::get_all_public(&mut conn)?;

    Ok(HttpResponse::Ok().json(starlist))
}

#[delete("/admin/api/stars/{id}")]
pub async fn delete_star(
    dbpool: web::Data<DbPool>,
    params: web::Path<GenericId>
    ) -> Result<HttpResponse, ServerError> {
    let mut conn = get_conn(&dbpool)?;

    Ok(HttpResponse::Ok().body("Not implemented"))
}

