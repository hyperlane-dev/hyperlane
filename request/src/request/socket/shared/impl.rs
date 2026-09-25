use super::*;

impl std::fmt::Display for WebSocketError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            WebSocketErrorKind::Connection => write!(f, "Connection error: {}", self.message),
            WebSocketErrorKind::String => write!(f, "String error: {}", self.message),
            WebSocketErrorKind::Timeout => write!(f, "Timeout error: {}", self.message),
            WebSocketErrorKind::InvalidUrl => write!(f, "Invalid URL: {}", self.message),
            WebSocketErrorKind::Io => write!(f, "IO error: {}", self.message),
            WebSocketErrorKind::Tls => write!(f, "TLS error: {}", self.message),
        }
    }
}

impl std::error::Error for WebSocketError {}

impl WebSocketError {
    pub(crate) fn connection<T: ToString>(message: T) -> Self {
        Self {
            kind: WebSocketErrorKind::Connection,
            message: message.to_string(),
        }
    }

    pub(crate) fn protocol<T: ToString>(message: T) -> Self {
        Self {
            kind: WebSocketErrorKind::String,
            message: message.to_string(),
        }
    }

    pub(crate) fn timeout<T: ToString>(message: T) -> Self {
        Self {
            kind: WebSocketErrorKind::Timeout,
            message: message.to_string(),
        }
    }

    pub(crate) fn invalid_url<T: ToString>(message: T) -> Self {
        Self {
            kind: WebSocketErrorKind::InvalidUrl,
            message: message.to_string(),
        }
    }

    pub(crate) fn io<T: ToString>(message: T) -> Self {
        Self {
            kind: WebSocketErrorKind::Io,
            message: message.to_string(),
        }
    }

    pub(crate) fn tls<T: ToString>(message: T) -> Self {
        Self {
            kind: WebSocketErrorKind::Tls,
            message: message.to_string(),
        }
    }
}

impl SharedWebSocketBuilder {
    pub(crate) fn parse_url(url: &str) -> Result<HttpUrlComponents, WebSocketError> {
        if url.is_empty() {
            return Err(WebSocketError::invalid_url("URL is empty"));
        }
        let mut url_obj: HttpUrlComponents = HttpUrlComponents::default();
        if url.starts_with("ws://") {
            url_obj.protocol = HTTP_LOWERCASE.to_string();
            url_obj.port = Some(80);
        } else if url.starts_with("wss://") {
            url_obj.protocol = HTTP_UPPERCASE.to_string();
            url_obj.port = Some(443);
        } else {
            return Err(WebSocketError::invalid_url("Invalid WebSocket URL scheme"));
        }
        let without_protocol: &str = if let Some(stripped) = url.strip_prefix("ws://") {
            stripped
        } else {
            &url[6..]
        };
        let parts: Vec<&str> = without_protocol.splitn(2, '/').collect();
        let host_port: &str = parts[0];
        let path: &str = if parts.len() > 1 { parts[1] } else { "" };
        if host_port.contains(':') {
            let host_port_parts: Vec<&str> = host_port.splitn(2, ':').collect();
            url_obj.host = Some(host_port_parts[0].to_string());
            if let Ok(port) = host_port_parts[1].parse::<u16>() {
                url_obj.port = Some(port);
            }
        } else {
            url_obj.host = Some(host_port.to_string());
        }
        url_obj.path = Some(if path.is_empty() {
            "/".to_string()
        } else {
            format!("/{path}")
        });
        Ok(url_obj)
    }
}
