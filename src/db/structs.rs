use diesel::{self, prelude::*};
use chrono::{NaiveDate};
use serde::{Deserialize, Serialize};

use crate::db::schema::{star, transaction};

// Combines the notification eventType and:
// - formType for orders
// - paymentType for payments
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum EventType {
    OrderMembership = 0,
    OrderDonation,
    OrderUnknown,
    PaymentMonthlyDonation,
    PaymentMembership,
    PaymentDonation,
    PaymentUnknown,
}

#[derive(Identifiable, AsChangeset, Serialize, Deserialize, Queryable, Debug, Clone)]
#[diesel(table_name = transaction)]
pub struct Transaction {
    pub id: i32,
    pub username: String,
    pub message: String,
    pub email: String,
    pub day: NaiveDate,
    pub amount: i32,
    pub gems: i32,
    pub token: String,
    pub ha_id: i32,
    pub receipt_url: String,
    pub event_type: i32,
    pub is_mail_sent: bool,
    pub is_token_used: bool,
    pub is_checked: bool,
}

#[derive(Identifiable, Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
#[diesel(table_name = star)]
pub struct Star {
    pub id: i32,
    pub startype: i32,
    pub position_x: f32,
    pub position_y: f32,
    pub transactionid: i32,
}
