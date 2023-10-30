use crate::errors::{ErrorKind, ServerError, throw};
use crate::config::structs::Config;
use crate::{DbPool, DbConn, db};
use crate::db::structs::{Transaction, EventType};
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
        throw(ErrorKind::CallbackParseFail, format!("{}: {:?}", e, String::from_utf8(bodybytes.to_vec())))
    })?;

    if let Some(new_tr) = NewTransaction::from_callback(&jresp) {
        let override_result = check_override_tr(&mut conn, &new_tr.0);

        // handle the membership+donation case
        if let Some(another_new_tr) = new_tr.1 {
            // returns error message for the donation if exists,
            // instead of the membership
            check_override_tr(&mut conn, &another_new_tr)?;
        } else {
            override_result?;
        }

        // no need for a request body
        Ok(HttpResponse::Ok().finish())
    } else {
        Err(throw(ErrorKind::CallbackReadFail, format!("Full response: {jresp}")))
    }
}

// if a transaction with the same helloasso ID exists in DB,
// overrides it by adding some more info.
fn check_override_tr(conn: &mut DbConn, new_tr: &NewTransaction) -> Result<(), ServerError> {
    // check if another transaction with the same ha_id exists
    if let Some(mut old_tr) = Transaction::find_by_haid(conn, new_tr.ha_id)? {
        // if that's the case, try to add missing username field
        if old_tr.username.is_empty() && !new_tr.username.is_empty() {

            old_tr.username = new_tr.username.clone();

            if old_tr.message.is_empty() && !new_tr.message.is_empty() {
                old_tr.message = new_tr.message.clone();
            }

            // upgrade old transaction event type when
            // we receive an order from an existing payment
            old_tr.event_type = match (EventType::from_int(old_tr.event_type).unwrap(), EventType::from_int(new_tr.event_type).unwrap()) {
                (EventType::PaymentDonation, EventType::OrderDonation)
                    | (EventType::PaymentMembership, EventType::OrderMembership) => new_tr.event_type,
                _ => old_tr.event_type,
            };

            Transaction::update(conn, &old_tr)?;

            return Err(throw(ErrorKind::TransactionUpdated, format!("ha_id: {}", new_tr.ha_id)));
        }
        // else, the transaction is not updated
        return Err(throw(ErrorKind::TransactionExists, format!("ha_id: {}", new_tr.ha_id)));
    }

    // explicitly casting result to Transaction
    // for db_insert to work
    let _: Transaction = db_insert(conn, db::schema::transaction::table, new_tr)?;
    Ok(())
}
