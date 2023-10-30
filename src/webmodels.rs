use crate::errors::{ErrorKind, ServerError, throw};
use crate::config::structs::Stars;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ClientStatus {
    pub code: i32,
    pub message: String,
}

// Given as GET argument
#[derive(Serialize, Deserialize)]
pub struct OwnToken {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct OwnTokenPost {
    pub token: String,
    pub username: String,
    pub message: String,
    pub stars: Vec<NewStarPost>,
}

#[derive(Serialize, Deserialize)]
pub struct NewStarPost {
    pub startype: i32,
    pub position_x: f32,
    pub position_y: f32,
}

#[derive(Serialize, Deserialize)]
pub struct GenericId {
    pub id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct SendMailId {
    pub tr_id: i32,
    pub tpl_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CallbackKey {
    pub callback_key: String,
}

impl OwnTokenPost {
    pub fn validate(&mut self, i_max_gems: i32) -> Result<(), ServerError> {
        let starlist = Stars::global();
        let mut spent_gems = 0;

        for star in &self.stars {
            let data_star = starlist.0.iter().find(|s| s.startype == star.startype);
            // check if startype exists
            if let Some(s) = data_star {
                spent_gems += s.price;
            } else {
                return Err(throw(
                    ErrorKind::StarPostInvalidStartype,
                    format!("given startype: {}", star.startype)
                ));
            }
            // check if percentage is safe
            if star.position_x < 0.0 || star.position_x >= 100.0 {
                return Err(throw(
                    ErrorKind::StarPostInvalidPct,
                    format!("given pct for position_x: {}", star.position_x)
                ));
            }
            if star.position_y < 0.0 || star.position_y >= 100.0 {
                return Err(throw(
                    ErrorKind::StarPostInvalidPct,
                    format!("given pct for position_y: {}", star.position_x)
                ));
            }
        }
        // check if the sum star is not higher than max gems
        if i32::from(spent_gems) > i_max_gems {
            return Err(throw(
                ErrorKind::StarPostTooManyStars,
                format!("star count: {} / {}", spent_gems, i_max_gems)
            ));
        }

        // sanitize/trim inputs like username and message
        self.username = sanitize(&self.username);
        self.username.truncate(15);
        
        self.message = sanitize(&self.message);
        self.message.truncate(50);
        
        Ok(())
    }
}

pub fn sanitize(s: &str) -> String {
    s.replace(['&', '<', '>', '\"', '\''], "")
}
