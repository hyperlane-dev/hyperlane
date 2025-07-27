use crate::*;

/// Converts a string slice into an `AttributeKey`.
impl From<&str> for AttributeKey {
    /// This implementation creates an `External` attribute key from a string slice.
    ///
    /// # Arguments
    ///
    /// - `key` - A string slice that represents the attribute key.
    ///
    /// # Returns
    ///
    /// An `AttributeKey::External` variant containing the provided key.
    fn from(key: &str) -> Self {
        AttributeKey::External(key.to_string())
    }
}

/// Converts a `String` into an `AttributeKey`.
impl From<String> for AttributeKey {
    /// This implementation creates an `External` attribute key from a `String`.
    ///
    /// # Arguments
    ///
    /// - `key` - A `String` that represents the attribute key.
    ///
    /// # Returns
    ///
    /// An `AttributeKey::External` variant containing the provided key.
    fn from(key: String) -> Self {
        AttributeKey::External(key)
    }
}

/// Converts an `InternalAttributeKey` into an `AttributeKey`.
impl From<InternalAttributeKey> for AttributeKey {
    /// This implementation wraps an `InternalAttributeKey` in the `Internal` variant of `AttributeKey`.
    ///
    /// # Arguments
    ///
    /// - `key` - An `InternalAttributeKey` to be converted.
    ///
    /// # Returns
    ///
    /// An `AttributeKey::Internal` variant containing the provided key.
    fn from(key: InternalAttributeKey) -> Self {
        AttributeKey::Internal(key)
    }
}
