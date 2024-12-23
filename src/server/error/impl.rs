use super::r#type::Error;
use std::{
    error::Error as StdError,
    fmt::{self, Display},
};

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TcpBindError => write!(f, "Tcp bind error"),
            Self::HttpReadError => write!(f, "Http read error"),
            Self::InvalidHttpRequest => write!(f, "Invalid http request"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
