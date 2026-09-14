use super::*;

/// Implements the `std::error::Error` trait for `HttpUrlError`.
impl std::error::Error for HttpUrlError {}

/// Implements the `Display` trait for `HttpUrlError`, allowing it to be formatted as a string.
impl Display for HttpUrlError {
    /// Formats the `HttpUrlError` variant into a human-readable string.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter to write the string into.
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating success or failure of the formatting operation.
    #[inline(always)]
    fn fmt(&self, data: &mut Formatter<'_>) -> fmt::Result {
        match self {
            HttpUrlError::InvalidUrl => write!(data, "Invalid URL"),
            HttpUrlError::Unknown => write!(data, "Unknown error"),
        }
    }
}

/// Converts a URL parse error to a `HttpUrlError`.
///
/// Maps URL parse errors to `InvalidUrl`.
impl From<ParseError> for HttpUrlError {
    /// Converts a URL parse error to a `HttpUrlError`.
    ///
    /// # Arguments
    ///
    /// - `ParseError`: The URL parse error to convert.
    ///
    /// # Returns
    ///
    /// - `HttpUrlError`: The corresponding error as `InvalidUrl`.
    #[inline(always)]
    fn from(_: ParseError) -> Self {
        HttpUrlError::InvalidUrl
    }
}

impl HttpUrlComponents {
    /// Parses a URL string into its components.
    ///
    /// Extracts protocol, host, port, path, query and fragment from the URL string.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The URL string to parse.
    ///
    /// # Returns
    ///
    /// - `Result<HttpUrlComponents, HttpUrlError>` - Either the parsed components or an error.
    #[inline]
    pub fn parse<U>(url: U) -> Result<Self, HttpUrlError>
    where
        U: AsRef<str>,
    {
        let parsed_url: Url = Url::parse(url.as_ref())?;
        let protocol: String = parsed_url.scheme().to_string();
        let res: Self = Self {
            protocol,
            host: parsed_url.host_str().map(|data: &str| data.to_string()),
            port: parsed_url.port(),
            path: Some(parsed_url.path().to_string()),
            query: parsed_url.query().map(|data: &str| data.to_string()),
            fragment: parsed_url.fragment().map(|data: &str| data.to_string()),
        };
        Ok(res)
    }
}
