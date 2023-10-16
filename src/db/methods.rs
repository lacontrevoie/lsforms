//use crate::db::structs::{Transaction, Star};
//use crate::db::models::{NewTransaction, NewStar};
use crate::{Config, DbPool, DbConn, PooledDbConn};
use crate::errors::{ErrorKind, ServerError, throw};
use crate::db::models::{NewTransaction, PublicStar, OwnStar};
use crate::db::structs::{Transaction, EventType};

use diesel::prelude::*;
use diesel::dsl;
use chrono::Utc;
use serde_json::Value;
use rand::distributions::{Alphanumeric, DistString};

pub fn get_conn(dbpool: &DbPool) -> Result<PooledDbConn, ServerError> {
    let mut conn = dbpool
        .get()
        .map_err(|e| throw(ErrorKind::DbPool, e.to_string()));
    conn
}

impl EventType {
    pub fn from_int(i: i32) -> Option<Self> {
        match i {
            0 => Some(EventType::OrderMembership),
            1 => Some(EventType::OrderMembershipAndDonation),
            2 => Some(EventType::OrderDonation),
            3 => Some(EventType::OrderUnknown),
            4 => Some(EventType::PaymentMonthlyDonation),
            5 => Some(EventType::PaymentMembership),
            6 => Some(EventType::PaymentDonation),
            7 => Some(EventType::PaymentUnknown),
            _ => None,
        }
    }
    pub fn to_int(&self) -> i32 {
        *self as i32
    }
}

impl PublicStar {
    pub fn get_all_public(conn: &mut DbConn) -> Result<Vec<PublicStar>, ServerError> {
        use crate::db::schema::star::{table as s, id, startype, position_x, position_y};
        use crate::db::schema::transaction::{table as t, day, username};

        s
            .inner_join(t)
            .select((id, startype, day, username, position_x, position_y))
            .get_results::<PublicStar>(conn)
            .map_err(|e| {
            throw(ErrorKind::DbFail, e.to_string())
        })
    }
}

impl OwnStar {
    // Returns Ok(Some()) if a transaction with the given token is found
    // Returns Ok(None) if no transaction is found with the token 
    // Returns Err in case of DB error
    pub fn get_from_token(conn: &mut DbConn, i_token: String) -> Result<Option<OwnStar>, ServerError> {
        use crate::db::schema::transaction::{table, token, username, message, gems, is_token_used};

        table
            .filter(token.eq(i_token))
            .filter(is_token_used.eq(false))
            .select((username, message, gems))
            .get_result::<OwnStar>(conn)
            .optional()
            .map_err(|e| {
                throw(ErrorKind::DbFail, e.to_string())
            })
    }
}

impl Transaction {

    pub fn find_by_haid(conn: &mut DbConn, i_haid: i32) -> Result<Option<Transaction>, ServerError> {
        use crate::db::schema::transaction::{table, ha_id};
        table.filter(ha_id.eq(i_haid))
            .get_result::<Transaction>(conn)
            .optional()
            .map_err(|e| {
                throw(ErrorKind::DbFail, e.to_string())
            })
    }

    pub fn update(conn: &mut DbConn, i_tr: &Transaction) -> Result<Transaction, ServerError> {
        use crate::db::schema::transaction::{table};

        diesel::update(table)
            .set(i_tr)
            .get_result::<Transaction>(conn)
            .map_err(|e| {
                throw(ErrorKind::DbFail, e.to_string())
            })
    }
    pub fn toggle_check(conn: &mut DbConn, i_id: i32) -> Result<Transaction, ServerError> {
        use crate::db::schema::transaction::{table, is_checked, id};

        diesel::update(table)
            .filter(id.eq(i_id))
            .set(is_checked.eq(dsl::not(is_checked)))
            .get_result::<Transaction>(conn)
            .map_err(|e| {
                throw(ErrorKind::DbFail, e.to_string())
            })
    }
    
    pub fn send_mail(conn: &mut DbConn, i_id: i32) -> Result<Transaction, ServerError> {
        use crate::db::schema::transaction::{table, is_mail_sent, id};

        diesel::update(table)
            .filter(id.eq(i_id))
            .set(is_mail_sent.eq(true))
            .get_result::<Transaction>(conn)
            .map_err(|e| {
                throw(ErrorKind::DbFail, e.to_string())
            })
    }
}

impl NewTransaction {
    pub fn from_callback(jresp: &Value) -> Option<NewTransaction> {

        // Decode callback from helloasso API
        //
        // there are multiple cases to handle, and our structs
        // oversimplifies their data.

        let ha_event_type = jresp.get("eventType")?.as_str()?;
        let j = jresp.get("data")?;

        if ha_event_type == "Order" {
            let inner_payment = j.get("payments")?.get(0)?;

            let custom_fields = j.get("items")?.get(0)?.get("customFields");

            let mut cf_username = String::new();
            let mut cf_message = String::new();

            // define custom fields
            if let Some(cf) = custom_fields {
                let config = Config::global();
                for e in cf.as_array()? {
                    if e.get("name")? == &config.helloasso.field_username {
                        cf_username = e.get("answer")?.as_str()?.to_string();
                    }
                }
                for e in cf.as_array()? {
                    if e.get("name")? == &config.helloasso.field_message {
                        cf_message = e.get("answer")?.as_str()?.to_string();
                    }
                }
            }

            // define event type
            let event_type = match j.get("formType")?.as_str()? {
                "Membership" => {
                    if j.get("payments")?.get(1).is_some() {
                        EventType::OrderMembershipAndDonation
                    } else {
                        EventType::OrderMembership
                    }
                },
                "Donation" => EventType::OrderDonation,
                // can be CrowdFunding, Event, PaymentForm, Checkout or Shop
                _ => EventType::OrderUnknown,
            };

            let amount = inner_payment.get("amount")?.as_i64()? as i32;

            Some(NewTransaction {
                username: cf_username,
                message: cf_message,
                email: j.get("payer")?.get("email")?.as_str()?.to_string(),
                day: Utc::now().naive_utc().date(),
                amount: amount,
                gems: Self::calc_gems(amount),
                token: Self::gen_random(),
                ha_id: inner_payment.get("id")?.as_i64()? as i32,
                receipt_url: inner_payment.get("paymentReceiptUrl")?.as_str()?.to_string(),
                event_type: event_type.to_int(),
                is_mail_sent: false,
                is_token_used: false,
                is_checked: false,
            })

        }
        else if ha_event_type == "Payment" {

            // define event type
            let event_type = match j.get("items")?.get(0)?.get("type")?.as_str()? {
                "MonthlyDonation" => EventType::PaymentMonthlyDonation,
                "Membership" => EventType::PaymentMembership,
                "Donation" => EventType::PaymentDonation,
                // can be Payment, Registration, MonthlyPayment,
                // OfflineDonation, Contribution, Bonus or Product
                _ => EventType::PaymentUnknown,
            };

            let amount = j.get("amount")?.as_i64()? as i32;

            Some(NewTransaction {
                username: String::new(),
                message: String::new(),
                email: j.get("payer")?.get("email")?.as_str()?.to_string(),
                day: Utc::now().naive_utc().date(),
                amount: amount,
                gems: Self::calc_gems(amount),
                token: Self::gen_random(),
                ha_id: j.get("id")?.as_i64()? as i32,
                receipt_url: j.get("paymentReceiptUrl")?.as_str()?.to_string(),
                event_type: event_type.to_int(),
                is_mail_sent: false,
                is_token_used: false,
                is_checked: false,
            })
        }
        else {
            // Form event types and others (?).
            None
        }
    }
    pub fn validate(&mut self) {
        if self.gems == 0 {
            self.gems = Self::calc_gems(self.amount);
        }

        if self.token.is_empty() {
            self.token = Self::gen_random();
        }
    }

    fn gen_random() -> String {
        Alphanumeric.sample_string(&mut rand::thread_rng(), 32)
    }

    // returns a number of gems depending on the donation amount.
    // minimum -> 5 Gems, maximum -> 250 Gems (500€+).
    pub fn calc_gems(amount: i32) -> i32 {

        match amount / 100 {
            i32::MIN..=0 => 0,
            1..=499 => {
                let mut gems = (0.48 * (amount / 100) as f32) as i32;
                // round to the nearest 5 multiplier
                let gems_modulo = gems % 5;
                gems = gems + 5 - gems_modulo;

                if gems_modulo >= 3 {
                    gems = gems + 5;
                }
                gems
            },
            500..=i32::MAX => 250,
        }
    }
}
