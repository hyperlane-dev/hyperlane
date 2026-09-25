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
    /// Create a new project from template
    New,
    /// Generate template components
    Template,
    /// Show help
    Help,
    /// Show version
    Version,
}
