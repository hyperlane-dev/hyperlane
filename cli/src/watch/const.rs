/// Program re-run by the watch command on every source change.
pub(crate) const CARGO_RUN_PROGRAM: &str = "cargo";

/// Message reported when the watch command runs outside a crate.
pub(crate) const ERROR_SRC_DIRECTORY_NOT_FOUND: &str =
    "src directory not found in current directory";
