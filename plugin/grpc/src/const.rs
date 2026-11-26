/// Canonical gRPC `Content-Type` header value.
pub const GRPC_CONTENT_TYPE: &str = "application/grpc";

/// Response header carrying the numeric gRPC status code.
pub const GRPC_STATUS_HEADER: &str = "grpc-status";

/// Response header carrying the percent-encoded gRPC error message.
pub const GRPC_MESSAGE_HEADER: &str = "grpc-message";

/// Request/response header identifying the gRPC payload compression coding.
pub const GRPC_ENCODING_HEADER: &str = "grpc-encoding";

/// Request header carrying the client-supplied RPC timeout budget.
pub const GRPC_TIMEOUT_HEADER: &str = "grpc-timeout";

/// Default maximum decoded gRPC message size (4 MiB).
pub const DEFAULT_GRPC_MAX_MESSAGE_SIZE: usize = 4 * 1024 * 1024;

/// Length of the 5-byte gRPC Length-Prefixed-Message header.
pub const GRPC_FRAME_HEADER_LEN: usize = 5;

/// Default decompression scratch buffer size.
pub(crate) const GRPC_DECOMPRESS_BUF_SIZE: usize = 4096;

/// Uppercase hexadecimal digits for RFC 3986 percent-encoding of `grpc-message`.
pub(crate) const GRPC_HEX_UPPER: &[u8; 16] = b"0123456789ABCDEF";

/// Identity encoding token indicating uncompressed gRPC frames.
pub(crate) const GRPC_IDENTITY_ENCODING: &str = "identity";

/// Error text when a gRPC payload is truncated before the 5-byte frame header.
pub(crate) const ERR_GRPC_TRUNCATED_HEADER: &str = "Truncated gRPC frame header";

/// Error text when a gRPC payload has an invalid compression flag (`> 1`).
pub(crate) const ERR_GRPC_INVALID_FLAG: &str = "Invalid gRPC compression flag";

/// Error text when a gRPC payload declares a length exceeding the configured cap.
pub(crate) const ERR_GRPC_MSG_TOO_LARGE: &str = "gRPC message exceeds max_message_size";

/// Error text when a gRPC payload ends before the declared frame length.
pub(crate) const ERR_GRPC_TRUNCATED_BODY: &str = "Truncated gRPC frame payload";

/// Error text when a frame is flagged compressed without a supported `grpc-encoding`.
pub(crate) const ERR_GRPC_UNSUPPORTED_ENCODING: &str = "Unsupported or missing grpc-encoding";

/// Error text when a gRPC method path is not registered in [`GrpcRouter`].
pub(crate) const ERR_GRPC_UNIMPLEMENTED: &str = "gRPC method not implemented";

/// Error text when a gRPC handler exceeds the caller's `grpc-timeout` deadline.
pub(crate) const ERR_GRPC_DEADLINE_EXCEEDED: &str = "gRPC deadline exceeded";
