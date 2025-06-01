use crate::*;

impl StdError for ServerError {}

impl Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TcpBind(err) => write!(f, "Tcp bind error{}{}", COLON_SPACE, err),
            Self::Unknown(err) => write!(f, "Unknown error{}{}", COLON_SPACE, err),
            Self::HttpRead(err) => write!(f, "Http read error{}{}", COLON_SPACE, err),
            Self::InvalidHttpRequest(err) => {
                write!(f, "Invalid http request{}{}", COLON_SPACE, err)
            }
        }
    }
}

impl StdError for RouteError {}

impl Display for RouteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPattern => {
                write!(f, "Route pattern cannot be empty")
            }
            Self::DuplicatePattern(err) => {
                write!(f, "Route pattern already exists{}{}", COLON_SPACE, err)
            }
            Self::InvalidRegexPattern(err) => {
                write!(f, "Invalid regex pattern{}{}", COLON_SPACE, err)
            }
        }
    }
}
