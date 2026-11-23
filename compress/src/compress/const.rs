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

/// The comma that separates two codings of an `Accept-Encoding` field value.
pub(crate) const ACCEPT_ENCODING_SEPARATOR: &str = ",";

/// The semicolon that introduces a parameter of a coding.
pub(crate) const PARAMETER_SEPARATOR: &str = ";";

/// The equal sign that joins a parameter name to its value.
pub(crate) const PARAMETER_ASSIGN: &str = "=";

/// The wildcard coding that stands for every coding a field does not name.
pub(crate) const ACCEPT_ENCODING_WILDCARD: &str = "*";

/// The parameter name that carries the relative quality of a coding.
pub(crate) const ACCEPT_ENCODING_QUALITY: &str = "q";

/// The quality a coding counts as when the field names no `q` parameter for it.
pub(crate) const DEFAULT_QUALITY: f32 = 1.0;
