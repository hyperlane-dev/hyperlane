use super::*;

impl RequestBuilder {
    /// Creates a new RequestBuilder instance.
    ///
    /// # Returns
    ///
    /// - `RequestBuilder` - A new builder instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the HTTP method to POST and the request URL.
    ///
    /// # Arguments
    ///
    /// - `&str` - The request URL.
    ///
    /// # Returns
    ///
    /// - `&mut RequestBuilder` - The builder for method chaining.
    pub fn post(&mut self, url: &str) -> &mut Self {
        self.http_request.methods = Arc::new(Method::Post);
        self.url(url);
        self
    }

    /// Sets the HTTP method to GET and the request URL.
    ///
    /// # Arguments
    ///
    /// - `&str` - The request URL.
    ///
    /// # Returns
    ///
    /// - `&mut RequestBuilder` - The builder for method chaining.
    pub fn get(&mut self, url: &str) -> &mut Self {
        self.http_request.methods = Arc::new(Method::Get);
        self.url(url);
        self
    }

    /// Sets the request URL.
    ///
    /// # Arguments
    ///
    /// - `&str` - The request URL.
    ///
    /// # Returns
    ///
    /// - `&mut RequestBuilder` - The builder for method chaining.
    fn url(&mut self, url: &str) -> &mut Self {
        self.http_request.url = Arc::new(url.to_owned());
        self
    }

    /// Forces HTTP/1.1 protocol version.
    ///
    /// # Returns
    ///
    /// - `&mut RequestBuilder` - The builder for method chaining.
    pub fn http1_1_only(&mut self) -> &mut Self {
        if let Ok(mut config) = self.http_request.config.write() {
            config.http_version = HttpVersion::Http1_1;
        }
        self
    }

    /// Forces HTTP/2 protocol version.
    ///
    /// # Returns
    ///
    /// - `&mut RequestBuilder` - The builder for method chaining.
    pub fn http2_only(&mut self) -> &mut Self {
        if let Ok(mut config) = self.http_request.config.write() {
            config.http_version = HttpVersion::Http2;
        }
        self
    }

    /// Sets request headers.
    ///
    /// # Arguments
    ///
    /// - `HashMapXxHash3_64<K, V>` - The headers to set.
    ///
    /// # Returns
    ///
    /// - `&mut RequestBuilder` - The builder for method chaining.
    pub fn headers<K, V>(&mut self, header: HashMapXxHash3_64<K, V>) -> &mut Self
    where
        K: ToString,
        V: ToString,
    {
        if let Some(tmp_header) = Arc::get_mut(&mut self.http_request.header) {
            for (key, value) in header {
                let key_str: String = key.to_string();
                let value_str: String = value.to_string();
                let mut found_existing: bool = false;
                let mut existing_key: Option<String> = None;
                for existing_key_ref in tmp_header.keys() {
                    if existing_key_ref.eq_ignore_ascii_case(&key_str) {
                        existing_key = Some(existing_key_ref.clone());
                        found_existing = true;
                        break;
                    }
                }
                if found_existing && let Some(existing_key) = existing_key {
                    tmp_header.remove(&existing_key);
                }
                let mut value_deque: VecDeque<String> = VecDeque::new();
                value_deque.push_front(value_str);
                tmp_header.insert(key_str, value_deque);
            }
        }
        self
    }

    /// Sets JSON request body.
    ///
    /// # Arguments
    ///
    /// - `Value` - The JSON body data.
    ///
    /// # Returns
    ///
    /// - `&mut RequestBuilder` - The builder for method chaining.
    pub fn json(&mut self, body: serde_json::Value) -> &mut Self {
        if let serde_json::Value::Object(map) = body {
            let mut res_body: HashMapXxHash3_64<String, serde_json::Value> = hash_map_xx_hash3_64();
            for (k, v) in map.iter() {
                res_body.insert(k.to_string(), v.clone());
            }
            self.http_request.body = Arc::new(Body::Json(res_body));
        }
        self
    }

    /// Sets plain text request body.
    ///
    /// # Arguments
    ///
    /// - `T` - The text body data (must implement ToString).
    ///
    /// # Returns
    ///
    /// - `&mut RequestBuilder` - The builder for method chaining.
    pub fn text<T: ToString>(&mut self, body: T) -> &mut Self {
        self.http_request.body = Arc::new(Body::Text(body.to_string()));
        self
    }

    /// Sets binary request body.
    ///
    /// # Arguments
    ///
    /// - `T` - The binary body data (must implement Into<Vec<u8>>).
    ///
    /// # Returns
    ///
    /// - `&mut RequestBuilder` - The builder for method chaining.
    pub fn body<T: Into<Vec<u8>>>(&mut self, body: T) -> &mut Self {
        self.http_request.body = Arc::new(Body::Binary(body.into()));
        self
    }

    /// Sets the timeout value for the current connection.
    ///
    /// This method sets the timeout duration for the connection, which is used to determine
    /// how long the system should wait for a response before considering the connection attempt
    /// as failed. The timeout value is stored in an `Arc` to allow it to be shared safely across
    /// multiple threads if needed.
    ///
    /// # Arguments
    ///
    /// - `u64` - The timeout duration in milliseconds. This value will be used to configure the
    ///   connection timeout.
    ///
    /// # Returns
    ///
    /// - `&mut RequestBuilder` - The builder for method chaining.
    pub fn timeout(&mut self, timeout: u64) -> &mut Self {
        if let Ok(mut config) = self.http_request.config.write() {
            config.timeout = timeout;
        }
        self
    }

    /// Enables HTTP redirection for the request.
    ///
    /// This method sets the `redirect` property of the `http_request` to `true`.
    ///
    /// # Returns
    ///
    /// - `&mut RequestBuilder` - The builder for method chaining.
    pub fn redirect(&mut self) -> &mut Self {
        if let Ok(mut config) = self.http_request.config.write() {
            config.redirect = true;
        }
        self
    }

    /// Unenables HTTP redirection for the request.
    ///
    /// This method sets the `redirect` property of the `http_request` to `false`.
    ///
    /// # Returns
    ///
    /// - `&mut RequestBuilder` - The builder for method chaining.
    pub fn unredirect(&mut self) -> &mut Self {
        if let Ok(mut config) = self.http_request.config.write() {
            config.redirect = false;
        };
        self
    }

    /// Sets the maximum number of allowed redirections for the HTTP request.
    ///
    /// This method updates the `max_redirect_times` field in the configuration and returns a mutable
    /// reference to `self` to enable method chaining.
    ///
    /// # Arguments
    ///
    /// - `usize` - The maximum number of redirections allowed. A value of `0` disables redirection.
    ///
    /// # Returns
    ///
    /// - `&mut RequestBuilder` - A mutable reference to the current instance for method chaining.
    ///
    /// # Notes
    ///
    /// Ensure that the value provided to `num` is within a valid range. Excessively high values
    /// may lead to performance issues or unintended behavior.
    pub fn max_redirect_times(&mut self, num: usize) -> &mut Self {
        if let Ok(mut config) = self.http_request.config.write() {
            config.max_redirect_times = num;
        }
        self
    }

    /// Sets the buffer size for the HTTP request configuration.
    ///
    /// This method allows you to set the size of the buffer used for reading
    /// the HTTP response. It modifies the `buffer` field of the HTTP request's
    /// configuration, which will be used when processing the response data.
    ///
    /// # Arguments
    ///
    /// - `usize` - The size of the buffer to be used, in bytes.
    ///
    /// # Returns
    ///
    /// - `&mut RequestBuilder` - Returns a mutable reference to `self`, allowing for method chaining.
    pub fn buffer(&mut self, buffer: usize) -> &mut Self {
        if let Ok(mut config) = self.http_request.config.write() {
            config.buffer = buffer;
        }
        self
    }

    /// Enables automatic response decoding.
    ///
    /// When enabled, the response body will be automatically decompressed if it is encoded
    /// using a supported compression format.
    ///
    /// # Returns
    ///
    /// - `&mut RequestBuilder` - A mutable reference to the current instance, allowing for method chaining.
    pub fn decode(&mut self) -> &mut Self {
        if let Ok(mut config) = self.http_request.config.write() {
            config.decode = true;
        }
        self
    }

    /// Disables automatic response decoding.
    ///
    /// When disabled, the response body will not be automatically decompressed,
    /// and the raw encoded data will be returned as-is.
    ///
    /// # Returns
    ///
    /// - `&mut RequestBuilder` - A mutable reference to the current instance, allowing for method chaining.
    pub fn undecode(&mut self) -> &mut Self {
        if let Ok(mut config) = self.http_request.config.write() {
            config.decode = false;
        }
        self
    }

    /// Sets an HTTP proxy for the request.
    ///
    /// This method configures the request to use an HTTP proxy server.
    ///
    /// # Arguments
    ///
    /// - `&str` - The hostname or IP address of the proxy server.
    /// - `u16` - The port number of the proxy server.
    ///
    /// # Returns
    ///
    /// - `&mut RequestBuilder` - A mutable reference to the current instance, allowing for method chaining.
    pub fn http_proxy(&mut self, host: &str, port: u16) -> &mut Self {
        if let Ok(mut config) = self.http_request.config.write() {
            config.proxy = Some(ProxyConfig {
                proxy_type: ProxyType::Http,
                host: host.to_string(),
                port,
                username: None,
                password: None,
            });
        }
        self
    }

    /// Sets an HTTPS proxy for the request.
    ///
    /// This method configures the request to use an HTTPS proxy server.
    ///
    /// # Arguments
    ///
    /// - `&str` - The hostname or IP address of the proxy server.
    /// - `u16` - The port number of the proxy server.
    ///
    /// # Returns
    ///
    /// - `&mut RequestBuilder` - A mutable reference to the current instance, allowing for method chaining.
    pub fn https_proxy(&mut self, host: &str, port: u16) -> &mut Self {
        if let Ok(mut config) = self.http_request.config.write() {
            config.proxy = Some(ProxyConfig {
                proxy_type: ProxyType::Https,
                host: host.to_string(),
                port,
                username: None,
                password: None,
            });
        }
        self
    }

    /// Sets a SOCKS5 proxy for the request.
    ///
    /// This method configures the request to use a SOCKS5 proxy server.
    ///
    /// # Arguments
    ///
    /// - `&str` - The hostname or IP address of the proxy server.
    /// - `u16` - The port number of the proxy server.
    ///
    /// # Returns
    ///
    /// - `&mut RequestBuilder` - A mutable reference to the current instance, allowing for method chaining.
    pub fn socks5_proxy(&mut self, host: &str, port: u16) -> &mut Self {
        if let Ok(mut config) = self.http_request.config.write() {
            config.proxy = Some(ProxyConfig {
                proxy_type: ProxyType::Socks5,
                host: host.to_string(),
                port,
                username: None,
                password: None,
            });
        }
        self
    }

    /// Sets an HTTP proxy with authentication for the request.
    ///
    /// This method configures the request to use an HTTP proxy server with username and password authentication.
    ///
    /// # Arguments
    ///
    /// - `&str` - The hostname or IP address of the proxy server.
    /// - `u16` - The port number of the proxy server.
    /// - `&str` - The username for proxy authentication.
    /// - `&str` - The password for proxy authentication.
    ///
    /// # Returns
    ///
    /// - `&mut RequestBuilder` - A mutable reference to the current instance, allowing for method chaining.
    pub fn http_proxy_auth(
        &mut self,
        host: &str,
        port: u16,
        username: &str,
        password: &str,
    ) -> &mut Self {
        if let Ok(mut config) = self.http_request.config.write() {
            config.proxy = Some(ProxyConfig {
                proxy_type: ProxyType::Http,
                host: host.to_string(),
                port,
                username: Some(username.to_string()),
                password: Some(password.to_string()),
            });
        }
        self
    }

    /// Sets an HTTPS proxy with authentication for the request.
    ///
    /// This method configures the request to use an HTTPS proxy server with username and password authentication.
    ///
    /// # Arguments
    ///
    /// - `&str` - The hostname or IP address of the proxy server.
    /// - `u16` - The port number of the proxy server.
    /// - `&str` - The username for proxy authentication.
    /// - `&str` - The password for proxy authentication.
    ///
    /// # Returns
    ///
    /// - `&mut RequestBuilder` - A mutable reference to the current instance, allowing for method chaining.
    pub fn https_proxy_auth(
        &mut self,
        host: &str,
        port: u16,
        username: &str,
        password: &str,
    ) -> &mut Self {
        if let Ok(mut config) = self.http_request.config.write() {
            config.proxy = Some(ProxyConfig {
                proxy_type: ProxyType::Https,
                host: host.to_string(),
                port,
                username: Some(username.to_string()),
                password: Some(password.to_string()),
            });
        }
        self
    }

    /// Sets a SOCKS5 proxy with authentication for the request.
    ///
    /// This method configures the request to use a SOCKS5 proxy server with username and password authentication.
    ///
    /// # Arguments
    ///
    /// - `&str` - The hostname or IP address of the proxy server.
    /// - `u16` - The port number of the proxy server.
    /// - `&str` - The username for proxy authentication.
    /// - `&str` - The password for proxy authentication.
    ///
    /// # Returns
    ///
    /// - `&mut RequestBuilder` - A mutable reference to the current instance, allowing for method chaining.
    pub fn socks5_proxy_auth(
        &mut self,
        host: &str,
        port: u16,
        username: &str,
        password: &str,
    ) -> &mut Self {
        if let Ok(mut config) = self.http_request.config.write() {
            config.proxy = Some(ProxyConfig {
                proxy_type: ProxyType::Socks5,
                host: host.to_string(),
                port,
                username: Some(username.to_string()),
                password: Some(password.to_string()),
            });
        }
        self
    }

    /// Finalizes the builder and returns a fully constructed async `HttpRequest` instance.
    ///
    /// This method takes the current configuration stored in `http_request`, creates a new
    /// `HttpRequest` instance with the configuration, and resets the builder's temporary
    /// state for further use.
    ///
    /// # Returns
    ///
    /// - `BoxAsyncRequestTrait` - Returns a fully constructed `BoxAsyncRequestTrait` instance based on the current builder state.
    pub fn build_async(&mut self) -> BoxAsyncRequestTrait {
        self.builder = self.http_request.clone();
        self.http_request = HttpRequest::default();
        Box::new(self.builder.clone())
    }

    /// Finalizes the builder and returns a fully constructed synchronous `HttpRequest` instance.
    ///
    /// This method takes the current configuration stored in `http_request`, creates a new
    /// `HttpRequest` instance with the configuration, and resets the builder's temporary
    /// state for further use.
    ///
    /// # Returns
    ///
    /// - `BoxRequestTrait` - Returns a fully constructed `BoxRequestTrait` instance based on the current builder state.
    pub fn build_sync(&mut self) -> BoxRequestTrait {
        self.builder = self.http_request.clone();
        self.http_request = HttpRequest::default();
        Box::new(self.builder.clone())
    }
}
