/// Available commands
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommandType {
    /// Format code using cargo fmt
    Fmt,
    /// Watch files using cargo-watch
    Watch,
    /// Bump version in Cargo.toml
    Bump,
    /// Publish packages in monorepo
    Publish,
    /// Sync [workspace.dependencies] entry versions and aliases with the
    /// [workspace.package].version declared in the workspace root and the
    /// `[package].name` declared in each member crate.
    Sync,
    /// Create a new project from template
    New,
    /// Generate template components
    Template,
    /// Show help
    Help,
    /// Show version
    Version,
}
