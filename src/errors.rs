use actix_web::{
    error, http::header::ContentType, http::StatusCode, HttpResponse, HttpResponseBuilder,
};
use std::fmt;
use std::fmt::Debug;
use serde::Serialize;

use crate::config::structs::VerboseLevel;
use crate::config::methods::load_config;

#[derive(Debug, Clone)]
#[repr(u16)]
pub enum ErrorKind {
    // critical errors: 0 -> 99
    FileNotFound = 0,
    FileReadFail,

    // warn/server-side/important errors: 100 -> 199
    CaptchaGenerationFailed = 100,
    FieldSelectNoOptions,
    EmailLanguageNotFound,
    EmailLinesFail,
    EmailSendFail,
    EmailHostnameReadFail,
    EmailBodyParseFail,
    EmailToParseFail,
    EmailFromParseFail,
    NoClientIP,

    // warn/client-side errors: 200 -> 299
    UnknownHost = 200,
    CaptchaDisabled,
    CaptchaFieldMissing,
    CaptchaPayloadB64Fail,
    CaptchaPayloadUtf8Fail,
    CaptchaResultInvalid,
    CaptchaFoundButDisabled,
    FormDataNotObject,
    FormParamNotString,
    FieldSelectWrongType,
    FieldEmailWrongType,
    FieldSelectOutOfRange,
    FieldRequiredButEmpty,
    FieldRequiredButMissing,
    FieldTooLong,

    // info: 300 -> 399
    // XXX = 300,
}

pub enum ErrorSeverity {
    Crit,
    Warn,
    Notice,
    Info,
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let severity = match self {
            ErrorSeverity::Crit => "CRITICAL",
            ErrorSeverity::Warn => "WARN",
            ErrorSeverity::Notice => "NOTICE",
            ErrorSeverity::Info => "INFO",
        };
        write!(f, "{severity}")
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone)]
pub struct ServerError {
    pub kind: ErrorKind,
    pub msg: String,
}

// easily create an ErrorInfo to throw an error
pub fn throw(kind: ErrorKind, msg: String) -> ServerError {
    ServerError { kind, msg }
}

impl ServerError {
    // for non-blocking errors
    pub fn display(&self) {
        let config = load_config();
        match (self.kind.severity(), &config.general.verbose_level) {
            (ErrorSeverity::Crit, VerboseLevel::Crit) => {
                println!("{self}");
            },
            (ErrorSeverity::Crit | ErrorSeverity::Warn, VerboseLevel::Warn) => {
                println!("{self}");
            },
            (ErrorSeverity::Crit
             | ErrorSeverity::Warn
             | ErrorSeverity::Notice,
             VerboseLevel::Notice) => {
                println!("{self}");
            },
            (ErrorSeverity::Crit
             | ErrorSeverity::Warn
             | ErrorSeverity::Notice
             | ErrorSeverity::Info,
             VerboseLevel::Info) => {
                println!("{self}");
            },
            _ => {}
        }
    }
}

impl ErrorKind {
    fn severity(&self) -> ErrorSeverity {
        let index: u16 = self.clone() as u16;
        match index {
            100..=199 => ErrorSeverity::Warn,
            200..=299 => ErrorSeverity::Notice,
            300..=399 => ErrorSeverity::Info,
            _ => ErrorSeverity::Crit,
        }
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}: {}", self.kind.severity(), self.kind, self.msg)
    }
}

#[derive(Serialize)]
pub struct ErrorServerStatus {
    pub status: String,
    pub error_kind: String,
    pub code: u16,
}

impl error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        // print to console
        self.display();

        // display limited information in http response
        HttpResponseBuilder::new(self.status_code())
            .content_type(ContentType::json())
            .json(ErrorServerStatus { status: "error".to_string(), error_kind: self.kind.to_string(), code: self.kind.clone() as u16 })
            //.body(format!("{{ \"status\": \"error\", \"error_kind\": \"{}\", \"code\": {} }}", self.kind, self.kind.clone() as u16))
    }

    fn status_code(&self) -> StatusCode {
        match self.kind.severity() as u16 {
            200..=299 => StatusCode::BAD_REQUEST,
            300..=399 => StatusCode::OK,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
