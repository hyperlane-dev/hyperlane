use super::*;

/// Structured gRPC error carrying a canonical [`GrpcStatus`] and message.
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct GrpcError {
    /// Canonical gRPC status code.
    #[get(type(copy))]
    pub(crate) status: GrpcStatus,
    /// Human-readable error message (percent-encoded on the wire).
    pub(crate) message: String,
}

/// Decoded 5-byte length-prefixed gRPC message frame.
#[derive(Clone, Data, Debug, Default, Eq, PartialEq)]
pub struct GrpcFrame {
    /// Whether the wire frame had its compression flag set (`1`).
    #[get(type(copy))]
    pub(crate) compressed: bool,
    /// Uncompressed message payload bytes.
    pub(crate) payload: Vec<u8>,
}

/// Configuration governing gRPC message size limits and response compression.
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct GrpcConfig {
    /// Maximum encoded and decoded gRPC message size in bytes.
    #[get(type(copy))]
    pub(crate) max_message_size: usize,
    /// Compression algorithm applied to outgoing response frames (`Compress::Unknown` = identity).
    #[get(type(copy))]
    pub(crate) compression: Compress,
}

/// Thread-safe registry mapping `/package.Service/Method` paths to unary async handlers.
#[derive(Clone)]
pub struct GrpcRouter {
    /// Registered unary handlers keyed by full gRPC path with `XxHash3_64`.
    pub(crate) methods: HashMapXxHash3_64<String, GrpcHandler>,
    /// Registered interceptors invoked before each gRPC call.
    pub(crate) interceptors: Vec<SharedGrpcInterceptor>,
}

/// Process-wide [`GrpcPlugin`] registry holding [`GrpcConfig`] and [`GrpcRouter`].
#[derive(Clone)]
pub struct GrpcPlugin {
    /// Active gRPC configuration.
    pub(crate) config: GrpcConfig,
    /// Registered gRPC service router.
    pub(crate) router: GrpcRouter,
}

/// Opt-in [`ServerHook`] dispatching gRPC `POST` requests with `Content-Type: application/grpc*`.
#[derive(Clone, Copy, Debug, Default)]
#[hyperlane::request_middleware(-65)]
pub struct GrpcHook;
