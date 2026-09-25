use super::*;

/// Implements the `Display` trait for `HttpVersion`, allowing it to be formatted as a string.
impl fmt::Display for HttpVersion {
    /// Formats the `HttpVersion` variant into its string representation.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter to write the string into.
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating success or failure of the formatting operation.
    fn fmt(&self, data: &mut Formatter<'_>) -> fmt::Result {
        let version_str: &str = match self {
            Self::Http0_9 => HTTP_VERSION_0_9,
            Self::Http1_0 => HTTP_VERSION_1_0,
            Self::Http1_1 => HTTP_VERSION_1_1,
            Self::Http2 => HTTP_VERSION_2,
            Self::Http3 => HTTP_VERSION_3,
            Self::Unknown(version) => version,
        };
        write!(data, "{version_str}")
    }
}

/// Implements the `FromStr` trait for `HttpVersion`, allowing conversion from a string slice.
impl FromStr for HttpVersion {
    /// The error type returned when conversion fails.
    type Err = String;

    /// Converts a string slice into an `HttpVersion` variant.
    ///
    /// This method attempts to parse the input string into a known `HttpVersion` variant.
    /// If the string does not match any known version, it returns an `Unknown` variant
    /// containing the original string.
    ///
    /// # Arguments
    ///
    /// - `&str` - The string slice to convert.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `HttpVersion` variant if successful, or `Self::Err` on failure.
    fn from_str(version_str: &str) -> Result<Self, Self::Err> {
        match version_str {
            HTTP_VERSION_0_9 => Ok(Self::Http0_9),
            HTTP_VERSION_1_0 => Ok(Self::Http1_0),
            HTTP_VERSION_1_1 => Ok(Self::Http1_1),
            HTTP_VERSION_2 => Ok(Self::Http2),
            HTTP_VERSION_3 => Ok(Self::Http3),
            _ => Ok(Self::Unknown(version_str.to_string())),
        }
    }
}

impl HttpVersion {
    /// Checks if the current version is HTTP/0.9.
    ///
    /// # Returns
    ///
    /// `true` if the version is HTTP/0.9, `false` otherwise.
    #[inline(always)]
    pub fn is_http0_9(&self) -> bool {
        matches!(self, Self::Http0_9)
    }

    /// Checks if the current version is HTTP/1.0.
    ///
    /// # Returns
    ///
    /// `true` if the version is HTTP/1.0, `false` otherwise.
    #[inline(always)]
    pub fn is_http1_0(&self) -> bool {
        matches!(self, Self::Http1_0)
    }

    /// Checks if the current version is HTTP/1.1.
    ///
    /// # Returns
    ///
    /// `true` if the version is HTTP/1.1, `false` otherwise.
    #[inline(always)]
    pub fn is_http1_1(&self) -> bool {
        matches!(self, Self::Http1_1)
    }

    /// Checks if the current version is HTTP/2.
    ///
    /// # Returns
    ///
    /// `true` if the version is HTTP/2, `false` otherwise.
    #[inline(always)]
    pub fn is_http2(&self) -> bool {
        matches!(self, Self::Http2)
    }

    /// Checks if the current version is HTTP/3.
    ///
    /// # Returns
    ///
    /// `true` if the version is HTTP/3, `false` otherwise.
    #[inline(always)]
    pub fn is_http3(&self) -> bool {
        matches!(self, Self::Http3)
    }

    /// Checks if the current version is unknown.
    ///
    /// # Returns
    ///
    /// `true` if the version is unknown, `false` otherwise.
    #[inline(always)]
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown(_))
    }

    /// Checks if the current version is HTTP/1.1 or higher.
    ///
    /// # Returns
    ///
    /// `true` if the version is HTTP/1.1, HTTP/2, or HTTP/3, `false` otherwise.
    #[inline(always)]
    pub fn is_http1_1_or_higher(&self) -> bool {
        matches!(self, Self::Http1_1 | Self::Http2 | Self::Http3)
    }

    /// Checks if the current version is a recognized HTTP version (not unknown).
    ///
    /// # Returns
    ///
    /// `true` if the version is a known HTTP version, `false` otherwise.
    #[inline(always)]
    pub fn is_http(&self) -> bool {
        !self.is_unknown()
    }
}
