/// The header name for content encoding.
pub(crate) const CONTENT_ENCODING: &str = "content-encoding";

/// An empty string.
pub(crate) const EMPTY_STR: &str = "";

/// The content encoding type for gzip.
pub(crate) const CONTENT_ENCODING_GZIP: &str = "gzip";

/// The content encoding type for deflate.
pub(crate) const CONTENT_ENCODING_DEFLATE: &str = "deflate";

/// The content encoding type for brotli.
pub(crate) const CONTENT_ENCODING_BROTLI: &str = "br";

/// The default brotli compression quality.
pub(crate) const BROTLI_DEFAULT_QUALITY: u32 = 5;

/// The default brotli sliding window size, in bits.
pub(crate) const BROTLI_DEFAULT_WINDOW_BITS: u32 = 22;
