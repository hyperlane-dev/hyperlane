use super::*;

/// Implementation of `From` trait for converting external errors into `ServerError`.
///
/// This allows using the `?` operator to automatically convert `IoError`
/// into `ServerError::TcpBind` when binding to a TCP socket.
impl From<io::Error> for ServerError {
    /// Creates a new `ServerError::TcpBind` instance from a `IoError`.
    ///
    /// # Arguments
    ///
    /// - `io::Error` - The `IoError` to convert.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `ServerError::TcpBind` instance.
    #[inline(always)]
    fn from(error: io::Error) -> Self {
        ServerError::TcpBind(error.to_string())
    }
}
