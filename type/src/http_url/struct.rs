use super::*;

/// Represents the components of a parsed HTTP URL.
///
/// This struct holds various parts of a URL, including the protocol, host, port,
/// path, query, and fragment, allowing for structured access to URL information.
///
/// The fields are crate-internal (§17.18): a consumer reaches them through
/// the generated `get_*` accessors. Mutators stay `pub(super)` — nothing
/// outside this crate builds the value.
#[derive(Clone, Data, Debug, Default, Eq, PartialEq)]
pub struct HttpUrlComponents {
    /// The URL scheme, such as "http" or "https".
    pub(crate) protocol: String,
    /// The host part of the URL.
    #[get(type(clone))]
    pub(crate) host: Option<String>,
    /// The port number in the URL, if specified.
    #[get(type(clone))]
    pub(crate) port: Option<u16>,
    /// The path in the URL.
    #[get(type(clone))]
    pub(crate) path: Option<String>,
    /// The query string in the URL.
    #[get(type(clone))]
    pub(crate) query: Option<String>,
    /// The fragment identifier.
    #[get(type(clone))]
    pub(crate) fragment: Option<String>,
}
