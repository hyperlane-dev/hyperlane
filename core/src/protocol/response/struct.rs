use super::*;

/// A parsed and validated protocol response head ready to emit directly.
#[derive(Debug)]
pub(crate) struct ParsedProtocolHead {
    /// The HTTP/2 or HTTP/3 response head with hop-by-hop headers removed.
    pub(crate) response: ProtocolResponse<()>,
    /// The validated content length announced by the head, if any.
    pub(crate) announced_len: Option<usize>,
    /// Whether body bytes must be suppressed for this response.
    pub(crate) suppress_body: bool,
}
