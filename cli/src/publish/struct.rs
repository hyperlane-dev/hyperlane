use super::*;

/// Package information in monorepo
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Package {
    /// Package name
    pub name: String,
    /// Package version
    pub version: String,
    /// Path to package directory
    pub path: PathBuf,
    /// Dependencies within the monorepo
    pub local_dependencies: Vec<String>,
}

/// Publish result for a single package
#[derive(Clone, Debug)]
pub struct PublishResult {
    /// Package name
    pub package_name: String,
    /// Whether publish succeeded
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
    /// Number of retries performed
    pub retries: u32,
}
