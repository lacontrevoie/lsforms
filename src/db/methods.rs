//use crate::db::structs::{Transaction, Star};
//use crate::db::models::{NewTransaction, NewStar};
use crate::{Config, DbPool, DbConn, PooledDbConn};
use crate::errors::{ErrorKind, ServerError, throw};
use crate::db::models::{NewStar, NewTransaction, PublicStar, OwnStar, OwnStarWithId};
use crate::db::structs::{Star, Transaction, EventType};
use crate::webmodels::{sanitize, OwnTokenPost};

use diesel::prelude::*;
use diesel::dsl;
use chrono::Utc;
use serde_json::Value;
use rand::distributions::{Alphanumeric, DistString};

pub fn get_conn(dbpool: &DbPool) -> Result<PooledDbConn, ServerError> {
    let conn = dbpool
        .get()
        .map_err(|e| throw(ErrorKind::DbPool, e.to_string()));
    conn
}

impl EventType {
    pub fn from_int(i: i32) -> Option<Self> {
        match i {
            0 => Some(EventType::OrderMembership),
            1 => Some(EventType::OrderDonation),
            2 => Some(EventType::OrderUnknown),
            3 => Some(EventType::PaymentMonthlyDonation),
            4 => Some(EventType::PaymentMembership),
            5 => Some(EventType::PaymentDonation),
            6 => Some(EventType::PaymentUnknown),
            _ => None,
        }
    }
    pub fn to_int(&self) -> i32 {
        *self as i32
    }
}

impl Star {
    pub fn insert_bulk(conn: &mut DbConn, i_stars: &[NewStar]) -> Result<usize, ServerError> {
        use crate::db::schema::star::{table};

        diesel::insert_into(table)
            .values(i_stars)
            .execute(conn)
            .map_err(|e| {
            throw(ErrorKind::DbFail, e.to_string())
        })
    }
}

impl PublicStar {
    pub fn get_all_public(conn: &mut DbConn) -> Result<Vec<PublicStar>, ServerError> {
        use crate::db::schema::star::{table as s, id, startype, position_x, position_y};
        use crate::db::schema::transaction::{table as t, day, username, message};

        s
            .inner_join(t)
            .select((id, startype, day, username, message, position_x, position_y))
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
    pub fn get_from_token(conn: &mut DbConn, i_token: &str) -> Result<Option<OwnStarWithId>, ServerError> {
        use crate::db::schema::transaction::{table, id, token, username, message, gems, is_token_used};

        table
            .filter(token.eq(i_token))
            .filter(is_token_used.eq(false))
            .select((id, username, message, gems))
            .get_result::<OwnStarWithId>(conn)
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
        use crate::db::schema::transaction::{table, id};

        diesel::update(table)
            .filter(id.eq(i_tr.id))
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
    
    pub fn update_with_stars(conn: &mut DbConn, i_id: i32, star_post: OwnTokenPost) -> Result<Transaction, ServerError> {
        use crate::db::schema::transaction::{table, is_token_used, username, message, id};

        diesel::update(table)
            .filter(id.eq(i_id))
            .set((is_token_used.eq(true), username.eq(star_post.username), message.eq(star_post.message)))
            .get_result::<Transaction>(conn)
            .map_err(|e| {
                throw(ErrorKind::DbFail, e.to_string())
            })
    }
}

impl NewTransaction {

    // TO REWORK - we can simplify this fuction.
    // a callback can contain up to 2 NewTransactions because
    // of the Membership+Donation case
    pub fn from_callback(jresp: &Value) -> Option<(NewTransaction, Option<NewTransaction>)> {

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
                        cf_username = sanitize(&cf_username);
                        cf_username.truncate(15);
                    }
                }
                for e in cf.as_array()? {
                    if e.get("name")? == &config.helloasso.field_message {
                        cf_message = e.get("answer")?.as_str()?.to_string();
                        cf_message = sanitize(&cf_message);
                        cf_message.truncate(50);
                    }
                }
            }

            // define event type
            let event_type = match j.get("formType")?.as_str()? {
                "Membership" => EventType::OrderMembership,
                "Donation" => EventType::OrderDonation,
                // can be CrowdFunding, Event, PaymentForm, Checkout or Shop
                _ => EventType::OrderUnknown,
            };

            let amount = inner_payment.get("items")?.get(0)?.get("shareAmount")?.as_i64()? as i32;

            let new_tr_0 = NewTransaction {
                username: cf_username,
                message: cf_message,
                email: j.get("payer")?.get("email")?.as_str()?.to_string(),
                day: Utc::now().naive_utc().date(),
                amount,
                gems: Self::calc_gems(amount),
                token: Self::gen_random(),
                ha_id: inner_payment.get("items")?.get(0)?.get("id")?.as_i64()? as i32,
                receipt_url: inner_payment.get("paymentReceiptUrl")?.as_str()?.to_string(),
                event_type: event_type.to_int(),
                is_mail_sent: false,
                is_token_used: false,
                is_checked: false,
            };

            if let Some(payment_1) = j.get("payments")?.get(0)?.get("items")?.get(1) {
                let mut new_tr_1 = new_tr_0.clone();
                new_tr_1.amount = payment_1.get("shareAmount")?.as_i64()? as i32;
                new_tr_1.gems = Self::calc_gems(new_tr_1.amount);
                new_tr_1.token = Self::gen_random();
                new_tr_1.ha_id = payment_1.get("id")?.as_i64()? as i32;
                new_tr_1.event_type = EventType::to_int(&EventType::OrderDonation);
                
                Some((new_tr_0, Some(new_tr_1)))
            } else {
                Some((new_tr_0, None))
            }
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

            let amount = j.get("items")?.get(0)?.get("shareAmount")?.as_i64()? as i32;

            let new_tr_0 = NewTransaction {
                username: String::new(),
                message: String::new(),
                email: j.get("payer")?.get("email")?.as_str()?.to_string(),
                day: Utc::now().naive_utc().date(),
                amount,
                gems: Self::calc_gems(amount),
                token: Self::gen_random(),
                ha_id: j.get("items")?.get(0)?.get("id")?.as_i64()? as i32,
                receipt_url: j.get("paymentReceiptUrl")?.as_str()?.to_string(),
                event_type: event_type.to_int(),
                is_mail_sent: false,
                is_token_used: false,
                is_checked: false,
            };


            // handle membership+donation case
            // copied from above
            // (wrongly?) consider the 2nd payment is a donation, the first being a membership.
            if let Some(payment_1) = j.get("items")?.get(1) {
                let mut new_tr_1 = new_tr_0.clone();
                new_tr_1.amount = payment_1.get("shareAmount")?.as_i64()? as i32;
                new_tr_1.gems = Self::calc_gems(new_tr_1.amount);
                new_tr_1.token = Self::gen_random();
                new_tr_1.ha_id = payment_1.get("id")?.as_i64()? as i32;
                new_tr_1.event_type = EventType::to_int(&EventType::PaymentDonation);
                
                Some((new_tr_0, Some(new_tr_1)))
            } else {
                Some((new_tr_0, None))
            }
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
                    gems += 5;
                }
                gems
            },
            500..=i32::MAX => 250,
        }
    }
}
