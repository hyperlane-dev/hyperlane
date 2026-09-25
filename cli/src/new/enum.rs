use super::*;

/// Errors that can occur during project creation
#[derive(Debug, thiserror::Error)]
pub enum NewError {
    /// IO error occurred
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    /// Git command not found
    #[error("Git is not installed or not found in PATH")]
    GitNotFound,
    /// Project already exists
    #[error("Project directory '{0}' already exists")]
    ProjectExists(String),
    /// Git clone failed
    #[error("Git clone failed: {0}")]
    CloneFailed(String),
    /// Invalid project name
    #[error("Invalid project name: {0}")]
    InvalidName(String),
}
