use diesel::{self, prelude::*};
use chrono::{NaiveDate};
use crate::db::schema::*;

#[derive(Serialize, Deserialize, Insertable, Debug, Clone)]
#[diesel(table_name = transaction)]
pub struct NewTransaction {
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

#[derive(Serialize, Insertable, Debug, Clone)]
#[diesel(table_name = star)]
pub struct NewStar {
    pub startype: i32,
    pub position_x: f32,
    pub position_y: f32,
    pub transactionid: i32,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct PublicStar {
    pub id: i32,
    pub startype: i32,
    pub day: NaiveDate,
    pub username: String,
    pub message: String,
    pub position_x: f32,
    pub position_y: f32,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct OwnStarWithId {
    pub id: i32,
    pub username: String,
    pub message: String,
    pub gems: i32,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct OwnStar {
    pub username: String,
    pub message: String,
    pub gems: i32,
}

