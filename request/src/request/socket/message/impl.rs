use super::*;

impl WebSocketMessage {
    pub fn text<T: ToString>(text: T) -> Self {
        Self::Text(text.to_string())
    }

    pub fn binary<T: Into<Vec<u8>>>(data: T) -> Self {
        Self::Binary(data.into())
    }

    pub fn ping<T: Into<Vec<u8>>>(data: T) -> Self {
        Self::Ping(data.into())
    }

    pub fn pong<T: Into<Vec<u8>>>(data: T) -> Self {
        Self::Pong(data.into())
    }

    pub fn close() -> Self {
        Self::Close
    }

    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text(_))
    }

    pub fn is_binary(&self) -> bool {
        matches!(self, Self::Binary(_))
    }

    pub fn is_ping(&self) -> bool {
        matches!(self, Self::Ping(_))
    }

    pub fn is_pong(&self) -> bool {
        matches!(self, Self::Pong(_))
    }

    pub fn is_close(&self) -> bool {
        matches!(self, Self::Close)
    }

    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(text) => Some(text),
            _ => None,
        }
    }

    pub fn as_binary(&self) -> Option<&[u8]> {
        match self {
            Self::Binary(data) => Some(data),
            _ => None,
        }
    }

    pub fn into_text(self) -> Option<String> {
        match self {
            Self::Text(text) => Some(text),
            _ => None,
        }
    }

    pub fn into_binary(self) -> Option<Vec<u8>> {
        match self {
            Self::Binary(data) => Some(data),
            _ => None,
        }
    }
}
