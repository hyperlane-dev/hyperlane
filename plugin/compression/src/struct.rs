use super::*;

/// The configuration of the compression plugin.
#[derive(Clone, Debug, Getter)]
pub struct CompressionConfig {
    /// The smallest body that is worth compressing.
    #[get(type(copy))]
    pub min_size: usize,
    /// The encodings that are offered, in the order they are preferred.
    pub encodings: Vec<Compress>,
}

/// The plugin that compresses the answers of a server.
#[derive(Clone, Copy, Debug, Default)]
pub struct CompressionPlugin;

/// The middleware that marks the answer with the encoding the client asked
/// for.
#[derive(Clone, Copy, Debug, Default)]
#[hyperlane::response_middleware(-110)]
pub struct CompressionMiddleware;
