/// Default MCP Streamable HTTP endpoint path.
pub const DEFAULT_MCP_ENDPOINT: &str = "/mcp";

/// Default MCP protocol revision advertised during `initialize`.
pub const DEFAULT_MCP_PROTOCOL_VERSION: &str = "2025-03-26";

/// HTTP header carrying the negotiated MCP session identifier.
pub const MCP_SESSION_ID_HEADER: &str = "mcp-session-id";

/// Default maximum concurrent MCP sessions stored in memory.
pub const DEFAULT_MCP_MAX_SESSIONS: usize = 1024;

/// Default maximum incoming JSON-RPC request body size (1 MiB).
pub const DEFAULT_MCP_MAX_BODY_SIZE: usize = 1024 * 1024;

/// Default MCP session idle TTL in milliseconds (30 minutes).
pub const DEFAULT_MCP_SESSION_TTL_MS: u64 = 1_800_000;

/// Number of independent session shards to reduce write-lock contention.
pub(crate) const MCP_SESSION_SHARD_COUNT: usize = 16;

/// Prefix prepended to generated MCP session identifiers.
pub(crate) const MCP_SESSION_PREFIX: &str = "mcp-sess-";

/// Lowercase hexadecimal alphabet used for stack-allocated session token formatting.
pub(crate) const MCP_HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";

/// SplitMix64 Weyl increment constant.
pub(crate) const SPLITMIX_GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;

/// SplitMix64 first avalanche multiplier.
pub(crate) const SPLITMIX_MUL1: u64 = 0xBF58_476D_1CE4_E5B9;

/// SplitMix64 second avalanche multiplier.
pub(crate) const SPLITMIX_MUL2: u64 = 0x94D0_49BB_1331_11EB;

/// JSON-RPC version string required by MCP messages.
pub(crate) const MCP_JSONRPC_VERSION: &str = "2.0";

/// Default server display name when none is supplied.
pub(crate) const DEFAULT_MCP_SERVER_NAME: &str = "hyperlane-mcp";

/// Default server version when none is supplied.
pub(crate) const DEFAULT_MCP_SERVER_VERSION: &str = "1.0.0";

/// JSON-RPC Parse Error code.
pub(crate) const MCP_ERR_PARSE: i64 = -32700;

/// JSON-RPC Invalid Request code.
pub(crate) const MCP_ERR_INVALID_REQUEST: i64 = -32600;

/// JSON-RPC Method Not Found code.
pub(crate) const MCP_ERR_METHOD_NOT_FOUND: i64 = -32601;

/// JSON-RPC Invalid Params code.
pub(crate) const MCP_ERR_INVALID_PARAMS: i64 = -32602;

/// JSON-RPC Internal Error code.
pub(crate) const MCP_ERR_INTERNAL: i64 = -32603;

/// MCP `initialize` method name.
pub(crate) const METHOD_INITIALIZE: &str = "initialize";

/// MCP `notifications/initialized` method name.
pub(crate) const METHOD_NOTIF_INITIALIZED: &str = "notifications/initialized";

/// MCP `ping` method name.
pub(crate) const METHOD_PING: &str = "ping";

/// MCP `tools/list` method name.
pub(crate) const METHOD_TOOLS_LIST: &str = "tools/list";

/// MCP `tools/call` method name.
pub(crate) const METHOD_TOOLS_CALL: &str = "tools/call";

/// MCP `resources/list` method name.
pub(crate) const METHOD_RESOURCES_LIST: &str = "resources/list";

/// MCP `resources/read` method name.
pub(crate) const METHOD_RESOURCES_READ: &str = "resources/read";

/// MCP `prompts/list` method name.
pub(crate) const METHOD_PROMPTS_LIST: &str = "prompts/list";

/// MCP `prompts/get` method name.
pub(crate) const METHOD_PROMPTS_GET: &str = "prompts/get";

/// Error text for JSON parse failure.
pub(crate) const MSG_MCP_PARSE_ERROR: &str = "Parse error";

/// Error text for malformed JSON-RPC envelope.
pub(crate) const MSG_MCP_INVALID_REQUEST: &str = "Invalid Request";

/// Error text for unknown MCP method.
pub(crate) const MSG_MCP_METHOD_NOT_FOUND: &str = "Method not found";

/// Error text for missing or invalid method parameters.
pub(crate) const MSG_MCP_INVALID_PARAMS: &str = "Invalid params";

/// Error text when the MCP session table reaches `max_sessions`.
pub(crate) const MSG_MCP_SESSION_LIMIT: &str = "MCP session limit reached";

/// Error text when an unknown tool name is invoked.
pub(crate) const MSG_MCP_UNKNOWN_TOOL: &str = "Unknown MCP tool";

/// Error text when an unknown resource URI is requested.
pub(crate) const MSG_MCP_UNKNOWN_RESOURCE: &str = "Unknown MCP resource";

/// Error text when an unknown prompt name is requested.
pub(crate) const MSG_MCP_UNKNOWN_PROMPT: &str = "Unknown MCP prompt";

/// JSON field key for JSON-RPC version.
pub(crate) const KEY_JSONRPC: &str = "jsonrpc";

/// JSON field key for JSON-RPC identifier.
pub(crate) const KEY_ID: &str = "id";

/// JSON field key for JSON-RPC method name.
pub(crate) const KEY_METHOD: &str = "method";

/// JSON field key for JSON-RPC params.
pub(crate) const KEY_PARAMS: &str = "params";

/// JSON field key for JSON-RPC result.
pub(crate) const KEY_RESULT: &str = "result";

/// JSON field key for JSON-RPC error object.
pub(crate) const KEY_ERROR: &str = "error";

/// JSON field key for JSON-RPC error code.
pub(crate) const KEY_CODE: &str = "code";

/// JSON field key for JSON-RPC error message.
pub(crate) const KEY_MESSAGE: &str = "message";

/// JSON field key for MCP content block type.
pub(crate) const KEY_TYPE: &str = "type";

/// JSON field key and value for MCP text content.
pub(crate) const KEY_TEXT: &str = "text";

/// JSON field key for MCP protocol version.
pub(crate) const KEY_PROTOCOL_VERSION: &str = "protocolVersion";

/// JSON field key for MCP server capabilities.
pub(crate) const KEY_CAPABILITIES: &str = "capabilities";

/// JSON field key for MCP tools capability and list field.
pub(crate) const KEY_TOOLS: &str = "tools";

/// JSON field key for MCP resources capability and list field.
pub(crate) const KEY_RESOURCES: &str = "resources";

/// JSON field key for MCP prompts capability and list field.
pub(crate) const KEY_PROMPTS: &str = "prompts";

/// JSON field key for MCP listChanged capability flag.
pub(crate) const KEY_LIST_CHANGED: &str = "listChanged";

/// JSON field key for MCP subscribe capability flag.
pub(crate) const KEY_SUBSCRIBE: &str = "subscribe";

/// JSON field key for MCP serverInfo object.
pub(crate) const KEY_SERVER_INFO: &str = "serverInfo";

/// JSON field key for entity name.
pub(crate) const KEY_NAME: &str = "name";

/// JSON field key for server version.
pub(crate) const KEY_VERSION: &str = "version";

/// JSON field key for tool or prompt arguments.
pub(crate) const KEY_ARGUMENTS: &str = "arguments";

/// JSON field key for resource URI.
pub(crate) const KEY_URI: &str = "uri";

/// JSON field key for resource read contents list.
pub(crate) const KEY_CONTENTS: &str = "contents";
