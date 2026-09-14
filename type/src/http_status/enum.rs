use super::*;

/// Standard HTTP status codes.
///
/// Includes informational, success, redirection, client and server error codes.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum HttpStatus {
    /// HTTP 100 Continue
    Continue,
    /// HTTP 101 Switching Protocols
    SwitchingProtocols,
    /// HTTP 102 Processing (WebDAV)
    Processing,
    /// HTTP 103 Early Hints
    EarlyHints,
    /// HTTP 200 OK
    #[default]
    Ok,
    /// HTTP 201 Created
    Created,
    /// HTTP 202 Accepted
    Accepted,
    /// HTTP 203 Non-Authoritative Information
    NonAuthoritativeInformation,
    /// HTTP 204 No Content
    NoContent,
    /// HTTP 205 Reset Content
    ResetContent,
    /// HTTP 206 Partial Content
    PartialContent,
    /// HTTP 207 Multi-Status (WebDAV)
    MultiStatus,
    /// HTTP 208 Already Reported (WebDAV)
    AlreadyReported,
    /// HTTP 226 IM Used
    IMUsed,
    /// HTTP 300 Multiple Choices
    MultipleChoices,
    /// HTTP 301 Moved Permanently
    MovedPermanently,
    /// HTTP 302 Found
    Found,
    /// HTTP 303 See Other
    SeeOther,
    /// HTTP 304 Not Modified
    NotModified,
    /// HTTP 305 Use Proxy
    UseProxy,
    /// HTTP 307 Temporary Redirect
    TemporaryRedirect,
    /// HTTP 308 Permanent Redirect
    PermanentRedirect,
    /// HTTP 400 Bad Request
    BadRequest,
    /// HTTP 401 Unauthorized
    Unauthorized,
    /// HTTP 402 Payment Required
    PaymentRequired,
    /// HTTP 403 Forbidden
    Forbidden,
    /// HTTP 404 Not Found
    NotFound,
    /// HTTP 405 Method Not Allowed
    MethodNotAllowed,
    /// HTTP 406 Not Acceptable
    NotAcceptable,
    /// HTTP 407 Proxy Authentication Required
    ProxyAuthenticationRequired,
    /// HTTP 408 Request Timeout
    RequestTimeout,
    /// HTTP 409 Conflict
    Conflict,
    /// HTTP 410 Gone
    Gone,
    /// HTTP 411 Length Required
    LengthRequired,
    /// HTTP 412 Precondition Failed
    PreconditionFailed,
    /// HTTP 413 Payload Too Large
    PayloadTooLarge,
    /// HTTP 414 URI Too Long
    URITooLong,
    /// HTTP 415 Unsupported Media Type
    UnsupportedMediaType,
    /// HTTP 416 Range Not Satisfiable
    RangeNotSatisfiable,
    /// HTTP 417 Expectation Failed
    ExpectationFailed,
    /// HTTP 418 I'm a teapot
    ImATeapot,
    /// HTTP 421 Misdirected Request
    MisdirectedRequest,
    /// HTTP 422 Unprocessable Entity (WebDAV)
    UnprocessableEntity,
    /// HTTP 423 Locked (WebDAV)
    Locked,
    /// HTTP 424 Failed Dependency (WebDAV)
    FailedDependency,
    /// HTTP 425 Too Early
    TooEarly,
    /// HTTP 426 Upgrade Required
    UpgradeRequired,
    /// HTTP 428 Precondition Required
    PreconditionRequired,
    /// HTTP 429 Too Many Requests
    TooManyRequests,
    /// HTTP 431 Request Header Fields Too Large
    RequestHeaderFieldsTooLarge,
    /// HTTP 451 Unavailable For Legal Reasons
    UnavailableForLegalReasons,
    /// HTTP 500 Internal Server Error
    InternalServerError,
    /// HTTP 501 Not Implemented
    NotImplemented,
    /// HTTP 502 Bad Gateway
    BadGateway,
    /// HTTP 503 Service Unavailable
    ServiceUnavailable,
    /// HTTP 504 Gateway Timeout
    GatewayTimeout,
    /// HTTP 505 HTTP Version Not Supported
    HTTPVersionNotSupported,
    /// HTTP 506 Variant Also Negotiates
    VariantAlsoNegotiates,
    /// HTTP 507 Insufficient Storage (WebDAV)
    InsufficientStorage,
    /// HTTP 508 Loop Detected (WebDAV)
    LoopDetected,
    /// HTTP 510 Not Extended
    NotExtended,
    /// HTTP 511 Network Authentication Required
    NetworkAuthenticationRequired,
    /// HTTP Unknown status code
    Unknown,
}
