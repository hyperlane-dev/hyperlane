use super::*;

/// Boxed `Send` future returned by an async MCP tool handler.
pub type McpToolFuture = Pin<Box<dyn Future<Output = Result<McpToolResult, McpError>> + Send>>;

/// Thread-safe async MCP tool handler receiving tool arguments JSON.
pub type McpToolHandler = Arc<dyn Fn(Value) -> McpToolFuture + Send + Sync>;

/// Boxed `Send` future returned by an async MCP resource reader.
pub type McpResourceFuture =
    Pin<Box<dyn Future<Output = Result<McpResourceContent, McpError>> + Send>>;

/// Thread-safe async MCP resource reader receiving the requested URI.
pub type McpResourceHandler = Arc<dyn Fn(String) -> McpResourceFuture + Send + Sync>;

/// Boxed `Send` future returned by an async MCP prompt renderer.
pub type McpPromptFuture = Pin<Box<dyn Future<Output = Result<Value, McpError>> + Send>>;

/// Thread-safe async MCP prompt handler receiving optional prompt arguments JSON.
pub type McpPromptHandler = Arc<dyn Fn(Value) -> McpPromptFuture + Send + Sync>;
