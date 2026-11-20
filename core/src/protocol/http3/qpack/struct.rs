/// Metadata extracted while decoding an HTTP/3 request HEADERS frame in place.
#[derive(Clone, Copy, Debug)]
pub(crate) struct H3DecodedRequestMeta {
    /// Declared `content-length`, if present.
    pub(crate) declared_content_length: Option<usize>,
    /// Whether the request carries `Expect: 100-continue`.
    pub(crate) expect_continue: bool,
    /// Whether the request method is `HEAD`.
    pub(crate) is_head: bool,
}

/// Metadata extracted while encoding an HTTP/1.1 response head straight into an HTTP/3 HEADERS frame.
#[derive(Clone, Copy, Debug)]
pub(crate) struct H3EncodedResponseMeta {
    /// Validated `content-length` announced in the response head, if any.
    pub(crate) announced_len: Option<usize>,
    /// Whether body bytes must be suppressed (HEAD, 1xx, 204, 304).
    pub(crate) suppress_body: bool,
}

/// Binary trie node used once at startup to build the 8-bit Huffman lookup tables.
#[derive(Clone, Copy)]
pub(super) struct HuffmanTrieNode {
    /// Child node index for bit `0`, or `u16::MAX` when absent.
    pub(super) left: u16,
    /// Child node index for bit `1`, or `u16::MAX` when absent.
    pub(super) right: u16,
    /// Decoded byte symbol at a leaf, or `u16::MAX` for an internal node.
    pub(super) sym: u16,
}
