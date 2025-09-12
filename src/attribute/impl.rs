use crate::*;

/// Implementation of `From` trait for `AttributeKey`.
impl From<&str> for AttributeKey {
    /// Converts a string slice into an `AttributeKey`.
    ///
    /// # Arguments
    ///
    /// - `&str` - The string slice to convert.
    ///
    /// # Returns
    ///
    /// - `AttributeKey` - The converted attribute key.
    fn from(key: &str) -> Self {
        AttributeKey::External(key.to_string())
    }
}

/// Implementation of `From` trait for `AttributeKey`.
impl From<String> for AttributeKey {
    /// Converts a `String` into an `AttributeKey`.
    ///
    /// # Arguments
    ///
    /// - `String` - The string to convert.
    ///
    /// # Returns
    ///
    /// - `AttributeKey` - The converted attribute key.
    fn from(key: String) -> Self {
        AttributeKey::External(key)
    }
}

/// Implementation of `From` trait for `AttributeKey`.
impl From<InternalAttribute> for AttributeKey {
    /// Converts an `InternalAttribute` into an `AttributeKey`.
    ///
    /// # Arguments
    ///
    /// - `InternalAttribute` - The internal attribute key to convert.
    ///
    /// # Returns
    ///
    /// - `AttributeKey` - The converted attribute key.
    fn from(key: InternalAttribute) -> Self {
        AttributeKey::Internal(key)
    }
}
