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

impl PanicInfo {
    pub(crate) fn new(message: String, location: Option<String>, payload: String) -> Self {
        Self {
            message,
            location,
            payload,
        }
    }

    pub(crate) fn from_panic_hook_info(info: &PanicHookInfo<'_>) -> Self {
        let message: String = if let Some(s) = info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic".to_string()
        };
        let location: Option<String> = info
            .location()
            .map(|loc| format!("{}:{}:{}", loc.file(), loc.line(), loc.column()));
        let payload: String = info.to_string();
        Self {
            message,
            location,
            payload,
        }
    }
}

impl Display for PanicInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let payload: &str = self.get_payload();
        let message: &str = self.get_message();
        let mut parts: Vec<String> = Vec::new();
        if !payload.is_empty() {
            let formatted_payload: String = payload
                .lines()
                .map(|line| format!("Panic payload: {}", line))
                .collect::<Vec<_>>()
                .join(BR);
            parts.push(formatted_payload);
        }
        if !message.is_empty() {
            let formatted_message: String = message
                .lines()
                .map(|line| format!("Panic message: {}", line))
                .collect::<Vec<_>>()
                .join(BR);
            parts.push(formatted_message);
        }
        if let Some(location) = self.get_location() {
            let mut formatted_location: String = String::new();
            for line in location.to_string().lines() {
                if !formatted_location.is_empty() {
                    formatted_location.push_str(BR);
                }
                formatted_location.push_str("Panic location: ");
                formatted_location.push_str(line);
            }
            if !formatted_location.is_empty() {
                parts.push(formatted_location);
            }
        }
        write!(f, "{}", parts.join(BR))
    }
}
