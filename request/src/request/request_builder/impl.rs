use super::*;

/// Inherent implementation of [`RequestBuilder`].
impl RequestBuilder {
    /// Create an empty builder (defaults: GET, no headers, default config).
    ///
    /// # Returns
    ///
    /// - `Self` - A fresh request builder with default settings.
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
    pub fn get<T>(&mut self, url: T) -> &mut Self
    where
        T: Into<String>,
    {
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
    pub fn post<T>(&mut self, url: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.get_mut_request().set_method(Method::Post);
        self.get_mut_request().set_url(url);
        self
    }

    /// Shortcut for `method(Method::Put)` + `url(url)`.
    ///
    /// # Arguments
    ///
    /// - `T` - A value convertible into the request URL, such as `&str` or `String`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn put<T>(&mut self, url: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.get_mut_request().set_method(Method::Put);
        self.get_mut_request().set_url(url);
        self
    }

    /// Shortcut for `method(Method::Delete)` + `url(url)`.
    ///
    /// # Arguments
    ///
    /// - `T` - A value convertible into the request URL, such as `&str` or `String`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn delete<T>(&mut self, url: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.get_mut_request().set_method(Method::Delete);
        self.get_mut_request().set_url(url);
        self
    }

    /// Shortcut for `method(Method::Patch)` + `url(url)`.
    ///
    /// # Arguments
    ///
    /// - `T` - A value convertible into the request URL, such as `&str` or `String`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn patch<T>(&mut self, url: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.get_mut_request().set_method(Method::Patch);
        self.get_mut_request().set_url(url);
        self
    }

    /// Shortcut for `method(Method::Head)` + `url(url)`.
    ///
    /// # Arguments
    ///
    /// - `T` - A value convertible into the request URL, such as `&str` or `String`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn head<T>(&mut self, url: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.get_mut_request().set_method(Method::Head);
        self.get_mut_request().set_url(url);
        self
    }

    /// Shortcut for `method(Method::Options)` + `url(url)`.
    ///
    /// # Arguments
    ///
    /// - `T` - A value convertible into the request URL, such as `&str` or `String`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn options<T>(&mut self, url: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.get_mut_request().set_method(Method::Options);
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
    pub fn url<T>(&mut self, url: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.get_mut_request().set_url(url);
        self
    }

    /// Append a percent-encoded query parameter to the request URL.
    ///
    /// # Arguments
    ///
    /// - `K` - The query parameter name.
    /// - `V` - The query parameter value.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn query<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let current: String = self.get_request().get_url().clone();
        let (base, fragment): (&str, Option<&str>) = match current.split_once(URL_FRAGMENT) {
            Some((prefix, frag)) => (prefix, Some(frag)),
            None => (current.as_str(), None),
        };
        let sep: char = if base.contains(URL_QUERY) {
            if base.ends_with(URL_QUERY) || base.ends_with('&') {
                '\0'
            } else {
                '&'
            }
        } else {
            URL_QUERY
        };
        let mut updated: String = String::with_capacity(
            current.len() + key.as_ref().len() * 3 + value.as_ref().len() * 3 + 2,
        );
        updated.push_str(base);
        if sep != '\0' {
            updated.push(sep);
        }
        push_percent_encoded(key.as_ref(), &mut updated);
        updated.push('=');
        push_percent_encoded(value.as_ref(), &mut updated);
        if let Some(frag) = fragment {
            updated.push(URL_FRAGMENT);
            updated.push_str(frag);
        }
        self.get_mut_request().set_url(updated);
        self
    }

    /// Append multiple percent-encoded query parameters to the request URL.
    ///
    /// # Arguments
    ///
    /// - `I` - Iterator yielding `(key, value)` pairs.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn query_pairs<I, K, V>(&mut self, pairs: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        for (key, value) in pairs {
            self.query(key, value);
        }
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
    pub fn header<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
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

    /// Set an `Authorization: Bearer <token>` header.
    ///
    /// # Arguments
    ///
    /// - `T` - The bearer token string.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn bearer_auth<T>(&mut self, token: T) -> &mut Self
    where
        T: AsRef<str>,
    {
        let value: String = format!("{BEARER_PREFIX}{}", token.as_ref());
        self.get_mut_request()
            .set_header(HEADER_AUTHORIZATION, value);
        self
    }

    /// Set an `Authorization: Basic <base64(username:password)>` header.
    ///
    /// # Arguments
    ///
    /// - `U` - The username string.
    /// - `P` - The password string.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn basic_auth<U, P>(&mut self, username: U, password: P) -> &mut Self
    where
        U: AsRef<str>,
        P: AsRef<str>,
    {
        let raw: String = format!("{}:{}", username.as_ref(), password.as_ref());
        let encoded: String = crate::utils::base64_encode(raw.as_bytes());
        let value: String = format!("{BASIC_PREFIX}{encoded}");
        self.get_mut_request()
            .set_header(HEADER_AUTHORIZATION, value);
        self
    }

    /// Append a `name=value` cookie to the outgoing `Cookie` header.
    ///
    /// # Arguments
    ///
    /// - `K` - Cookie name.
    /// - `V` - Cookie value.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn cookie<K, V>(&mut self, name: K, value: V) -> &mut Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let pair: String = format!("{}={}", name.as_ref(), value.as_ref());
        let existing: Option<String> = self
            .get_request()
            .get_headers()
            .iter()
            .find(|(k, _): &(&String, &String)| k.eq_ignore_ascii_case(COOKIE))
            .map(|(_, v): (&String, &String)| v.clone());
        let merged: String = match existing {
            Some(prev) if !prev.is_empty() => format!("{prev}; {pair}"),
            _ => pair,
        };
        self.get_mut_request().set_header(COOKIE, merged);
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
    pub fn remove_header<K>(&mut self, key: K) -> &mut Self
    where
        K: AsRef<str>,
    {
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
    pub fn body<B>(&mut self, bytes: B) -> &mut Self
    where
        B: Into<Vec<u8>>,
    {
        self.get_mut_request().set_body(Body::from_bytes(bytes));
        self
    }

    /// Set UTF-8 text body.
    ///
    /// # Arguments
    ///
    /// - `T` - A value convertible into the text body, such as `&str` or `String`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn body_text<T>(&mut self, text: T) -> &mut Self
    where
        T: Into<String>,
    {
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
    pub fn body_json<V>(&mut self, value: &V) -> &mut Self
    where
        V: Serialize,
    {
        if let Ok(bytes) = to_vec(value) {
            self.get_mut_request().set_body(Body::from_bytes(bytes));
        }
        self
    }

    /// Set an `application/x-www-form-urlencoded` body and matching `Content-Type` header.
    ///
    /// # Arguments
    ///
    /// - `I` - Iterator yielding `(key, value)` form fields.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn body_form<I, K, V>(&mut self, fields: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let mut encoded: String = String::new();
        for (key, value) in fields {
            if !encoded.is_empty() {
                encoded.push('&');
            }
            push_percent_encoded(key.as_ref(), &mut encoded);
            encoded.push('=');
            push_percent_encoded(value.as_ref(), &mut encoded);
        }
        self.get_mut_request()
            .set_header(CONTENT_TYPE, FORM_URLENCODED_CONTENT_TYPE);
        self.get_mut_request()
            .set_body(Body::from_bytes(encoded.into_bytes()));
        self
    }

    /// Set an RFC 7578 `multipart/form-data` body from `http_type::BodyPart` entries.
    ///
    /// # Arguments
    ///
    /// - `&[BodyPart]` - Multipart fields and uploaded file parts.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn body_multipart(&mut self, parts: &[BodyPart]) -> &mut Self {
        let (content_type, bytes): (String, Vec<u8>) = encode_multipart_parts(parts);
        self.get_mut_request()
            .set_header(CONTENT_TYPE, content_type);
        self.get_mut_request().set_body(Body::from_bytes(bytes));
        self
    }

    /// Add one or more PEM-encoded root CA certificates for TLS verification.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - Raw PEM-encoded X.509 certificate bytes.
    ///
    /// # Returns
    ///
    /// - `Result<&mut Self, RequestError>` - Updated builder or an error when PEM parsing fails.
    pub fn root_cert_pem(&mut self, pem_bytes: &[u8]) -> Result<&mut Self, RequestError> {
        let roots: &mut RootCertStore = self.get_mut_request().get_mut_tmp().get_mut_root_cert();
        let _: usize = add_pem_certs_to_store(roots, pem_bytes)?;
        Ok(self)
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
        self.get_mut_request().get_mut_config().set_timeout(ms);
        self
    }

    /// Set dedicated TCP connect timeout in milliseconds (`0` inherits `timeout`).
    ///
    /// # Arguments
    ///
    /// - `u64` - The connect timeout in milliseconds.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn connect_timeout(&mut self, ms: u64) -> &mut Self {
        self.get_mut_request()
            .get_mut_config()
            .set_connect_timeout(ms);
        self
    }

    /// Configure automatic bounded retries for idempotent transient failures (`0` = disabled).
    ///
    /// # Arguments
    ///
    /// - `usize` - Maximum number of retry attempts.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn retry(&mut self, max_retries: usize) -> &mut Self {
        self.get_mut_request()
            .get_mut_config()
            .set_max_retries(max_retries);
        self
    }

    /// Enable or disable HTTP keep-alive connection pooling for this request.
    ///
    /// # Arguments
    ///
    /// - `bool` - `true` to reuse persistent HTTP/1.1 or HTTP/2 connections from the pool, `false` to close after use.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn keep_alive(&mut self, enabled: bool) -> &mut Self {
        self.get_mut_request()
            .get_mut_config()
            .set_keep_alive(enabled);
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
        self.get_mut_request().get_mut_config().set_buffer_size(n);
        self
    }

    /// Force HTTP/1.1.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn http1_1_only(&mut self) -> &mut Self {
        self.get_mut_request()
            .get_mut_config()
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
            .get_mut_config()
            .set_http_version(HttpVersion::Http2);
        self
    }

    /// Enable auto-follow of 3xx redirects.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn redirect(&mut self) -> &mut Self {
        self.get_mut_request().get_mut_config().set_redirect(true);
        self
    }

    /// Disable auto-follow of 3xx redirects (default).
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn no_redirect(&mut self) -> &mut Self {
        self.get_mut_request().get_mut_config().set_redirect(false);
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
            .get_mut_config()
            .set_max_redirect_times(n);
        self
    }

    /// Enable automatic response body decompression (gzip / deflate / br).
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn decode(&mut self) -> &mut Self {
        self.get_mut_request().get_mut_config().set_decode(true);
        self
    }

    /// Disable automatic response body decompression.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn no_decode(&mut self) -> &mut Self {
        self.get_mut_request().get_mut_config().set_decode(false);
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
            .get_mut_config()
            .set_proxy(Some(proxy));
        self
    }

    /// Clear the proxy (use direct connection).
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to this builder for chaining.
    pub fn no_proxy(&mut self) -> &mut Self {
        self.get_mut_request().get_mut_config().set_proxy(None);
        self
    }

    /// Finalise the builder and return the [`HttpRequest`].
    ///
    /// # Returns
    ///
    /// - `HttpRequest` - The finalised request, leaving the builder empty.
    pub fn build(&mut self) -> HttpRequest {
        mem::take(self.get_mut_request())
    }
}
