use crate::*;

/// Implementation of `From` trait for converting external errors into `ServerError`.
///
/// This allows using the `?` operator to automatically convert `IoError`
/// into `ServerError::TcpBind` when binding to a TCP socket.
impl From<IoError> for ServerError {
    /// Creates a new `ServerError::TcpBind` instance from an `IoError`.
    ///
    /// # Arguments
    ///
    /// - `IoError` - The I/O error to convert.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `ServerError::TcpBind` instance.
    #[inline(always)]
    fn from(error: IoError) -> Self {
        ServerError::TcpBind(error.to_string())
    }
}
