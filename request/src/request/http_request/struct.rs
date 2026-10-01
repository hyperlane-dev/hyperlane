use super::*;

/// HTTP request data and execution state.
///
/// Holds everything needed to issue one outgoing request: method, URL,
/// headers (single-value `HashMap<String, String>` — see
/// `hyperlane-standards §8.1`), body, and per-request config (timeout,
/// redirect policy, proxy, decode, etc.).
///
/// `HttpRequest` is constructed either via [`crate::RequestBuilder`] or
/// directly from its public fields, then sent via [`HttpRequest::send`]
/// (sync) or [`HttpRequest::send_async`].
#[derive(Clone, Debug, Default, GetterMut)]
pub struct HttpRequest {
    /// HTTP method (`Method::Get` / `Method::Post` / ...).
    #[get_mut(skip)]
    pub method: Method,
    /// Target URL (string form, parsed on send).
    #[get_mut(skip)]
    pub url: String,
    /// Request headers (single-value).
    pub headers: HashMap<String, String>,
    /// Request body.
    #[get_mut(skip)]
    pub body: Body,
    /// Per-request config: timeout, redirects, proxy, decode, etc.
    #[get_mut(skip)]
    pub config: RequestConfig,
    /// Internal scratch: redirect-loop tracking + TLS root store.
    /// Private; only `HttpRequest` methods touch it.
    #[get_mut(skip)]
    pub(crate) tmp: Tmp,
}
