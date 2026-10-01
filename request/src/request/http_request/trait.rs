use super::*;

/// Asynchronous HTTP request trait.
///
/// Defines the interface for sending asynchronous HTTP requests.
pub trait AsyncRequestTrait: Send + Debug {
    /// The result type of the asynchronous request.
    type RequestResult: Sized;

    /// Sends the HTTP request asynchronously.
    ///
    /// # Returns
    ///
    /// - `Pin<Box<dyn Future<Output = Self::RequestResult> + Send + '_>>` -
    ///   A pinned boxed future representing the asynchronous operation.
    fn send(&mut self) -> Pin<Box<dyn Future<Output = Self::RequestResult> + Send + '_>>;
}

/// Synchronous HTTP request trait.
///
/// Defines the interface for sending synchronous HTTP requests.
pub trait RequestTrait: Send + Debug {
    /// The result type of the synchronous request.
    type RequestResult: Sized;

    /// Sends the HTTP request synchronously.
    ///
    /// # Returns
    ///
    /// - `Self::RequestResult` - The result of the synchronous request.
    fn send(&mut self) -> Self::RequestResult;
}

/// Blanket AsyncRead+AsyncWrite+Unpin+Send for any compatible stream.
pub(crate) trait AsyncReadWrite: AsyncRead + AsyncWrite + Unpin + Send {}
/// Blanket Read+Write for any compatible stream.
pub(crate) trait ReadWrite: Read + Write {}
