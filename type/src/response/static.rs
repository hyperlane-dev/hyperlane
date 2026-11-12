use super::*;

/// The default content type shared by responses without an explicit one.
pub(crate) static DEFAULT_CONTENT_TYPE: OnceLock<String> = OnceLock::new();
