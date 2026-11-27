/// Long form of the help flag.
pub(crate) const FLAG_LONG_HELP: &str = "--help";

/// Long form of the version flag.
pub(crate) const FLAG_LONG_VERSION: &str = "--version";

/// Sub-command that starts the file watcher.
pub(crate) const COMMAND_WATCH: &str = "watch";

/// Sub-command that scaffolds a component from a template.
pub(crate) const COMMAND_TEMPLATE: &str = "template";

/// Sub-command that runs unified `crate-cli` and Hyperlane formatting.
pub(crate) const COMMAND_FMT: &str = "fmt";

/// Dry-run verification flag for the `fmt` sub-command.
pub(crate) const FLAG_FMT_CHECK: &str = "--check";

/// Optional target path flag for the `fmt` sub-command.
pub(crate) const FLAG_FMT_PATH: &str = "--path";
