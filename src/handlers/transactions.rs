use crate::errors::{ErrorKind, ServerError, throw};
use crate::DbPool;
use crate::db;
use crate::db::generic::{db_get_all, db_insert, db_remove};
use crate::db::structs::Transaction;
use crate::db::methods::get_conn;
use crate::db::models::NewTransaction;
use crate::webmodels::{ClientStatus};
use crate::webmodels::GenericId;

use actix_web::{get, post, put, patch, delete, HttpResponse, web};

#[get("/admin/api/transaction")]
pub async fn get_transaction(dbpool: web::Data<DbPool>) -> Result<HttpResponse, ServerError> {
    let mut conn = get_conn(&dbpool)?;

    let translist: Vec<Transaction> = db_get_all(&mut conn, db::schema::transaction::table)?;
    Ok(HttpResponse::Ok().json(translist))
}

#[put("/admin/api/transaction")]
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
    params: web::Path<GenericId>
    ) -> Result<HttpResponse, ServerError> {
    let mut conn = get_conn(&dbpool)?;

    db_remove(&mut conn, db::schema::transaction::table, params.id)?;

    let c_ok = ClientStatus {
        code: 1001,
        message: "OK".to_string(),
    };
    Ok(HttpResponse::Ok().json(c_ok))
}

#[post("/admin/api/transaction/{id}/send_mail")]
pub async fn post_transaction_send_mail(
    dbpool: web::Data<DbPool>,
    params: web::Path<GenericId>
    ) -> Result<HttpResponse, ServerError> {
    let mut conn = get_conn(&dbpool)?;

    Transaction::send_mail(&mut conn, params.id)?;
        
    let c_ok = ClientStatus {
        code: 1002,
        message: "OK".to_string()
    };

    Ok(HttpResponse::Ok().json(c_ok))
}

#[post("/admin/api/transaction/{id}/toggle_check")]
pub async fn post_transaction_toggle_check(
    dbpool: web::Data<DbPool>,
    params: web::Path<GenericId>
    ) -> Result<HttpResponse, ServerError> {
    let mut conn = get_conn(&dbpool)?;

    Transaction::toggle_check(&mut conn, params.id)?;

    let c_ok = ClientStatus {
        code: 1003,
        message: "OK".to_string()
    };
    Ok(HttpResponse::Ok().json(c_ok))
}
