/// Defines operations that can be performed on response headers.
pub(crate) enum HeaderOperation {
    /// Sets an existing header value, replacing it if it already exists.
    Set,
    /// Adds a new header value, keeping any existing values with the same key.
    Add,
}
