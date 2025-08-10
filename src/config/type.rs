use crate::*;

/// A type alias for a `Result` that represents either a `ServerConfig` or a `serde_json::Error`.
///
/// This is used for operations that can fail during `ServerConfig` deserialization.
pub type ServerConfigResult = Result<ServerConfig, serde_json::Error>;
