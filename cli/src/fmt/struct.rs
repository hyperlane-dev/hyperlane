use super::*;

/// Structured formatting summary returned by `hyperlane fmt`.
#[derive(Clone, Data, Debug, Default, Eq, New, PartialEq)]
pub struct HyperlaneFmtReport {
    /// Number of non-Rust text files scanned by `crate-cli`.
    #[get(type(copy))]
    pub(crate) text_files_scanned: usize,
    /// Number of non-Rust text files formatted by `crate-cli`.
    #[get(type(copy))]
    pub(crate) text_files_formatted: usize,
    /// Number of `.rs` files scanned for Hyperlane attribute formatting.
    #[get(type(copy))]
    pub(crate) rust_files_scanned: usize,
    /// Number of `.rs` files whose Hyperlane attributes required normalization.
    #[get(type(copy))]
    pub(crate) rust_files_formatted: usize,
    /// Whether `--check` dry-run mode was active.
    #[get(type(copy))]
    pub(crate) check_only: bool,
    /// Display paths of all files modified (or needing modification in `--check` mode).
    pub(crate) changed_files: Vec<String>,
}
