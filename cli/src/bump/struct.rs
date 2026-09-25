/// Parsed version components following semantic versioning
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Version {
    /// Major version number
    pub major: u64,
    /// Minor version number
    pub minor: u64,
    /// Patch version number
    pub patch: u64,
    /// Optional pre-release identifier (e.g., "alpha", "beta", "rc.1")
    pub prerelease: Option<String>,
}
