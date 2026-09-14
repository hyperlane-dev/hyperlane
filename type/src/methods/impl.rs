use super::*;

/// Implements the `Default` trait for `Method`.
impl Default for Method {
    /// Returns the default `Method` variant, which is `UNKNOWN` with an empty string.
    ///
    /// # Returns
    ///
    /// The default `Method` variant.
    #[inline(always)]
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}

/// Implements the `Display` trait for `Method`, allowing it to be formatted as a string.
impl Display for Method {
    /// Formats the `Method` variant into its string representation.
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
        let res: &str = match self {
            Self::Get => GET,
            Self::Post => POST,
            Self::Connect => CONNECT,
            Self::Delete => DELETE,
            Self::Head => HEAD,
            Self::Patch => PATCH,
            Self::Trace => TRACE,
            Self::Put => PUT,
            Self::Options => OPTIONS,
            Self::Unknown(methods) => methods,
        };
        write!(data, "{res}")
    }
}

/// Implements the `FromStr` trait for `Method`, allowing conversion from a string slice.
impl FromStr for Method {
    /// The error type returned when conversion fails.
    type Err = ();

    /// Converts a string slice into a `Method` variant.
    ///
    /// This method attempts to parse the input string into a known `Method` variant.
    /// If the string does not match any known method, it returns an `UNKNOWN` variant
    /// containing the original string.
    ///
    /// # Arguments
    ///
    /// - `&str` - The string slice to convert.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Method` variant if successful, or `Self::Err` on failure.
    #[inline(always)]
    fn from_str(methods_str: &str) -> Result<Self, Self::Err> {
        match methods_str {
            GET => Ok(Self::Get),
            POST => Ok(Self::Post),
            PUT => Ok(Self::Put),
            DELETE => Ok(Self::Delete),
            PATCH => Ok(Self::Patch),
            HEAD => Ok(Self::Head),
            OPTIONS => Ok(Self::Options),
            CONNECT => Ok(Self::Connect),
            TRACE => Ok(Self::Trace),
            _ => Ok(Self::Unknown(methods_str.to_string())),
        }
    }
}

impl Method {
    /// Checks if the current method is `GET`.
    ///
    /// # Returns
    ///
    /// `true` if the method is `GET`, `false` otherwise.
    #[inline(always)]
    pub fn is_get(&self) -> bool {
        matches!(self, Self::Get)
    }

    /// Checks if the current method is `POST`.
    ///
    /// # Returns
    ///
    /// `true` if the method is `POST`, `false` otherwise.
    #[inline(always)]
    pub fn is_post(&self) -> bool {
        matches!(self, Self::Post)
    }

    /// Checks if the current method is `PUT`.
    ///
    /// # Returns
    ///
    /// `true` if the method is `PUT`, `false` otherwise.
    #[inline(always)]
    pub fn is_put(&self) -> bool {
        matches!(self, Self::Put)
    }

    /// Checks if the current method is `DELETE`.
    ///
    /// # Returns
    ///
    /// `true` if the method is `DELETE`, `false` otherwise.
    #[inline(always)]
    pub fn is_delete(&self) -> bool {
        matches!(self, Self::Delete)
    }

    /// Checks if the current method is `PATCH`.
    ///
    /// # Returns
    ///
    /// `true` if the method is `PATCH`, `false` otherwise.
    #[inline(always)]
    pub fn is_patch(&self) -> bool {
        matches!(self, Self::Patch)
    }

    /// Checks if the current method is `HEAD`.
    ///
    /// # Returns
    ///
    /// `true` if the method is `HEAD`, `false` otherwise.
    #[inline(always)]
    pub fn is_head(&self) -> bool {
        matches!(self, Self::Head)
    }

    /// Checks if the current method is `OPTIONS`.
    ///
    /// # Returns
    ///
    /// `true` if the method is `OPTIONS`, `false` otherwise.
    #[inline(always)]
    pub fn is_options(&self) -> bool {
        matches!(self, Self::Options)
    }

    /// Checks if the current method is `CONNECT`.
    ///
    /// # Returns
    ///
    /// `true` if the method is `CONNECT`, `false` otherwise.
    #[inline(always)]
    pub fn is_connect(&self) -> bool {
        matches!(self, Self::Connect)
    }

    /// Checks if the current method is `TRACE`.
    ///
    /// # Returns
    ///
    /// `true` if the method is `TRACE`, `false` otherwise.
    #[inline(always)]
    pub fn is_trace(&self) -> bool {
        matches!(self, Self::Trace)
    }

    /// Checks if the current method is `UNKNOWN`.
    ///
    /// # Returns
    ///
    /// `true` if the method is `UNKNOWN`, `false` otherwise.
    #[inline(always)]
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown(_))
    }
}
