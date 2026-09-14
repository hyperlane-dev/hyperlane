use super::*;

/// An alias for `Method`, representing the HTTP request method.
pub type RequestMethod = Method;

/// An alias for `String`, representing the host part of an HTTP request.
pub type RequestHost = String;

/// An alias for `HttpVersion`, representing the HTTP version.
pub type RequestVersion = HttpVersion;

/// An alias for `String`, representing the path portion of the request URL.
pub type RequestPath = String;

/// An alias for `String`, representing the key type for request query parameters.
pub type RequestQuerysKey = String;

/// An alias for `String`, representing the value type for request query parameters.
pub type RequestQuerysValue = String;

/// An alias for `HashMapXxHash3_64<RequestQuerysKey, RequestQuerysValue>`, representing all query parameters parsed from the request URL.
pub type RequestQuerys = HashMapXxHash3_64<RequestQuerysKey, RequestQuerysValue>;

/// An alias for `Vec<u8>`, representing the raw binary body of the request.
pub type RequestBody = Vec<u8>;

/// An alias for `String`, representing the request body as a UTF-8 encoded string.
pub type RequestBodyString = String;

/// An alias for `String`, representing the key type for request headers.
pub type RequestHeadersKey = String;

/// An alias for `String`, representing a single value for an HTTP request header.
pub type RequestHeadersValueItem = String;

/// An alias for `VecDeque<RequestHeadersValueItem>`, representing a collection of values for a single HTTP request header.
pub type RequestHeadersValue = VecDeque<RequestHeadersValueItem>;

/// An alias for `HashMapXxHash3_64<RequestHeadersKey, RequestHeadersValue>`, representing all headers sent with the HTTP request.
pub type RequestHeaders = HashMapXxHash3_64<RequestHeadersKey, RequestHeadersValue>;

/// An alias for `RwLockReadGuard<'a, Request>`, representing a read guard for a `Request` wrapped in a `RwLock`.
pub type RwLockReadGuardRequest<'a> = RwLockReadGuard<'a, Request>;

/// An alias for `RwLockWriteGuard<'a, Request>`, representing a write guard for a `Request` wrapped in a `RwLock`.
pub type RwLockWriteGuardRequest<'a> = RwLockWriteGuard<'a, Request>;
