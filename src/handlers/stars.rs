use crate::errors::{ErrorKind, ServerError, throw};
use crate::db::methods::get_conn;
use crate::db::generic::db_remove;
use crate::db::models::{NewStar, PublicStar, OwnStar};
use crate::db::structs::{Star, Transaction};
use crate::webmodels::{GenericId, OwnToken, OwnTokenPost, ClientStatus};
use crate::{DbPool, db};

use actix_web::{get, post, delete, HttpResponse, web};

#[get("/api/stars/own")]
pub async fn get_stars_own(
    dbpool: web::Data<DbPool>,
    web::Query(query): web::Query<OwnToken>,
    ) -> Result<HttpResponse, ServerError> {
    
    let mut conn = get_conn(&dbpool)?;

    let ownstar_withid = OwnStar::get_from_token(&mut conn, &query.token)?;


    if let Some(o) = ownstar_withid {
        let ownstar = OwnStar {
            username: o.username,
            message: o.message,
            gems: o.gems,
        };
        Ok(HttpResponse::Ok().json(ownstar))
    } else {

        let c_err = ClientStatus {
            code: 3001,
            message: "Token invalid or already used".to_string()
        };
        Ok(HttpResponse::Ok().json(c_err))
    }
}

#[post("/api/stars/own")]
pub async fn post_stars_own(
    dbpool: web::Data<DbPool>,
    web::Json(mut star_post): web::Json<OwnTokenPost>,
    ) -> Result<HttpResponse, ServerError> {
    let mut conn = get_conn(&dbpool)?;

    // check if the stars are sent by the rightful user
    let ownstar = OwnStar::get_from_token(&mut conn, &star_post.token)?;

    if let Some(o) = ownstar {
        // parse and validate the stars
        star_post.validate(o.gems)?;

        // 
        let mut new_stars: Vec<NewStar> = Vec::new();
        for s in &star_post.stars {
            new_stars.push(NewStar {
                startype: s.startype,
                position_x: s.position_x,
                position_y: s.position_y,
                transactionid: o.id,
            });
        }

        Star::insert_bulk(&mut conn, &new_stars)?;
        Transaction::update_with_stars(&mut conn, o.id, star_post)?;

        let c_ok = ClientStatus {
            code: 1001,
            message: "OK".to_string()
        };
        Ok(HttpResponse::Ok().json(c_ok))
    } else {
        Err(throw(ErrorKind::StarPostInvalidToken, format!("given token: {}", star_post.token)))
    }
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

    db_remove(&mut conn, db::schema::star::table, params.id)?;

    let c_ok = ClientStatus {
        code: 1001,
        message: "OK".to_string()
    };
    Ok(HttpResponse::Ok().json(c_ok))
}

