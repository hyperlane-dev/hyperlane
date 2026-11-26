use super::*;

/// Global initialized [`McpPlugin`] instance.
pub(crate) static MCP_PLUGIN: OnceLock<McpPlugin> = OnceLock::new();
