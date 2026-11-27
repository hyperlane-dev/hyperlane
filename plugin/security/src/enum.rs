/// Security policy violation returned by built-in guards or custom [`crate::SecurityRule`] hooks.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityViolation {
    /// Request body exceeded configured maximum byte size (`413 Payload Too Large`).
    BodyTooLarge,
    /// Request path/URI exceeded configured maximum length (`414 URI Too Long`).
    UriTooLong,
    /// `Host` header did not match the configured allowlist (`400 Bad Request`).
    InvalidHost,
    /// Request path contained traversal sequences or NUL bytes (`400 Bad Request`).
    PathTraversal,
    /// Double-submit HMAC CSRF token was absent, mismatched, or forged (`403 Forbidden`).
    CsrfInvalid,
    /// Custom violation emitted by an application-supplied [`crate::SecurityRule`].
    Custom {
        /// HTTP status code returned to the client.
        status_code: usize,
        /// Human-readable rejection reason.
        message: String,
    },
}
