use crate::*;

#[derive(Debug)]
pub enum ServerError {
    TcpBind(String),
    Unknown(String),
    HttpRead(String),
    InvalidHttpRequest(Request),
}

#[derive(Debug)]
pub(crate) enum RouteError {
    EmptyPattern,
    DuplicatePattern(String),
    InvalidRegexPattern(String),
}
