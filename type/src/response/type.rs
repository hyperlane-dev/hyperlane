use super::*;

/// An alias for `Vec<u8>`, representing the binary body of the HTTP response.
pub type ResponseBody = Vec<u8>;

/// An alias for `String`, representing the body of the HTTP response as a UTF-8 string.
pub type ResponseBodyString = String;

/// An alias for `String`, representing the key type used in HTTP response headers.
pub type ResponseHeadersKey = String;

/// An alias for `String`, representing a single value string for an HTTP response header.
pub type ResponseHeadersValueItem = String;

/// An alias for `VecDeque<ResponseHeadersValueItem>`, representing a collection of values for a single HTTP response header.
pub type ResponseHeadersValue = VecDeque<ResponseHeadersValueItem>;

/// An alias for `HashMapXxHash3_64<ResponseHeadersKey, ResponseHeadersValue>`, representing a map of HTTP response headers.
pub type ResponseHeaders = HashMapXxHash3_64<ResponseHeadersKey, ResponseHeadersValue>;

/// An alias for `HttpVersion`, representing the HTTP version of the response.
pub type ResponseVersion = HttpVersion;

/// An alias for `usize`, representing the numeric status code of the HTTP response.
pub type ResponseStatusCode = usize;

/// An alias for `String`, representing the reason phrase associated with the HTTP status code.
pub type ResponseReasonPhrase = String;

/// An alias for `Vec<u8>`, representing the full serialized binary content of the HTTP response.
pub type ResponseData = Vec<u8>;

/// An alias for `String`, representing the full serialized content of the HTTP response as a UTF-8 string.
pub type ResponseDataString = String;

/// An alias for `RwLockReadGuard<'a, Response>`, representing a read guard to a shared `Response` value protected by `RwLock`.
pub type RwLockReadGuardResponse<'a> = RwLockReadGuard<'a, Response>;

/// An alias for `RwLockWriteGuard<'a, Response>`, representing a write guard to a shared `Response` value protected by `RwLock`.
pub type RwLockWriteGuardResponse<'a> = RwLockWriteGuard<'a, Response>;
