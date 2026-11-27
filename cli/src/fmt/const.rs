/// Cargo manifest filename detected when running workspace `cargo fmt`.
pub(crate) const FMT_CARGO_TOML: &str = "Cargo.toml";

/// Cargo executable name.
pub(crate) const FMT_BIN_CARGO: &str = "cargo";

/// Cargo `fmt` subcommand argument.
pub(crate) const FMT_ARG_FMT: &str = "fmt";

/// Cargo `--all` workspace formatting flag.
pub(crate) const FMT_ARG_ALL: &str = "--all";

/// Separator preceding rustfmt flags.
pub(crate) const FMT_ARG_DOUBLE_DASH: &str = "--";

/// Dry-run verification flag forwarded to rustfmt.
pub(crate) const FMT_ARG_CHECK: &str = "--check";

/// Rust source file extension scanned for Hyperlane attribute formatting.
pub(crate) const FMT_EXT_RS: &str = "rs";

/// Directories skipped during recursive Rust source traversal.
pub(crate) const FMT_SKIP_DIRS: &[&str] = &[
    ".git",
    ".cargo",
    ".codegraph",
    "target",
    "node_modules",
    "dist",
    "build",
    "vendor",
];

/// Hyperlane procedural attribute names whose single-line parameter whitespace is normalized.
pub(crate) const HYPERLANE_ATTR_NAMES: &[&str] = &[
    "route",
    "request_middleware",
    "response_middleware",
    "task_panic",
    "request_error",
    "hyperlane",
    "methods",
    "host",
    "referer",
    "filter",
    "send",
    "try_send",
    "flush",
    "try_flush",
    "http",
    "ws",
    "h2c",
    "reject",
    "aborted",
    "closed",
    "epilogue",
    "prologue",
    "response_header",
    "response_body",
    "response_reason_phrase",
    "response_status_code",
    "response_version",
    "request_version",
    "request_body",
    "request_body_json",
    "request_header",
    "request_headers",
    "request_query",
    "request_querys",
    "request_cookie",
    "request_cookies",
    "route_param",
    "route_params",
    "attribute",
    "attributes",
    "clear_response_headers",
];

/// Error prefix returned when `hyperlane fmt --check` detects unformatted Hyperlane attributes.
pub(crate) const ERR_HYPERLANE_ATTR_CHECK_FAILED: &str =
    "unformatted Hyperlane attributes found in";

/// Error prefix returned when `cargo fmt` fails.
pub(crate) const ERR_CARGO_FMT_FAILED: &str = "cargo fmt exited with non-zero status";
