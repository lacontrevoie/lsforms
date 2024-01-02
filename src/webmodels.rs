use crate::config::structs::Stars;
use crate::errors::{throw, ErrorKind, ServerError};

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
                    format!("given startype: {}", star.startype),
                ));
            }
            // check if percentage is safe
            if star.position_x < 0.0 || star.position_x >= 100.0 {
                return Err(throw(
                    ErrorKind::StarPostInvalidPct,
                    format!("given pct for position_x: {}", star.position_x),
                ));
            }
            if star.position_y < 0.0 || star.position_y >= 100.0 {
                return Err(throw(
                    ErrorKind::StarPostInvalidPct,
                    format!("given pct for position_y: {}", star.position_x),
                ));
            }
        }
        // check if the sum star is not higher than max gems
        if i32::from(spent_gems) > i_max_gems {
            return Err(throw(
                ErrorKind::StarPostTooManyStars,
                format!("star count: {spent_gems} / {i_max_gems}"),
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
    let convert: Vec<(char, &str)> = vec![
        ('&',  "&amp;"),
        ('<',  "&lt;"),
        ('>',  "&gt;"),
        ('"',  "&quot;"),
        ('\'', "&#039;"),
    ];

    let values: Vec<&str> = convert.iter()
        .map(|v| v.1)
        .collect();

    let mut output: String = String::from("");

    // manually sanitize '&' if not already done
    let mut i = 0;
    while let Some(j) = s[i..].find('&') {
        output.push_str(&s[i..i + j]);
        match values.iter().position(|v| s[i + j..].starts_with(v)) {
            Some(p) => {
                output.push_str(values[p]);
                i += j + values[p].len();
            },
            None => {
                output.push_str("&amp;");
                i += j + 1;
            },
        }
    }
    match i {
        0 => output.push_str(s),
        _ => output.push_str(&s[i..]),
    }

    // now we can safely convert the other characters
    convert.into_iter()
        .filter(|(from, _)| *from != '&')
        .for_each(|(from, to)| {
            output = output.replace(from, to);
        });

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webmodel_sanitize() {
        let checks: Vec<(&str, &str)> = vec![
            // check the conversion
            ("<", "&lt;"),
            (">", "&gt;"),
            ("&", "&amp;"),
            ("\"", "&quot;"),
            ("'", "&#039;"),

            ("<3", "&lt;3"),
            ("<>&\"'", "&lt;&gt;&amp;&quot;&#039;"),
            ("<>", "&lt;&gt;"),

            // the conversion has already been done, do nothing
            ("&lt;", "&lt;"),
            ("&gt;", "&gt;"),
            ("&amp;", "&amp;"),
            ("&quot;", "&quot;"),
            ("&#039;", "&#039;"),

            // string with a mix of both
            ("<3 &amp; <3", "&lt;3 &amp; &lt;3"),
            ("<script&gt;</script>", "&lt;script&gt;&lt;/script&gt;"),
            ("&lt;3 ><", "&lt;3 &gt;&lt;"),
        ];

        for (input, expected) in checks.into_iter() {
            assert_eq!(sanitize(input), expected);
        }
    }
}
