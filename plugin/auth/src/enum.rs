/// Authentication mechanism that verified an [`crate::AuthPrincipal`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AuthScheme {
    /// JSON Web Token (`Authorization: Bearer <jwt>`).
    #[default]
    BearerJwt,
    /// Static or rotated API key header (for example `X-Api-Key`).
    ApiKey,
    /// HTTP Basic credentials (`Authorization: Basic <base64>`).
    Basic,
    /// Application-defined custom authenticator scheme.
    Custom,
}

/// Errors produced during token decoding, credential verification, or RBAC authorization.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthError {
    /// No credentials were provided on the request.
    MissingCredentials,
    /// Credentials or token failed parsing or signature verification.
    InvalidCredentials,
    /// JWT `exp` timestamp has passed beyond the configured leeway.
    TokenExpired,
    /// JWT `nbf` timestamp is still in the future beyond the configured leeway.
    TokenNotYetValid,
    /// JWT `alg` header is not supported (for example `none` or RS256 on HS256 codec).
    UnsupportedAlgorithm,
    /// JWT `iss` or `aud` claim did not satisfy policy requirements.
    ClaimMismatch,
    /// Authenticated principal lacks one or more required roles.
    Forbidden,
}
