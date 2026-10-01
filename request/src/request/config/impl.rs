use super::*;

impl RequestConfig {
    /// Sets the `buffer_size` field.
    ///
    /// # Arguments
    ///
    /// - `usize` - The new buffer size in bytes.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to the updated configuration.
    pub fn set_buffer_size(&mut self, v: usize) -> &mut Self {
        self.buffer_size = v;
        self
    }

    /// Gets the `buffer_size` field.
    ///
    /// # Returns
    ///
    /// - `usize` - The buffer size in bytes.
    pub fn get_buffer_size(&self) -> usize {
        self.buffer_size
    }

    /// Sets the `timeout` field.
    ///
    /// # Arguments
    ///
    /// - `u64` - The new request timeout in seconds.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to the updated configuration.
    pub fn set_timeout(&mut self, v: u64) -> &mut Self {
        self.timeout = v;
        self
    }

    /// Gets the `timeout` field.
    ///
    /// # Returns
    ///
    /// - `u64` - The request timeout in seconds.
    pub fn get_timeout(&self) -> u64 {
        self.timeout
    }

    /// Sets the `max_redirect_times` field.
    ///
    /// # Arguments
    ///
    /// - `usize` - The maximum number of redirects to follow.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to the updated configuration.
    pub fn set_max_redirect_times(&mut self, v: usize) -> &mut Self {
        self.max_redirect_times = v;
        self
    }

    /// Gets the `max_redirect_times` field.
    ///
    /// # Returns
    ///
    /// - `usize` - The maximum number of redirects to follow.
    pub fn get_max_redirect_times(&self) -> usize {
        self.max_redirect_times
    }

    /// Sets the `http_version` field.
    ///
    /// # Arguments
    ///
    /// - `HttpVersion` - The HTTP version to negotiate.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to the updated configuration.
    pub fn set_http_version(&mut self, v: HttpVersion) -> &mut Self {
        self.http_version = v;
        self
    }

    /// Gets the `http_version` field.
    ///
    /// # Returns
    ///
    /// - `&HttpVersion` - A reference to the negotiated HTTP version.
    pub fn get_http_version(&self) -> &HttpVersion {
        &self.http_version
    }

    /// Sets the `redirect` field.
    ///
    /// # Arguments
    ///
    /// - `bool` - Whether redirects should be followed.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to the updated configuration.
    pub fn set_redirect(&mut self, v: bool) -> &mut Self {
        self.redirect = v;
        self
    }

    /// Gets the `redirect` field.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether redirects are followed.
    pub fn get_redirect(&self) -> bool {
        self.redirect
    }

    /// Sets the `decode` field.
    ///
    /// # Arguments
    ///
    /// - `bool` - Whether the response body should be decoded.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to the updated configuration.
    pub fn set_decode(&mut self, v: bool) -> &mut Self {
        self.decode = v;
        self
    }

    /// Gets the `decode` field.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the response body is decoded.
    pub fn get_decode(&self) -> bool {
        self.decode
    }

    /// Sets the `proxy` field.
    ///
    /// # Arguments
    ///
    /// - `Option<Proxy>` - The proxy to route requests through, or `None` for a direct connection.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to the updated configuration.
    pub fn set_proxy(&mut self, v: Option<Proxy>) -> &mut Self {
        self.proxy = v;
        self
    }

    /// Gets the `proxy` field.
    ///
    /// # Returns
    ///
    /// - `&Option<Proxy>` - A reference to the configured proxy, or `None` for a direct connection.
    pub fn get_proxy(&self) -> &Option<Proxy> {
        &self.proxy
    }
}
