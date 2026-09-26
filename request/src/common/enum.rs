use super::*;

/// HTTP request body content.
///
/// Holds the raw bytes of the request body. Use [`RequestBuilder::body`] /
/// [`RequestBuilder::body_json`] / [`RequestBuilder::body_text`] on the builder
/// to populate; users normally do not construct `Body` directly.
///
/// `Body` is intentionally a single-value type (`Vec<u8>`) to match the
/// `Request` / `Response` design in `hyperlane-core` — see
/// `hyperlane-standards §8.1`. Higher-level framing (json vs text vs binary)
/// lives in the builder, not in the data type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct Body {
    /// Raw body bytes.
    pub bytes: Vec<u8>,
}

impl Body {
    /// Empty body.
    pub const fn empty() -> Self {
        Self { bytes: Vec::new() }
    }

    /// Construct from any `Into<Vec<u8>>`.
    pub fn from_bytes<B: Into<Vec<u8>>>(bytes: B) -> Self {
        Self {
            bytes: bytes.into(),
        }
    }

    /// View body as `&[u8]`.
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    /// Try to view body as UTF-8 string.
    pub fn as_str(&self) -> Option<&str> {
        std::str::from_utf8(&self.bytes).ok()
    }
}

impl Display for Body {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.as_str() {
            Some(s) => f.write_str(s),
            None => f.write_str(&format!("{:?}", self.bytes)),
        }
    }
}
