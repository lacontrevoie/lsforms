use actix_web::{error, http::StatusCode, http::header::ContentType, HttpResponseBuilder, HttpRequest, HttpResponse};
use std::fmt;
use std::fmt::Debug;

// TODO: transform errorkinds into regular messages/notifications
// and add type/kind (error, info) in another attribute
// for message color

#[derive(Debug, Clone)]
#[repr(u16)]
pub enum ErrorKind {
    // critical errors: 0 -> 99
    DbPool = 0,
    DbFail,
    LinkDeleteDbFail,
    AwaitFail,
    // warn errors: 100 -> 199
    BadServerAdminKey = 100,
    BlockedLinkShortener,
    BlockedLinkSpam,
    BlockedLinkFreehost,
    BlockedName,
    CaptchaFail,
    // notice errors: 200 -> 299
    UnsupportedProtocol = 200,
    LinkAlreadyExists,
    InvalidKey,
    NotManagingPhishing,
    NotDeletingPhishing,
    CookieParseFail,
    // info errors: 300 -> 399
    LinkNotFound = 300,
    InvalidUrlFrom,
    InvalidUrlTo,
    InvalidLink,
    SelflinkForbidden,
    NotFound,
    PhishingLinkReached,
}

pub enum ErrorSeverity {
    Crit,
    Warn,
    Notice,
    Info
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let severity = match self {
            ErrorSeverity::Crit => "CRITICAL",
            ErrorSeverity::Warn => "WARN",
            ErrorSeverity::Notice => "NOTICE",
            ErrorSeverity::Info => "INFO",
        };
        write!(f, "{}", severity)
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
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

impl ErrorKind {
    fn severity(&self) -> ErrorSeverity {
        let index: u16 = self.clone() as u16;
        match index {
            0..=99 => ErrorSeverity::Crit,
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

impl error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        // print to console
        eprintln!("{}", self);

        HttpResponseBuilder::new(self.status_code())
            .content_type(ContentType::html())
            .body(format!("{}", self.clone()))
        // TODO: customize errors
    }

    fn status_code(&self) -> StatusCode {
        match &self.kind.severity() {
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

