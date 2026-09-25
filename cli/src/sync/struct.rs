/// Result of a sync operation: which entries were rewritten.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyncReport {
    /// Workspace version that was applied to every member entry.
    pub workspace_version: String,
    /// Pair of (old dep LHS, new dep LHS) for entries whose dep alias
    /// was renamed to match the member's actual `[package].name`.
    pub renamed_entries: Vec<(String, String)>,
    /// Pair of (member path, new dep LHS) for entries whose version
    /// literal was rewritten to `workspace_version`.
    pub versioned_entries: Vec<(String, String)>,
    /// `true` if the workspace Cargo.toml was mutated by this run.
    pub file_changed: bool,
}
