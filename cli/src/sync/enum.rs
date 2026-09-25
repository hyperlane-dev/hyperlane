use super::*;

/// Error types for sync operation
#[derive(Debug, thiserror::Error)]
pub enum SyncError {
    /// Failed to parse Cargo.toml
    #[error("Failed to parse Cargo.toml")]
    ManifestParseError,
    /// Failed to serialize Cargo.toml
    #[error("Failed to serialize Cargo.toml")]
    ManifestSerializeError,
    /// Workspace package version not found
    #[error("workspace.package.version not found in {0}")]
    WorkspaceVersionMissing(String),
    /// Workspace members list not found
    #[error("workspace.members not found in {0}")]
    WorkspaceMembersMissing(String),
    /// Member crate Cargo.toml is missing
    #[error("member crate Cargo.toml not found: {0}")]
    MemberManifestMissing(String),
    /// Member crate name not found
    #[error("member crate [package].name not found in {0}")]
    MemberNameMissing(String),
    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
}
