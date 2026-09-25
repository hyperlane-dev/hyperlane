use super::*;

/// Regex pattern to match derive attribute
///
/// This pattern matches `#[derive(...)]` attributes in Rust code.
pub static DERIVE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    regex::Regex::new(r"#\[derive\s*\(([^)]+)\)\]").expect("Invalid regex pattern")
});
