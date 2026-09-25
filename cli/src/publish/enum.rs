use super::*;

/// Error types for publish operation
#[derive(Debug, thiserror::Error)]
pub enum PublishError {
    /// Failed to parse Cargo.toml
    #[error("Failed to parse Cargo.toml")]
    ManifestParseError,
    /// Circular dependency detected
    #[error("Circular dependency detected")]
    CircularDependency,
    /// Sync step failed before publish could start
    #[error("workspace sync failed: {0}")]
    SyncFailed(#[from] crate::sync::SyncError),
    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
}
