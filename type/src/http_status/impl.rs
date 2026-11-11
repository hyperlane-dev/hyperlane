use super::*;

/// The `HttpStatus` enum represents the HTTP status codes.
///
/// It maps common HTTP status codes to their respective meanings. It provides methods to retrieve
/// the corresponding numeric code as well as the associated status text. Additionally, it implements
/// conversion from a string representation of the status code.
impl HttpStatus {
    /// Gets the numeric HTTP status code.
    ///
    /// Returns the corresponding HTTP status code number for the enum variant.
    ///
    /// # Returns
    ///
    /// - `ResponseStatusCode` - The numeric status code.
    pub fn code(&self) -> ResponseStatusCode {
        match self {
            Self::Continue => 100,
            Self::SwitchingProtocols => 101,
            Self::Processing => 102,
            Self::EarlyHints => 103,
            Self::Ok => 200,
            Self::Created => 201,
            Self::Accepted => 202,
            Self::NonAuthoritativeInformation => 203,
            Self::NoContent => 204,
            Self::ResetContent => 205,
            Self::PartialContent => 206,
            Self::MultiStatus => 207,
            Self::AlreadyReported => 208,
            Self::IMUsed => 226,
            Self::MultipleChoices => 300,
            Self::MovedPermanently => 301,
            Self::Found => 302,
            Self::SeeOther => 303,
            Self::NotModified => 304,
            Self::UseProxy => 305,
            Self::TemporaryRedirect => 307,
            Self::PermanentRedirect => 308,
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::PaymentRequired => 402,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::MethodNotAllowed => 405,
            Self::NotAcceptable => 406,
            Self::ProxyAuthenticationRequired => 407,
            Self::RequestTimeout => 408,
            Self::Conflict => 409,
            Self::Gone => 410,
            Self::LengthRequired => 411,
            Self::PreconditionFailed => 412,
            Self::PayloadTooLarge => 413,
            Self::URITooLong => 414,
            Self::UnsupportedMediaType => 415,
            Self::RangeNotSatisfiable => 416,
            Self::ExpectationFailed => 417,
            Self::ImATeapot => 418,
            Self::MisdirectedRequest => 421,
            Self::UnprocessableEntity => 422,
            Self::Locked => 423,
            Self::FailedDependency => 424,
            Self::TooEarly => 425,
            Self::UpgradeRequired => 426,
            Self::PreconditionRequired => 428,
            Self::TooManyRequests => 429,
            Self::RequestHeaderFieldsTooLarge => 431,
            Self::UnavailableForLegalReasons => 451,
            Self::InternalServerError => 500,
            Self::NotImplemented => 501,
            Self::BadGateway => 502,
            Self::ServiceUnavailable => 503,
            Self::GatewayTimeout => 504,
            Self::HTTPVersionNotSupported => 505,
            Self::VariantAlsoNegotiates => 506,
            Self::InsufficientStorage => 507,
            Self::LoopDetected => 508,
            Self::NotExtended => 510,
            Self::NetworkAuthenticationRequired => 511,
            Self::Unknown => 0,
        }
    }

    /// Gets the textual description for a status code.
    ///
    /// Returns the standard HTTP status text for the given numeric code.
    ///
    /// # Arguments
    ///
    /// - `ResponseStatusCode` - The numeric HTTP status code.
    ///
    /// # Returns
    ///
    /// - `String` - The standard status text description.
    pub fn phrase(code: ResponseStatusCode) -> String {
        Self::phrase_ref(code).to_owned()
    }

    /// Gets the textual description for a status code without copying it.
    ///
    /// The phrase of a status is a constant, so a caller that only reads it
    /// borrows it rather than allocating a copy for every response.
    ///
    /// # Arguments
    ///
    /// - `ResponseStatusCode` - The numeric HTTP status code.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The standard status text description.
    pub fn phrase_ref(code: ResponseStatusCode) -> &'static str {
        match code {
            100 => CONTINUE,
            101 => SWITCHING_PROTOCOLS,
            102 => PROCESSING,
            103 => EARLY_HINTS,
            200 => OK,
            201 => CREATED,
            202 => ACCEPTED,
            203 => NON_AUTHORITATIVE_INFORMATION,
            204 => NO_CONTENT,
            205 => RESET_CONTENT,
            206 => PARTIAL_CONTENT,
            207 => MULTI_STATUS,
            208 => ALREADY_REPORTED,
            226 => IM_USED,
            300 => MULTIPLE_CHOICES,
            301 => MOVED_PERMANENTLY,
            302 => FOUND,
            303 => SEE_OTHER,
            304 => NOT_MODIFIED,
            305 => USE_PROXY,
            307 => TEMPORARY_REDIRECT,
            308 => PERMANENT_REDIRECT,
            400 => BAD_REQUEST,
            401 => UNAUTHORIZED,
            402 => PAYMENT_REQUIRED,
            403 => FORBIDDEN,
            404 => NOT_FOUND,
            405 => METHOD_NOT_ALLOWED,
            406 => NOT_ACCEPTABLE,
            407 => PROXY_AUTHENTICATION_REQUIRED,
            408 => REQUEST_TIMEOUT,
            409 => CONFLICT,
            410 => GONE,
            411 => LENGTH_REQUIRED,
            412 => PRECONDITION_FAILED,
            413 => PAYLOAD_TOO_LARGE,
            414 => URI_TOO_LONG,
            415 => UNSUPPORTED_MEDIA_TYPE,
            416 => RANGE_NOT_SATISFIABLE,
            417 => EXPECTATION_FAILED,
            418 => IM_A_TEAPOT,
            421 => MISDIRECTED_REQUEST,
            422 => UNPROCESSABLE_ENTITY,
            423 => LOCKED,
            424 => FAILED_DEPENDENCY,
            425 => TOO_EARLY,
            426 => UPGRADE_REQUIRED,
            428 => PRECONDITION_REQUIRED,
            429 => TOO_MANY_REQUESTS,
            431 => REQUEST_HEADER_FIELDS_TOO_LARGE,
            451 => UNAVAILABLE_FOR_LEGAL_REASONS,
            500 => INTERNAL_SERVER_ERROR,
            501 => NOT_IMPLEMENTED,
            502 => BAD_GATEWAY,
            503 => SERVICE_UNAVAILABLE,
            504 => GATEWAY_TIMEOUT,
            505 => HTTP_VERSION_NOT_SUPPORTED,
            506 => VARIANT_ALSO_NEGOTIATES,
            507 => INSUFFICIENT_STORAGE,
            508 => LOOP_DETECTED,
            510 => NOT_EXTENDED,
            511 => NETWORK_AUTHENTICATION_REQUIRED,
            _ => UNKNOWN,
        }
    }

    /// Checks if status matches a string representation.
    ///
    /// Compares case-insensitively against both numeric code and text description.
    ///
    /// # Arguments
    ///
    /// - `C` - The string to compare against, which requires the `AsRef<str>` bound.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the string matches either code or description.
    pub fn same<C>(&self, code_str: C) -> bool
    where
        C: AsRef<str>,
    {
        self.to_string().eq_ignore_ascii_case(code_str.as_ref())
    }
}

/// Implements the `Display` trait for `HttpStatus`, allowing it to be formatted as a string.
impl Display for HttpStatus {
    /// Formats the status code as text.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter to write to.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - The formatting result.
    fn fmt(&self, data: &mut Formatter<'_>) -> fmt::Result {
        let res: &str = match self {
            Self::Continue => CONTINUE,
            Self::SwitchingProtocols => SWITCHING_PROTOCOLS,
            Self::Processing => PROCESSING,
            Self::EarlyHints => EARLY_HINTS,
            Self::Ok => OK,
            Self::Created => CREATED,
            Self::Accepted => ACCEPTED,
            Self::NonAuthoritativeInformation => NON_AUTHORITATIVE_INFORMATION,
            Self::NoContent => NO_CONTENT,
            Self::ResetContent => RESET_CONTENT,
            Self::PartialContent => PARTIAL_CONTENT,
            Self::MultiStatus => MULTI_STATUS,
            Self::AlreadyReported => ALREADY_REPORTED,
            Self::IMUsed => IM_USED,
            Self::MultipleChoices => MULTIPLE_CHOICES,
            Self::MovedPermanently => MOVED_PERMANENTLY,
            Self::Found => FOUND,
            Self::SeeOther => SEE_OTHER,
            Self::NotModified => NOT_MODIFIED,
            Self::UseProxy => USE_PROXY,
            Self::TemporaryRedirect => TEMPORARY_REDIRECT,
            Self::PermanentRedirect => PERMANENT_REDIRECT,
            Self::BadRequest => BAD_REQUEST,
            Self::Unauthorized => UNAUTHORIZED,
            Self::PaymentRequired => PAYMENT_REQUIRED,
            Self::Forbidden => FORBIDDEN,
            Self::NotFound => NOT_FOUND,
            Self::MethodNotAllowed => METHOD_NOT_ALLOWED,
            Self::NotAcceptable => NOT_ACCEPTABLE,
            Self::ProxyAuthenticationRequired => PROXY_AUTHENTICATION_REQUIRED,
            Self::RequestTimeout => REQUEST_TIMEOUT,
            Self::Conflict => CONFLICT,
            Self::Gone => GONE,
            Self::LengthRequired => LENGTH_REQUIRED,
            Self::PreconditionFailed => PRECONDITION_FAILED,
            Self::PayloadTooLarge => PAYLOAD_TOO_LARGE,
            Self::URITooLong => URI_TOO_LONG,
            Self::UnsupportedMediaType => UNSUPPORTED_MEDIA_TYPE,
            Self::RangeNotSatisfiable => RANGE_NOT_SATISFIABLE,
            Self::ExpectationFailed => EXPECTATION_FAILED,
            Self::ImATeapot => IM_A_TEAPOT,
            Self::MisdirectedRequest => MISDIRECTED_REQUEST,
            Self::UnprocessableEntity => UNPROCESSABLE_ENTITY,
            Self::Locked => LOCKED,
            Self::FailedDependency => FAILED_DEPENDENCY,
            Self::TooEarly => TOO_EARLY,
            Self::UpgradeRequired => UPGRADE_REQUIRED,
            Self::PreconditionRequired => PRECONDITION_REQUIRED,
            Self::TooManyRequests => TOO_MANY_REQUESTS,
            Self::RequestHeaderFieldsTooLarge => REQUEST_HEADER_FIELDS_TOO_LARGE,
            Self::UnavailableForLegalReasons => UNAVAILABLE_FOR_LEGAL_REASONS,
            Self::InternalServerError => INTERNAL_SERVER_ERROR,
            Self::NotImplemented => NOT_IMPLEMENTED,
            Self::BadGateway => BAD_GATEWAY,
            Self::ServiceUnavailable => SERVICE_UNAVAILABLE,
            Self::GatewayTimeout => GATEWAY_TIMEOUT,
            Self::HTTPVersionNotSupported => HTTP_VERSION_NOT_SUPPORTED,
            Self::VariantAlsoNegotiates => VARIANT_ALSO_NEGOTIATES,
            Self::InsufficientStorage => INSUFFICIENT_STORAGE,
            Self::LoopDetected => LOOP_DETECTED,
            Self::NotExtended => NOT_EXTENDED,
            Self::NetworkAuthenticationRequired => NETWORK_AUTHENTICATION_REQUIRED,
            Self::Unknown => UNKNOWN,
        };
        write!(data, "{res}")
    }
}

/// Implements the `FromStr` trait for `HttpStatus`, allowing conversion from a string slice.
impl FromStr for HttpStatus {
    /// The error type returned when conversion fails.
    type Err = ();

    /// Parses a string into an HttpStatus.
    ///
    /// Attempts to convert the string to a numeric code and match known status codes.
    ///
    /// # Arguments
    ///
    /// - `&str` - The string to parse.
    ///
    /// # Returns
    ///
    /// - `Result<Self, Self::Err>` - The parsed status or error.
    fn from_str(code_str: &str) -> Result<Self, Self::Err> {
        if let Ok(code) = code_str.parse::<ResponseStatusCode>() {
            match code {
                100 => Ok(Self::Continue),
                101 => Ok(Self::SwitchingProtocols),
                102 => Ok(Self::Processing),
                103 => Ok(Self::EarlyHints),
                200 => Ok(Self::Ok),
                201 => Ok(Self::Created),
                202 => Ok(Self::Accepted),
                203 => Ok(Self::NonAuthoritativeInformation),
                204 => Ok(Self::NoContent),
                205 => Ok(Self::ResetContent),
                206 => Ok(Self::PartialContent),
                207 => Ok(Self::MultiStatus),
                208 => Ok(Self::AlreadyReported),
                226 => Ok(Self::IMUsed),
                300 => Ok(Self::MultipleChoices),
                301 => Ok(Self::MovedPermanently),
                302 => Ok(Self::Found),
                303 => Ok(Self::SeeOther),
                304 => Ok(Self::NotModified),
                305 => Ok(Self::UseProxy),
                307 => Ok(Self::TemporaryRedirect),
                308 => Ok(Self::PermanentRedirect),
                400 => Ok(Self::BadRequest),
                401 => Ok(Self::Unauthorized),
                402 => Ok(Self::PaymentRequired),
                403 => Ok(Self::Forbidden),
                404 => Ok(Self::NotFound),
                405 => Ok(Self::MethodNotAllowed),
                406 => Ok(Self::NotAcceptable),
                407 => Ok(Self::ProxyAuthenticationRequired),
                408 => Ok(Self::RequestTimeout),
                409 => Ok(Self::Conflict),
                410 => Ok(Self::Gone),
                411 => Ok(Self::LengthRequired),
                412 => Ok(Self::PreconditionFailed),
                413 => Ok(Self::PayloadTooLarge),
                414 => Ok(Self::URITooLong),
                415 => Ok(Self::UnsupportedMediaType),
                416 => Ok(Self::RangeNotSatisfiable),
                417 => Ok(Self::ExpectationFailed),
                418 => Ok(Self::ImATeapot),
                421 => Ok(Self::MisdirectedRequest),
                422 => Ok(Self::UnprocessableEntity),
                423 => Ok(Self::Locked),
                424 => Ok(Self::FailedDependency),
                425 => Ok(Self::TooEarly),
                426 => Ok(Self::UpgradeRequired),
                428 => Ok(Self::PreconditionRequired),
                429 => Ok(Self::TooManyRequests),
                431 => Ok(Self::RequestHeaderFieldsTooLarge),
                451 => Ok(Self::UnavailableForLegalReasons),
                500 => Ok(Self::InternalServerError),
                501 => Ok(Self::NotImplemented),
                502 => Ok(Self::BadGateway),
                503 => Ok(Self::ServiceUnavailable),
                504 => Ok(Self::GatewayTimeout),
                505 => Ok(Self::HTTPVersionNotSupported),
                506 => Ok(Self::VariantAlsoNegotiates),
                507 => Ok(Self::InsufficientStorage),
                508 => Ok(Self::LoopDetected),
                510 => Ok(Self::NotExtended),
                511 => Ok(Self::NetworkAuthenticationRequired),
                _ => Ok(Self::Unknown),
            }
        } else {
            Ok(Self::Unknown)
        }
    }
}
