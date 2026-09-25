use super::*;

/// Result type for HTTP requests, containing either a response or error.
pub type RequestResult = Result<BoxResponseTrait, RequestError>;

/// Boxed trait object for asynchronous HTTP requests.
pub type BoxAsyncRequestTrait = Box<dyn AsyncRequestTrait<RequestResult = RequestResult>>;

/// Boxed trait object for synchronous HTTP requests.
pub type BoxRequestTrait = Box<dyn RequestTrait<RequestResult = RequestResult>>;

/// Boxed trait object for asynchronous read/write streams.
pub(crate) type BoxAsyncReadWrite = Box<dyn AsyncReadWrite>;

/// Boxed trait object for synchronous read/write streams.
pub(crate) type BoxReadWrite = Box<dyn ReadWrite>;
