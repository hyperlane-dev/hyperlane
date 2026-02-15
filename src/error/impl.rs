use crate::*;

/// Implementation of `From` trait for converting external errors into `ServerError`.
///
/// This allows using the `?` operator to automatically convert `std::io::Error`
/// into `ServerError::TcpBind` when binding to a TCP socket.
impl From<std::io::Error> for ServerError {
    /// Creates a new `ServerError::TcpBind` instance from a `std::io::Error`.
    ///
    /// # Arguments
    ///
    /// - `std::io::Error` - The `std::io::Error` to convert.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `ServerError::TcpBind` instance.
    #[inline(always)]
    fn from(error: std::io::Error) -> Self {
        ServerError::TcpBind(error.to_string())
    }
}
