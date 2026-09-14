use super::*;

/// Builder for constructing HTTP cookies.
///
/// Provides methods to set various cookie attributes like expiration,
/// domain, path, and security flags before building the final cookie string.
#[derive(Clone, Data, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CookieBuilder {
    /// Cookie name identifier.
    #[set(type(AsRef<str>))]
    pub(super) name: CookieKey,
    /// Cookie value content.
    #[set(type(AsRef<str>))]
    pub(super) value: CookieValue,
    /// Optional expiration date/time string.
    #[set(skip)]
    pub(super) expires: Option<String>,
    /// Optional maximum age in seconds.
    #[set(skip)]
    pub(super) max_age: Option<i64>,
    /// Optional domain scope for the cookie.
    #[set(skip)]
    pub(super) domain: Option<String>,
    /// Optional path scope for the cookie.
    #[set(skip)]
    pub(super) path: Option<String>,
    /// Optional flag indicating secure (HTTPS-only) transmission.
    #[set(skip)]
    pub(super) secure: Option<bool>,
    /// Optional flag preventing JavaScript access.
    #[set(skip)]
    pub(super) http_only: Option<bool>,
    /// Optional SameSite policy setting.
    #[set(skip)]
    pub(super) same_site: Option<String>,
}

/// Parser for HTTP Cookie headers.
///
/// Provides functionality to parse Cookie header strings into key-value pairs.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Cookie;
