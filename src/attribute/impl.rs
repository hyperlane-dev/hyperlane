use crate::*;

/// Implementation of `From` trait for `Attribute`.
impl From<&str> for Attribute {
    /// Converts a string slice into an `Attribute`.
    ///
    /// # Arguments
    ///
    /// - `&str` - The string slice to convert.
    ///
    /// # Returns
    ///
    /// - `Attribute` - The converted attribute key.
    fn from(key: &str) -> Self {
        Attribute::External(key.to_string())
    }
}

/// Implementation of `From` trait for `Attribute`.
impl From<String> for Attribute {
    /// Converts a `String` into an `Attribute`.
    ///
    /// # Arguments
    ///
    /// - `String` - The string to convert.
    ///
    /// # Returns
    ///
    /// - `Attribute` - The converted attribute key.
    fn from(key: String) -> Self {
        Attribute::External(key)
    }
}

/// Implementation of `From` trait for `Attribute`.
impl From<InternalAttribute> for Attribute {
    /// Converts an `InternalAttribute` into an `Attribute`.
    ///
    /// # Arguments
    ///
    /// - `InternalAttribute` - The internal attribute key to convert.
    ///
    /// # Returns
    ///
    /// - `Attribute` - The converted attribute key.
    fn from(key: InternalAttribute) -> Self {
        Attribute::Internal(key)
    }
}
