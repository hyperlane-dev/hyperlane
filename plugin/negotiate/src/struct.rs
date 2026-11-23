/// The configuration of [`CompressMiddleware`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompressConfig {
    /// The body length below which a response is answered without a content coding.
    pub(crate) min_length: usize,
}

/// The middleware that answers with the content coding the client accepts.
///
/// # Examples
///
/// ```rust
/// use hyperlane::Server;
/// use hyperlane_plugin_negotiate::CompressMiddleware;
///
/// let mut server: Server = Server::default();
/// server.response_middleware::<CompressMiddleware>();
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct CompressMiddleware;

/// The middleware that answers a conditional request with `304 Not Modified`.
#[derive(Clone, Copy, Debug, Default)]
pub struct ConditionalMiddleware;

/// A strong entity-tag of a representation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntityTag(pub(crate) String);
