/// Errors returned by MCP tool, resource, prompt, or protocol handlers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum McpError {
    /// Invalid or missing parameters (`-32602`).
    InvalidParams(String),
    /// Requested tool, resource, or prompt was not found (`-32601`).
    NotFound(String),
    /// Internal execution failure (`-32603`).
    Internal(String),
}

/// Outcome returned by [`crate::McpRegistry::handle_json_bytes`] for HTTP transport translation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum McpDispatchOutcome {
    /// JSON response payload plus optional newly assigned `Mcp-Session-Id`.
    Response {
        /// Serialized JSON-RPC response bytes.
        body: Vec<u8>,
        /// Newly minted session id when answering `initialize`.
        new_session_id: Option<String>,
    },
    /// Notification accepted with HTTP `202 Accepted` and no body.
    Accepted,
    /// Caller supplied an unknown or expired `Mcp-Session-Id` (`404 Not Found`).
    UnknownSession,
    /// Active session table reached `max_sessions` (`503 Service Unavailable`).
    SessionLimitReached,
}
