use super::*;

#[test]
fn test_http_status_code() {
    assert_eq!(HttpStatus::Continue.code(), 100);
    assert_eq!(HttpStatus::SwitchingProtocols.code(), 101);
    assert_eq!(HttpStatus::Processing.code(), 102);
    assert_eq!(HttpStatus::EarlyHints.code(), 103);
    assert_eq!(HttpStatus::Ok.code(), 200);
    assert_eq!(HttpStatus::Created.code(), 201);
    assert_eq!(HttpStatus::Accepted.code(), 202);
    assert_eq!(HttpStatus::NonAuthoritativeInformation.code(), 203);
    assert_eq!(HttpStatus::NoContent.code(), 204);
    assert_eq!(HttpStatus::ResetContent.code(), 205);
    assert_eq!(HttpStatus::PartialContent.code(), 206);
    assert_eq!(HttpStatus::MultiStatus.code(), 207);
    assert_eq!(HttpStatus::AlreadyReported.code(), 208);
    assert_eq!(HttpStatus::IMUsed.code(), 226);
    assert_eq!(HttpStatus::MultipleChoices.code(), 300);
    assert_eq!(HttpStatus::MovedPermanently.code(), 301);
    assert_eq!(HttpStatus::Found.code(), 302);
    assert_eq!(HttpStatus::SeeOther.code(), 303);
    assert_eq!(HttpStatus::NotModified.code(), 304);
    assert_eq!(HttpStatus::UseProxy.code(), 305);
    assert_eq!(HttpStatus::TemporaryRedirect.code(), 307);
    assert_eq!(HttpStatus::PermanentRedirect.code(), 308);
    assert_eq!(HttpStatus::BadRequest.code(), 400);
    assert_eq!(HttpStatus::Unauthorized.code(), 401);
    assert_eq!(HttpStatus::PaymentRequired.code(), 402);
    assert_eq!(HttpStatus::Forbidden.code(), 403);
    assert_eq!(HttpStatus::NotFound.code(), 404);
    assert_eq!(HttpStatus::MethodNotAllowed.code(), 405);
    assert_eq!(HttpStatus::NotAcceptable.code(), 406);
    assert_eq!(HttpStatus::ProxyAuthenticationRequired.code(), 407);
    assert_eq!(HttpStatus::RequestTimeout.code(), 408);
    assert_eq!(HttpStatus::Conflict.code(), 409);
    assert_eq!(HttpStatus::Gone.code(), 410);
    assert_eq!(HttpStatus::LengthRequired.code(), 411);
    assert_eq!(HttpStatus::PreconditionFailed.code(), 412);
    assert_eq!(HttpStatus::PayloadTooLarge.code(), 413);
    assert_eq!(HttpStatus::URITooLong.code(), 414);
    assert_eq!(HttpStatus::UnsupportedMediaType.code(), 415);
    assert_eq!(HttpStatus::RangeNotSatisfiable.code(), 416);
    assert_eq!(HttpStatus::ExpectationFailed.code(), 417);
    assert_eq!(HttpStatus::ImATeapot.code(), 418);
    assert_eq!(HttpStatus::MisdirectedRequest.code(), 421);
    assert_eq!(HttpStatus::UnprocessableEntity.code(), 422);
    assert_eq!(HttpStatus::Locked.code(), 423);
    assert_eq!(HttpStatus::FailedDependency.code(), 424);
    assert_eq!(HttpStatus::TooEarly.code(), 425);
    assert_eq!(HttpStatus::UpgradeRequired.code(), 426);
    assert_eq!(HttpStatus::PreconditionRequired.code(), 428);
    assert_eq!(HttpStatus::TooManyRequests.code(), 429);
    assert_eq!(HttpStatus::RequestHeaderFieldsTooLarge.code(), 431);
    assert_eq!(HttpStatus::UnavailableForLegalReasons.code(), 451);
    assert_eq!(HttpStatus::InternalServerError.code(), 500);
    assert_eq!(HttpStatus::NotImplemented.code(), 501);
    assert_eq!(HttpStatus::BadGateway.code(), 502);
    assert_eq!(HttpStatus::ServiceUnavailable.code(), 503);
    assert_eq!(HttpStatus::GatewayTimeout.code(), 504);
    assert_eq!(HttpStatus::HTTPVersionNotSupported.code(), 505);
    assert_eq!(HttpStatus::VariantAlsoNegotiates.code(), 506);
    assert_eq!(HttpStatus::InsufficientStorage.code(), 507);
    assert_eq!(HttpStatus::LoopDetected.code(), 508);
    assert_eq!(HttpStatus::NotExtended.code(), 510);
    assert_eq!(HttpStatus::NetworkAuthenticationRequired.code(), 511);
    assert_eq!(HttpStatus::Unknown.code(), 0);
}

#[test]
fn test_http_status_phrase() {
    assert_eq!(HttpStatus::phrase(100), "Continue");
    assert_eq!(HttpStatus::phrase(101), "Switching Protocols");
    assert_eq!(HttpStatus::phrase(102), "Processing");
    assert_eq!(HttpStatus::phrase(103), "Early Hints");
    assert_eq!(HttpStatus::phrase(200), "OK");
    assert_eq!(HttpStatus::phrase(201), "Created");
    assert_eq!(HttpStatus::phrase(202), "Accepted");
    assert_eq!(HttpStatus::phrase(203), "Non-Authoritative Information");
    assert_eq!(HttpStatus::phrase(204), "No Content");
    assert_eq!(HttpStatus::phrase(205), "Reset Content");
    assert_eq!(HttpStatus::phrase(206), "Partial Content");
    assert_eq!(HttpStatus::phrase(207), "Multi-Status");
    assert_eq!(HttpStatus::phrase(208), "Already Reported");
    assert_eq!(HttpStatus::phrase(226), "IM Used");
    assert_eq!(HttpStatus::phrase(300), "Multiple Choices");
    assert_eq!(HttpStatus::phrase(301), "Moved Permanently");
    assert_eq!(HttpStatus::phrase(302), "Found");
    assert_eq!(HttpStatus::phrase(303), "See Other");
    assert_eq!(HttpStatus::phrase(304), "Not Modified");
    assert_eq!(HttpStatus::phrase(305), "Use Proxy");
    assert_eq!(HttpStatus::phrase(307), "Temporary Redirect");
    assert_eq!(HttpStatus::phrase(308), "Permanent Redirect");
    assert_eq!(HttpStatus::phrase(400), "Bad Request");
    assert_eq!(HttpStatus::phrase(401), "Unauthorized");
    assert_eq!(HttpStatus::phrase(402), "Payment Required");
    assert_eq!(HttpStatus::phrase(403), "Forbidden");
    assert_eq!(HttpStatus::phrase(404), "Not Found");
    assert_eq!(HttpStatus::phrase(405), "Method Not Allowed");
    assert_eq!(HttpStatus::phrase(406), "Not Acceptable");
    assert_eq!(HttpStatus::phrase(407), "Proxy Authentication Required");
    assert_eq!(HttpStatus::phrase(408), "Request Timeout");
    assert_eq!(HttpStatus::phrase(409), "Conflict");
    assert_eq!(HttpStatus::phrase(410), "Gone");
    assert_eq!(HttpStatus::phrase(411), "Length Required");
    assert_eq!(HttpStatus::phrase(412), "Precondition Failed");
    assert_eq!(HttpStatus::phrase(413), "Payload Too Large");
    assert_eq!(HttpStatus::phrase(414), "URI Too Long");
    assert_eq!(HttpStatus::phrase(415), "Unsupported Media Type");
    assert_eq!(HttpStatus::phrase(416), "Range Not Satisfiable");
    assert_eq!(HttpStatus::phrase(417), "Expectation Failed");
    assert_eq!(HttpStatus::phrase(418), "I'm a teapot");
    assert_eq!(HttpStatus::phrase(421), "Misdirected Request");
    assert_eq!(HttpStatus::phrase(422), "Unprocessable Entity");
    assert_eq!(HttpStatus::phrase(423), "Locked");
    assert_eq!(HttpStatus::phrase(424), "Failed Dependency");
    assert_eq!(HttpStatus::phrase(425), "Too Early");
    assert_eq!(HttpStatus::phrase(426), "Upgrade Required");
    assert_eq!(HttpStatus::phrase(428), "Precondition Required");
    assert_eq!(HttpStatus::phrase(429), "Too Many Requests");
    assert_eq!(HttpStatus::phrase(431), "Request Header Fields Too Large");
    assert_eq!(HttpStatus::phrase(451), "Unavailable For Legal Reasons");
    assert_eq!(HttpStatus::phrase(500), "Internal Server Error");
    assert_eq!(HttpStatus::phrase(501), "Not Implemented");
    assert_eq!(HttpStatus::phrase(502), "Bad Gateway");
    assert_eq!(HttpStatus::phrase(503), "Service Unavailable");
    assert_eq!(HttpStatus::phrase(504), "Gateway Timeout");
    assert_eq!(HttpStatus::phrase(505), "HTTP Version Not Supported");
    assert_eq!(HttpStatus::phrase(506), "Variant Also Negotiates");
    assert_eq!(HttpStatus::phrase(507), "Insufficient Storage");
    assert_eq!(HttpStatus::phrase(508), "Loop Detected");
    assert_eq!(HttpStatus::phrase(510), "Not Extended");
    assert_eq!(HttpStatus::phrase(511), "Network Authentication Required");
    assert_eq!(HttpStatus::phrase(999), "Unknown");
}

#[test]
fn test_http_status_display() {
    assert_eq!(HttpStatus::Ok.to_string(), "OK");
    assert_eq!(HttpStatus::NotFound.to_string(), "Not Found");
    assert_eq!(
        HttpStatus::InternalServerError.to_string(),
        "Internal Server Error"
    );
    assert_eq!(HttpStatus::Continue.to_string(), "Continue");
    assert_eq!(HttpStatus::BadRequest.to_string(), "Bad Request");
    assert_eq!(HttpStatus::Unknown.to_string(), "Unknown");
}

#[test]
fn test_http_status_same() {
    assert!(HttpStatus::Ok.same("OK"));
    assert!(HttpStatus::Ok.same("ok"));
    assert!(HttpStatus::Ok.same("Ok"));
    assert!(HttpStatus::NotFound.same("Not Found"));
    assert!(HttpStatus::NotFound.same("not found"));
    assert!(HttpStatus::NotFound.same("NOT FOUND"));
    assert!(!HttpStatus::Ok.same("Bad Request"));
    assert!(!HttpStatus::NotFound.same("OK"));
}

#[test]
fn test_http_status_from_str() {
    assert_eq!("100".parse::<HttpStatus>().unwrap(), HttpStatus::Continue);
    assert_eq!("200".parse::<HttpStatus>().unwrap(), HttpStatus::Ok);
    assert_eq!("404".parse::<HttpStatus>().unwrap(), HttpStatus::NotFound);
    assert_eq!(
        "500".parse::<HttpStatus>().unwrap(),
        HttpStatus::InternalServerError
    );
    assert_eq!("400".parse::<HttpStatus>().unwrap(), HttpStatus::BadRequest);
    assert_eq!("666".parse::<HttpStatus>().unwrap(), HttpStatus::Unknown);
    assert_eq!("".parse::<HttpStatus>().unwrap(), HttpStatus::Unknown);
}

#[test]
fn test_http_status_default() {
    assert_eq!(HttpStatus::default(), HttpStatus::Ok);
}

#[test]
fn test_http_status_clone() {
    let status: HttpStatus = HttpStatus::Ok;
    let cloned_status: HttpStatus = status;
    assert_eq!(status, cloned_status);
}

#[test]
fn test_http_status_debug() {
    let status: HttpStatus = HttpStatus::Ok;
    let debug_str: String = format!("{status:?}");
    assert_eq!(debug_str, "Ok");
}

#[test]
fn test_http_status_equality() {
    assert_eq!(HttpStatus::Ok, HttpStatus::Ok);
    assert_ne!(HttpStatus::Ok, HttpStatus::NotFound);
    assert_eq!(
        HttpStatus::InternalServerError,
        HttpStatus::InternalServerError
    );
    assert_ne!(HttpStatus::BadRequest, HttpStatus::Unauthorized);
}

#[test]
fn test_http_status_informational() {
    assert_eq!(HttpStatus::Continue.code(), 100);
    assert_eq!(HttpStatus::SwitchingProtocols.code(), 101);
    assert_eq!(HttpStatus::Processing.code(), 102);
    assert_eq!(HttpStatus::EarlyHints.code(), 103);
}

#[test]
fn test_http_status_success() {
    assert_eq!(HttpStatus::Ok.code(), 200);
    assert_eq!(HttpStatus::Created.code(), 201);
    assert_eq!(HttpStatus::Accepted.code(), 202);
    assert_eq!(HttpStatus::NoContent.code(), 204);
}

#[test]
fn test_http_status_redirection() {
    assert_eq!(HttpStatus::MultipleChoices.code(), 300);
    assert_eq!(HttpStatus::MovedPermanently.code(), 301);
    assert_eq!(HttpStatus::Found.code(), 302);
    assert_eq!(HttpStatus::NotModified.code(), 304);
}

#[test]
fn test_http_status_client_error() {
    assert_eq!(HttpStatus::BadRequest.code(), 400);
    assert_eq!(HttpStatus::Unauthorized.code(), 401);
    assert_eq!(HttpStatus::Forbidden.code(), 403);
    assert_eq!(HttpStatus::NotFound.code(), 404);
    assert_eq!(HttpStatus::MethodNotAllowed.code(), 405);
}

#[test]
fn test_http_status_server_error() {
    assert_eq!(HttpStatus::InternalServerError.code(), 500);
    assert_eq!(HttpStatus::NotImplemented.code(), 501);
    assert_eq!(HttpStatus::BadGateway.code(), 502);
    assert_eq!(HttpStatus::ServiceUnavailable.code(), 503);
    assert_eq!(HttpStatus::GatewayTimeout.code(), 504);
}

#[test]
fn test_http_status_special_codes() {
    assert_eq!(HttpStatus::ImATeapot.code(), 418);
    assert_eq!(HttpStatus::ImATeapot.to_string(), "I'm a teapot");
    assert_eq!(HttpStatus::Unknown.code(), 0);
    assert_eq!(HttpStatus::Unknown.to_string(), "Unknown");
}
