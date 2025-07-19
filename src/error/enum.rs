use crate::*;

#[derive(CustomDebug, DisplayDebug, PartialEq, Eq)]
pub enum ServerError {
    TcpBind(String),
    Unknown(String),
    HttpRead(String),
    InvalidHttpRequest(Request),
}

#[derive(CustomDebug, DisplayDebug, PartialEq, Eq)]
pub enum RouteError {
    EmptyPattern,
    DuplicatePattern(String),
    InvalidRegexPattern(String),
}
