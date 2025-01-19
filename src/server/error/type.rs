use crate::*;

#[derive(Debug)]
pub enum Error {
    TcpBindError(String),
    HttpReadError(String),
    InvalidHttpRequest(RequestError),
    Unknown,
}
