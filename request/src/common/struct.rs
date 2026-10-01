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
#[derive(Clone, Debug, Default, Eq, Getter, PartialEq, Serialize)]
pub struct Body {
    /// Raw body bytes.
    pub bytes: Vec<u8>,
}
