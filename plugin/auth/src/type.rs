use super::*;

/// Boxed `Send` future returned by an async [`Authenticator`].
pub type AuthFuture<'a> =
    Pin<Box<dyn Future<Output = Result<AuthPrincipal, AuthError>> + Send + 'a>>;

/// Thread-safe shared reference to a pluggable [`Authenticator`].
pub type SharedAuthenticator = Arc<dyn Authenticator>;
