use super::*;

/// Pluggable asynchronous authenticator resolving a Hyperlane [`Context`] into an [`AuthPrincipal`].
pub trait Authenticator: Send + Sync + 'static {
    /// Authenticates the incoming request represented by `ctx`.
    ///
    /// # Arguments
    ///
    /// - `&'a self` - Shared reference to self.
    /// - `&'a Context` - Incoming request context.
    ///
    /// # Returns
    ///
    /// - `AuthFuture<'a>` - Future resolving to the verified [`AuthPrincipal`] or [`AuthError`].
    fn authenticate<'a>(&'a self, ctx: &'a Context) -> AuthFuture<'a>;
}
