/// Available commands
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommandType {
    /// Watch files using cargo-watch
    Watch,
    /// Create a new project from template
    New,
    /// Generate template components
    Template,
    /// Show help
    Help,
    /// Show version
    Version,
}
