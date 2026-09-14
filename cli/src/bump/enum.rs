/// Types of version bumps
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BumpVersionType {
    /// Bump patch version (0.1.0 -> 0.1.1)
    Patch,
    /// Bump minor version (0.1.0 -> 0.2.0)
    Minor,
    /// Bump major version (0.1.0 -> 1.0.0)
    Major,
    /// Remove pre-release identifier to make it a release version
    Release,
    /// Add or bump alpha pre-release version (0.1.0 -> 0.1.0-alpha, 0.1.0-alpha -> 0.1.0-alpha.1)
    Alpha,
    /// Add or bump beta pre-release version (0.1.0 -> 0.1.0-beta, 0.1.0-alpha.2 -> 0.1.0-beta.1)
    Beta,
    /// Add or bump rc pre-release version (0.1.0 -> 0.1.0-rc, 0.1.0-beta.1 -> 0.1.0-rc.1)
    Rc,
}
