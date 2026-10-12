use super::*;

/// Represents the components of a parsed HTTP URL.
///
/// This struct holds various parts of a URL, including the protocol, host, port,
/// path, query, and fragment, allowing for structured access to URL information.
///
/// The fields stay `pub`: `type/tests/http_url` reads them directly, so this is
/// published API and §17.14 does not recall it (semver). Mutators stay
/// `pub(super)` — nothing outside this crate builds the value.
#[derive(Clone, Data, Debug, Default, Eq, PartialEq)]
pub struct HttpUrlComponents {
    /// The URL scheme, such as "http" or "https".
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub protocol: String,
    /// The host part of the URL.
    #[get(type(clone))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub host: Option<String>,
    /// The port number in the URL, if specified.
    #[get(type(clone))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub port: Option<u16>,
    /// The path in the URL.
    #[get(type(clone))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub path: Option<String>,
    /// The query string in the URL.
    #[get(type(clone))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub query: Option<String>,
    /// The fragment identifier.
    #[get(type(clone))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub fragment: Option<String>,
}
