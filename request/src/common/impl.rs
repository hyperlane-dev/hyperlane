use super::*;

impl Body {
    /// Empty body.
    pub const fn empty() -> Self {
        Self { bytes: Vec::new() }
    }

    /// Construct from any `Into<Vec<u8>>`.
    ///
    /// # Arguments
    ///
    /// - `B` - The raw bytes to store, converted into the owned `Vec<u8>`.
    pub fn from_bytes<B: Into<Vec<u8>>>(bytes: B) -> Self {
        Self {
            bytes: bytes.into(),
        }
    }

    /// View body as `&[u8]`.
    ///
    /// # Returns
    ///
    /// - `&[u8]` - The raw body bytes.
    pub fn get_bytes_ref(&self) -> &[u8] {
        &self.bytes
    }

    /// View body as `&[u8]`.
    ///
    /// # Returns
    ///
    /// - `&[u8]` - The raw body bytes.
    pub fn as_slice(&self) -> &[u8] {
        self.get_bytes()
    }

    /// Try to view body as UTF-8 string.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - The body decoded as UTF-8, or `None` when the bytes
    ///   are not valid UTF-8.
    pub fn as_str(&self) -> Option<&str> {
        std::str::from_utf8(self.get_bytes()).ok()
    }
}

impl Display for Body {
    /// Formats the `Body` as its UTF-8 text, falling back to the debug
    /// representation of the raw bytes when the body is not valid UTF-8.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - A mutable reference to a `Formatter` used for
    ///   writing the formatted string.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - A `fmt::Result` indicating whether the formatting was
    ///   successful.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.as_str() {
            Some(s) => f.write_str(s),
            None => f.write_str(&format!("{:?}", self.bytes)),
        }
    }
}
