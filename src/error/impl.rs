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
    pub(crate) fn new(
        message: String,
        location: Option<String>,
        payload: String,
        request_id: Option<String>,
        request_method: Option<String>,
        request_path: Option<String>,
        remote_addr: Option<String>,
        user_agent: Option<String>,
        request_duration: Option<Duration>,
    ) -> Self {
        Self {
            message,
            location,
            payload,
            request_id,
            request_method,
            request_path,
            remote_addr,
            user_agent,
            request_duration,
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
        let request_context: Option<RequestContextRef> = get_current_request_context();
        let (request_id, request_method, request_path, remote_addr, user_agent, request_duration) =
            if let Some(ctx) = request_context {
                (
                    Some(ctx.get_request_id().to_string()),
                    Some(ctx.get_method().to_string()),
                    Some(ctx.get_path().to_string()),
                    ctx.get_remote_addr().clone(),
                    ctx.get_user_agent().clone(),
                    Some(ctx.get_duration()),
                )
            } else {
                (None, None, None, None, None, None)
            };
        Self {
            message,
            location,
            payload,
            request_id,
            request_method,
            request_path,
            remote_addr,
            user_agent,
            request_duration,
        }
    }

    fn format_request_info(&self) -> String {
        let mut result: String = String::new();
        if let Some(request_id) = &self.request_id {
            result.push_str(BR);
            result.push_str("Request ID: ");
            result.push_str(request_id);
        }
        if let Some(method) = &self.request_method {
            result.push_str(BR);
            result.push_str("Request Method: ");
            result.push_str(method);
        }
        if let Some(path) = &self.request_path {
            result.push_str(BR);
            result.push_str("Request Path: ");
            result.push_str(path);
        }
        if let Some(addr) = &self.remote_addr {
            result.push_str(BR);
            result.push_str("Remote Address: ");
            result.push_str(addr);
        }
        if let Some(ua) = &self.user_agent {
            result.push_str(BR);
            result.push_str("User Agent: ");
            result.push_str(ua);
        }
        if let Some(duration) = &self.request_duration {
            result.push_str(BR);
            result.push_str("Request Duration: ");
            result.push_str(&format!("{:.3}ms", duration.as_secs_f64() * 1000.0));
        }
        result
    }
}

impl Display for PanicInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let payload: &str = self.get_payload();
        let message: &str = self.get_message();
        let formatted_payload: String = payload
            .lines()
            .map(|line| format!("Panic payload: {}", line))
            .collect::<Vec<_>>()
            .join(BR);
        let formatted_message: String = message
            .lines()
            .map(|line| format!("Panic message: {}", line))
            .collect::<Vec<_>>()
            .join(BR);
        let formatted_location: String = match self.get_location() {
            Some(location) => {
                let mut result: String = String::new();
                result.push_str(BR);
                for line in location.to_string().lines() {
                    result.push_str("Panic location: ");
                    result.push_str(line);
                    result.push_str(BR);
                }
                if result.ends_with(BR) {
                    result.truncate(result.len() - BR.len());
                }
                result
            }
            None => String::new(),
        };
        let formatted_request_info: String = self.format_request_info();
        write!(
            f,
            "{}{}{}{}{}",
            formatted_payload, BR, formatted_message, formatted_location, formatted_request_info
        )
    }
}
