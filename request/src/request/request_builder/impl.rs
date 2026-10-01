use super::*;

impl RequestBuilder {
    /// Returns the underlying request mutably.
    ///
    /// # Returns
    ///
    /// - `&mut HttpRequest` - The mutable request being built.
    pub fn get_request_mut(&mut self) -> &mut HttpRequest {
        &mut self.request
    }

    /// Create an empty builder (defaults: GET, no headers, default config).
    pub fn new() -> Self {
        Self::default()
    }

    /// Shortcut for `method(Method::Get)` + `url(url)`.
    ///
    /// # Arguments
    ///
    /// - `T` - A value convertible into the request URL, such as `&str` or `String`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn get<T: Into<String>>(&mut self, url: T) -> &mut Self {
        self.get_mut_request().set_method(Method::Get);
        self.get_mut_request().set_url(url);
        self
    }

    /// Shortcut for `method(Method::Post)` + `url(url)`.
    ///
    /// # Arguments
    ///
    /// - `T` - A value convertible into the request URL, such as `&str` or `String`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn post<T: Into<String>>(&mut self, url: T) -> &mut Self {
        self.get_mut_request().set_method(Method::Post);
        self.get_mut_request().set_url(url);
        self
    }

    /// Set HTTP method explicitly (`Method::Get` / `Method::Post` / etc.).
    ///
    /// # Arguments
    ///
    /// - `Method` - The HTTP method to apply to the request.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn method(&mut self, method: Method) -> &mut Self {
        self.get_mut_request().set_method(method);
        self
    }

    /// Set URL.
    ///
    /// # Arguments
    ///
    /// - `T` - A value convertible into the request URL, such as `&str` or `String`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn url<T: Into<String>>(&mut self, url: T) -> &mut Self {
        self.get_mut_request().set_url(url);
        self
    }

    /// Set a single header (last write wins on duplicate keys).
    ///
    /// # Arguments
    ///
    /// - `K` - A header name that can be borrowed as a string slice.
    /// - `V` - A header value that can be borrowed as a string slice.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn header<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) -> &mut Self {
        self.get_mut_request().set_header(key, value);
        self
    }

    /// Set many headers at once.
    ///
    /// # Arguments
    ///
    /// - `HashMap<K, V>` - The header name-value pairs to apply, where each name and
    ///   value can be borrowed as a string slice.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn headers<K, V>(&mut self, headers: HashMap<K, V>) -> &mut Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        for (k, v) in headers {
            self.get_mut_request().set_header(k, v);
        }
        self
    }

    /// Remove a header by key.
    ///
    /// # Arguments
    ///
    /// - `K` - The header name to remove, which can be borrowed as a string slice.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn remove_header<K: AsRef<str>>(&mut self, key: K) -> &mut Self {
        self.get_mut_request().remove_header(key);
        self
    }

    /// Clear all headers.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn clear_headers(&mut self) -> &mut Self {
        self.get_mut_request().clear_headers();
        self
    }

    /// Set raw body bytes.
    ///
    /// # Arguments
    ///
    /// - `B` - A value convertible into the raw body bytes, such as `Vec<u8>` or
    ///   `&[u8]`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn body<B: Into<Vec<u8>>>(&mut self, bytes: B) -> &mut Self {
        self.get_mut_request().set_body(Body::from_bytes(bytes));
        self
    }

    /// Set UTF-8 text body (will be encoded per `Content-Type` on send).
    ///
    /// # Arguments
    ///
    /// - `T` - A value convertible into the text body, such as `&str` or `String`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn body_text<T: Into<String>>(&mut self, text: T) -> &mut Self {
        self.get_mut_request()
            .set_body(Body::from_bytes(text.into().into_bytes()));
        self
    }

    /// Set JSON body (serialised via `serde_json`).
    ///
    /// # Arguments
    ///
    /// - `&V` - A reference to the value to serialise as the JSON body.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn body_json<V: serde::Serialize>(&mut self, value: &V) -> &mut Self {
        if let Ok(bytes) = serde_json::to_vec(value) {
            self.get_mut_request().set_body(Body::from_bytes(bytes));
        }
        self
    }

    /// Set request timeout in milliseconds.
    ///
    /// # Arguments
    ///
    /// - `u64` - The request timeout in milliseconds.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn timeout(&mut self, ms: u64) -> &mut Self {
        self.get_mut_request().get_config_mut().set_timeout(ms);
        self
    }

    /// Set per-read buffer size.
    ///
    /// # Arguments
    ///
    /// - `usize` - The per-read buffer size in bytes.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn buffer_size(&mut self, n: usize) -> &mut Self {
        self.get_mut_request().get_config_mut().set_buffer_size(n);
        self
    }

    /// Force HTTP/1.1.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn http1_1_only(&mut self) -> &mut Self {
        self.get_mut_request()
            .get_config_mut()
            .set_http_version(HttpVersion::Http1_1);
        self
    }

    /// Force HTTP/2.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn http2_only(&mut self) -> &mut Self {
        self.get_mut_request()
            .get_config_mut()
            .set_http_version(HttpVersion::Http2);
        self
    }

    /// Enable auto-follow of 3xx redirects.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn redirect(&mut self) -> &mut Self {
        self.get_mut_request().get_config_mut().set_redirect(true);
        self
    }

    /// Disable auto-follow of 3xx redirects (default).
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn no_redirect(&mut self) -> &mut Self {
        self.get_mut_request().get_config_mut().set_redirect(false);
        self
    }

    /// Maximum number of redirects to follow (default `DEFAULT_MAX_REDIRECT_TIMES`).
    ///
    /// # Arguments
    ///
    /// - `usize` - The maximum number of redirects to follow.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn max_redirect_times(&mut self, n: usize) -> &mut Self {
        self.get_mut_request()
            .get_config_mut()
            .set_max_redirect_times(n);
        self
    }

    /// Enable automatic response body decompression (gzip / deflate / br).
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn decode(&mut self) -> &mut Self {
        self.get_mut_request().get_config_mut().set_decode(true);
        self
    }

    /// Disable automatic response body decompression.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn no_decode(&mut self) -> &mut Self {
        self.get_mut_request().get_config_mut().set_decode(false);
        self
    }

    /// Set the proxy configuration. Pass `None` (via [`Proxy::default()`) to
    /// clear an existing proxy.
    ///
    /// Construct via [`Proxy::http`] / [`Proxy::https`] / [`Proxy::socks5`]
    /// and optionally chain `.auth(user, pass)`.
    ///
    /// # Arguments
    ///
    /// - `Proxy` - The proxy configuration to apply to the request.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn proxy(&mut self, proxy: Proxy) -> &mut Self {
        self.get_mut_request()
            .get_config_mut()
            .set_proxy(Some(proxy));
        self
    }

    /// Clear the proxy (use direct connection).
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn no_proxy(&mut self) -> &mut Self {
        self.get_mut_request().get_config_mut().set_proxy(None);
        self
    }

    /// Finalise the builder and return the [`HttpRequest`].
    ///
    /// # Returns
    ///
    /// - `HttpRequest` - The finalised request, leaving the builder empty.
    pub fn build(&mut self) -> HttpRequest {
        std::mem::take(self.get_mut_request())
    }
}
