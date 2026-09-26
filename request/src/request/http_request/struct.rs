use super::*;

/// Result of an HTTP request send.
///
/// Always `Result<HttpResponse, RequestError>` — no boxed trait object,
/// no associated types. The response is a value type: clone it cheaply
/// (`HttpResponse: Clone`) or pass it around by reference.
pub type RequestResult = Result<HttpResponse, RequestError>;

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
#[derive(Clone, Debug, Default)]
pub struct HttpRequest {
    /// HTTP method (`Method::Get` / `Method::Post` / ...).
    pub method: Method,
    /// Target URL (string form, parsed on send).
    pub url: String,
    /// Request headers (single-value).
    pub headers: HashMap<String, String>,
    /// Request body.
    pub body: Body,
    /// Per-request config: timeout, redirects, proxy, decode, etc.
    pub config: RequestConfig,
    /// Internal scratch: redirect-loop tracking + TLS root store.
    /// Private; only `HttpRequest` methods touch it.
    pub(crate) tmp: Tmp,
}

impl HttpRequest {
    /// Create a GET request for `url`.
    pub fn get(url: impl Into<String>) -> Self {
        Self {
            method: Method::Get,
            url: url.into(),
            headers: HashMap::new(),
            body: Body::default(),
            config: RequestConfig::default(),
            tmp: Tmp::default(),
        }
    }

    /// Create a POST request for `url`.
    pub fn post(url: impl Into<String>) -> Self {
        Self {
            method: Method::Post,
            url: url.into(),
            headers: HashMap::new(),
            body: Body::default(),
            config: RequestConfig::default(),
            tmp: Tmp::default(),
        }
    }

    /// Builder-style: set method.
    pub fn set_method(&mut self, method: Method) -> &mut Self {
        self.method = method;
        self
    }

    /// Builder-style: set URL.
    pub fn set_url(&mut self, url: impl Into<String>) -> &mut Self {
        self.url = url.into();
        self
    }

    /// Builder-style: set a single header (case-insensitive on read; the
    /// last value wins for repeated keys).
    pub fn set_header<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) -> &mut Self {
        let normalized = Self::normalize_header_key(key.as_ref());
        self.headers.insert(normalized, value.as_ref().to_owned());
        self
    }

    /// Remove a header by key.
    pub fn remove_header<K: AsRef<str>>(&mut self, key: K) -> &mut Self {
        let normalized = Self::normalize_header_key(key.as_ref());
        self.headers.remove(&normalized);
        self
    }

    /// Clear all headers.
    pub fn clear_headers(&mut self) -> &mut Self {
        self.headers.clear();
        self
    }

    /// Set body (raw bytes).
    pub fn set_body(&mut self, body: Body) -> &mut Self {
        self.body = body;
        self
    }

    /// Set a single config field by mutating the embedded `RequestConfig`.
    pub fn set_config(&mut self, config: RequestConfig) -> &mut Self {
        self.config = config;
        self
    }

    /// Normalize a header key to lowercase so `set_header` /
    /// `remove_header` lookups are case-insensitive.
    fn normalize_header_key(key: &str) -> String {
        key.to_ascii_lowercase()
    }
}
