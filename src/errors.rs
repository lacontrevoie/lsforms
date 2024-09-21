use actix_web::{
    error, http::header::ContentType, http::StatusCode, HttpResponse, HttpResponseBuilder,
};
use std::fmt;
use std::fmt::Debug;

#[derive(Debug, Clone)]
#[repr(u16)]
pub enum ErrorKind {
    // critical errors: 0 -> 99
    FileNotFound = 0,
    FileReadFail,

    // warn/server-side/important errors: 100 -> 199
    CallbackKeyInvalid = 100,

    // warn/client-side errors: 200 -> 299
    StarPostInvalidToken = 200,

    // info: 300 -> 399
    TransactionExists = 300,
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

impl error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        // print to console
        eprintln!("{self}");

        HttpResponseBuilder::new(self.status_code())
            .content_type(ContentType::html())
            .body(format!("Error: {}", self.kind))
        // TODO: customize errors
    }

    fn status_code(&self) -> StatusCode {
        match self.kind.severity() as u16 {
            200..=299 => StatusCode::BAD_REQUEST,
            300..=399 => StatusCode::OK,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
