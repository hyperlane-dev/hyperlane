use crate::*;

#[derive(CustomDebug, DisplayDebug)]
pub enum ServerError {
    TcpBind(String),
    Unknown(String),
    HttpRead(String),
    InvalidHttpRequest(Request),
}

#[derive(CustomDebug, DisplayDebug)]
pub(crate) enum RouteError {
    EmptyPattern,
    DuplicatePattern(String),
    InvalidRegexPattern(String),
}
