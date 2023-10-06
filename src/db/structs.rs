use diesel::{self, prelude::*};
use chrono::{DateTime, Utc};

#[diesel(table_name = transactions)]
#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Transactions {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub time: DateTime<Utc>,
    pub amount: i32,
    pub gems: i32,
    pub token: String,
    pub is_mail_sent: bool,
    pub is_token_used: bool,
}

#[derive(Serialize, Queryable, Debug, Clone)]
#[diesel(table_name = stars)]
pub struct Stars {
    pub id: i32,
    pub startype: i32,
    pub position_x: f32,
    pub position_y: f32,
    pub transactionid: i32,
}
