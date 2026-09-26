use super::*;

/// Fluent builder for [`HttpRequest`].
///
/// Single-purpose, chainable. Each method returns `&mut Self` for fluent
/// chaining; call [`RequestBuilder::build`] to obtain the final
/// [`HttpRequest`].
///
/// # Examples
///
/// ```ignore
/// use http_request::{RequestBuilder, Proxy};
///
/// // sync GET
/// let mut req = RequestBuilder::new()
///     .get("https://example.com/api")
///     .header("Accept", "application/json")
///     .timeout(5_000)
///     .build();
/// let _resp = req.send();
///
/// // async POST with JSON body
/// let req = RequestBuilder::new()
///     .post("https://example.com/api")
///     .body_json(&serde_json::json!({"k": "v"}))
///     .header("Content-Type", "application/json")
///     .proxy(Proxy::https("127.0.0.1", 7890).auth("u", "p"))
///     .build();
/// let _resp = req.send_async().await;
/// ```
#[derive(Clone, Debug, Default)]
pub struct RequestBuilder {
    request: HttpRequest,
}

impl RequestBuilder {
    /// Create an empty builder (defaults: GET, no headers, default config).
    pub fn new() -> Self {
        Self::default()
    }

    /// Shortcut for `method(Method::Get)` + `url(url)`.
    pub fn get(&mut self, url: impl Into<String>) -> &mut Self {
        self.request.set_method(Method::Get);
        self.request.set_url(url);
        self
    }

    /// Shortcut for `method(Method::Post)` + `url(url)`.
    pub fn post(&mut self, url: impl Into<String>) -> &mut Self {
        self.request.set_method(Method::Post);
        self.request.set_url(url);
        self
    }

    /// Set HTTP method explicitly (`Method::Get` / `Method::Post` / etc.).
    pub fn method(&mut self, method: Method) -> &mut Self {
        self.request.set_method(method);
        self
    }

    /// Set URL.
    pub fn url(&mut self, url: impl Into<String>) -> &mut Self {
        self.request.set_url(url);
        self
    }

    /// Set a single header (last write wins on duplicate keys).
    pub fn header<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) -> &mut Self {
        self.request.set_header(key, value);
        self
    }

    /// Set many headers at once.
    pub fn headers<K, V>(&mut self, headers: HashMap<K, V>) -> &mut Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        for (k, v) in headers {
            self.request.set_header(k, v);
        }
        self
    }

    /// Remove a header by key.
    pub fn remove_header<K: AsRef<str>>(&mut self, key: K) -> &mut Self {
        self.request.remove_header(key);
        self
    }

    /// Clear all headers.
    pub fn clear_headers(&mut self) -> &mut Self {
        self.request.clear_headers();
        self
    }

    /// Set raw body bytes.
    pub fn body<B: Into<Vec<u8>>>(&mut self, bytes: B) -> &mut Self {
        self.request.set_body(Body::from_bytes(bytes));
        self
    }

    /// Set UTF-8 text body (will be encoded per `Content-Type` on send).
    pub fn body_text<T: Into<String>>(&mut self, text: T) -> &mut Self {
        self.request
            .set_body(Body::from_bytes(text.into().into_bytes()));
        self
    }

    /// Set JSON body (serialised via `serde_json`).
    pub fn body_json<V: serde::Serialize>(&mut self, value: &V) -> &mut Self {
        if let Ok(bytes) = serde_json::to_vec(value) {
            self.request.set_body(Body::from_bytes(bytes));
        }
        self
    }

    /// Set request timeout in milliseconds.
    pub fn timeout(&mut self, ms: u64) -> &mut Self {
        self.request.get_config_mut().set_timeout(ms);
        self
    }

    /// Set per-read buffer size.
    pub fn buffer_size(&mut self, n: usize) -> &mut Self {
        self.request.get_config_mut().set_buffer_size(n);
        self
    }

    /// Force HTTP/1.1.
    pub fn http1_1_only(&mut self) -> &mut Self {
        self.request.get_config_mut().http_version = HttpVersion::Http1_1;
        self
    }

    /// Force HTTP/2.
    pub fn http2_only(&mut self) -> &mut Self {
        self.request.get_config_mut().http_version = HttpVersion::Http2;
        self
    }

    /// Enable auto-follow of 3xx redirects.
    pub fn redirect(&mut self) -> &mut Self {
        self.request.get_config_mut().set_redirect(true);
        self
    }

    /// Disable auto-follow of 3xx redirects (default).
    pub fn no_redirect(&mut self) -> &mut Self {
        self.request.get_config_mut().set_redirect(false);
        self
    }

    /// Maximum number of redirects to follow (default `DEFAULT_MAX_REDIRECT_TIMES`).
    pub fn max_redirect_times(&mut self, n: usize) -> &mut Self {
        self.request.get_config_mut().set_max_redirect_times(n);
        self
    }

    /// Enable automatic response body decompression (gzip / deflate / br).
    pub fn decode(&mut self) -> &mut Self {
        self.request.get_config_mut().set_decode(true);
        self
    }

    /// Disable automatic response body decompression.
    pub fn no_decode(&mut self) -> &mut Self {
        self.request.get_config_mut().set_decode(false);
        self
    }

    /// Set the proxy configuration. Pass `None` (via [`Proxy::default()`) to
    /// clear an existing proxy.
    ///
    /// Construct via [`Proxy::http`] / [`Proxy::https`] / [`Proxy::socks5`]
    /// and optionally chain `.auth(user, pass)`.
    pub fn proxy(&mut self, proxy: Proxy) -> &mut Self {
        self.request.get_config_mut().set_proxy(Some(proxy));
        self
    }

    /// Clear the proxy (use direct connection).
    pub fn no_proxy(&mut self) -> &mut Self {
        self.request.get_config_mut().set_proxy(None);
        self
    }

    /// Finalise the builder and return the [`HttpRequest`].
    pub fn build(&mut self) -> HttpRequest {
        std::mem::take(&mut self.request)
    }
}
