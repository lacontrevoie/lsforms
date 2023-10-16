use crate::errors::{ErrorKind, ServerError, throw};
use crate::config::structs::Config;
use crate::{DbPool, db};
use crate::db::structs::Transaction;
use crate::db::methods::get_conn;
use crate::db::generic::db_insert;
use crate::db::models::NewTransaction;
use crate::webmodels::CallbackKey;

use actix_web::{post, HttpResponse, web};
use serde_json::Value;


#[post("/callback/helloasso/{callback_key}")]
pub async fn callback_helloasso(
    bodybytes: web::Bytes,
    dbpool: web::Data<DbPool>,
    params: web::Path<CallbackKey>
    ) -> Result<HttpResponse, ServerError> {

    let mut conn = get_conn(&dbpool)?;
    let config = Config::global();

    if config.helloasso.callback_key != params.callback_key {
        return Err(throw(ErrorKind::CallbackKeyInvalid, "Wrong callback key!".to_string()));
    }

    let jresp = serde_json::from_slice::<Value>(&bodybytes).map_err(|e| {
        throw(ErrorKind::CallbackParseFail, e.to_string())
    })?;

    if let Some(tr) = NewTransaction::from_callback(&jresp) {

        // check if another transaction with the same ha_id exists
        if Transaction::find_by_haid(&mut conn, tr.ha_id)?.is_some() {
            return Err(throw(ErrorKind::TransactionExists, format!("ha_id: {}", tr.ha_id)));
        }

        // explicitly casting result to Transaction
        // for db_insert to work
        let _: Transaction = db_insert(&mut conn, db::schema::transaction::table, tr)?;

        // no need for a request body
        Ok(HttpResponse::Ok().finish())
    } else {
        return Err(throw(ErrorKind::CallbackReadFail, format!("Full response: {}", jresp)));
    }
}

