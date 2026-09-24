use super::*;

/// Provides default configuration values for HTTP requests.
///
/// # Returns
///
/// - `Config` - A configuration instance with default values
impl Default for Config {
    #[inline(always)]
    fn default() -> Self {
        Self {
            timeout: DEFAULT_HIGH_SECURITY_READ_TIMEOUT_MS,
            url_obj: HttpUrlComponents::default(),
            redirect: false,
            max_redirect_times: DEFAULT_MAX_REDIRECT_TIMES,
            redirect_times: 0,
            http_version: HttpVersion::default(),
            buffer: DEFAULT_BUFFER_SIZE,
            decode: true,
            proxy: None,
        }
    }
}
