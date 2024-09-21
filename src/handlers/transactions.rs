/*
use crate::emails;
use crate::errors::{throw, ErrorKind, ServerError};
use crate::webmodels::{ClientStatus, GenericId, SendMailId};
use crate::{db, Config, DbPool};

use actix_web::{delete, get, patch, post, put, web, HttpResponse};

#[get("/admin/api/transaction")]
pub async fn get_transaction(dbpool: web::Data<DbPool>) -> Result<HttpResponse, ServerError> {
    let mut conn = get_conn(&dbpool)?;

    let mut translist: Vec<Transaction> = db_get_all(&mut conn, db::schema::transaction::table)?;
    translist.sort_by(|a, b| b.id.cmp(&a.id));
    Ok(HttpResponse::Ok().json(translist))
}

pub async fn put_transaction(
    dbpool: web::Data<DbPool>,
    web::Json(mut tr): web::Json<NewTransaction>,
) -> Result<HttpResponse, ServerError> {
    let mut conn = get_conn(&dbpool)?;

    tr.validate();

    let tr: Transaction = db_insert(&mut conn, db::schema::transaction::table, tr)?;

    Ok(HttpResponse::Ok().json(tr))
}

#[patch("/admin/api/transaction")]
pub async fn patch_transaction(
    dbpool: web::Data<DbPool>,
    web::Json(tr): web::Json<Transaction>,
) -> Result<HttpResponse, ServerError> {
    let mut conn = get_conn(&dbpool)?;

    let upd_tr = Transaction::update(&mut conn, &tr)?;

    Ok(HttpResponse::Ok().json(upd_tr))
}
#[delete("/admin/api/transaction/{id}")]
pub async fn delete_transaction(
    dbpool: web::Data<DbPool>,
    params: web::Path<GenericId>,
) -> Result<HttpResponse, ServerError> {
    let mut conn = get_conn(&dbpool)?;

    db_remove(&mut conn, db::schema::transaction::table, params.id)?;

    let c_ok = ClientStatus {
        code: 1001,
        message: "OK".to_string(),
    };
    Ok(HttpResponse::Ok().json(c_ok))
}

#[post("/admin/api/transaction/{id}/toggle_check")]
pub async fn post_transaction_toggle_check(
    dbpool: web::Data<DbPool>,
    params: web::Path<GenericId>,
) -> Result<HttpResponse, ServerError> {
    let mut conn = get_conn(&dbpool)?;

    Transaction::toggle_check(&mut conn, params.id)?;

    let c_ok = ClientStatus {
        code: 1003,
        message: "OK".to_string(),
    };
    Ok(HttpResponse::Ok().json(c_ok))
}

#[get("/admin/api/email_templates")]
pub async fn get_email_templates() -> Result<HttpResponse, ServerError> {
    let config = Config::global();

    Ok(HttpResponse::Ok().json(&config.mail.templates))
}*/
