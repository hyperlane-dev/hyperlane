use super::*;

/// Configuration for an MCP Streamable HTTP server endpoint.
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct McpServerConfig {
    /// Server implementation name advertised in `initialize`.
    pub(crate) name: String,
    /// Server implementation version advertised in `initialize`.
    pub(crate) version: String,
    /// MCP protocol revision string (default `"2025-03-26"`).
    pub(crate) protocol_version: String,
    /// HTTP path matched by [`McpHook`] (default `"/mcp"`).
    pub(crate) endpoint: String,
    /// Explicit allowlist of `Origin` values (empty permits all origins; non-empty blocks DNS rebinding).
    pub(crate) allowed_origins: Vec<String>,
    /// Maximum concurrent active MCP sessions tracked in memory.
    #[get(type(copy))]
    pub(crate) max_sessions: usize,
    /// Maximum incoming request body size in bytes.
    #[get(type(copy))]
    pub(crate) max_body_size: usize,
    /// Session idle TTL in milliseconds (`0` disables expiration).
    #[get(type(copy))]
    pub(crate) session_ttl_ms: u64,
}

/// Metadata describing one registered MCP tool.
#[derive(Clone, Data, Debug, Deserialize, PartialEq, Serialize)]
pub struct McpTool {
    /// Unique tool name.
    pub(crate) name: String,
    /// Human-readable description of what the tool does.
    pub(crate) description: String,
    /// JSON Schema describing the tool's input arguments object.
    #[serde(rename = "inputSchema")]
    pub(crate) input_schema: Value,
}

/// Result returned by an MCP tool invocation.
#[derive(Clone, Data, Debug, Deserialize, PartialEq, Serialize)]
pub struct McpToolResult {
    /// MCP content blocks (such as `{"type":"text","text":"..."}`).
    pub(crate) content: Vec<Value>,
    /// Marks the tool call result as a tool-level error when `true`.
    #[serde(rename = "isError")]
    #[get(type(copy))]
    pub(crate) is_error: bool,
}

/// Metadata describing one readable MCP resource.
#[derive(Clone, Data, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct McpResource {
    /// Canonical resource URI.
    pub(crate) uri: String,
    /// Human-readable resource display name.
    pub(crate) name: String,
    /// Short resource description.
    pub(crate) description: String,
    /// MIME media type (for example `"text/plain"` or `"application/json"`).
    #[serde(rename = "mimeType")]
    pub(crate) mime_type: String,
}

/// Content returned by reading one MCP resource.
#[derive(Clone, Data, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct McpResourceContent {
    /// Resource URI that produced this content.
    pub(crate) uri: String,
    /// MIME media type of `text`.
    #[serde(rename = "mimeType")]
    pub(crate) mime_type: String,
    /// UTF-8 textual content.
    pub(crate) text: String,
}

/// Metadata describing one prompt argument.
#[derive(Clone, Data, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct McpPromptArgument {
    /// Argument name.
    pub(crate) name: String,
    /// Argument description.
    pub(crate) description: String,
    /// Whether callers must supply this argument.
    #[get(type(copy))]
    pub(crate) required: bool,
}

/// Metadata describing one registered MCP prompt template.
#[derive(Clone, Data, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct McpPrompt {
    /// Unique prompt name.
    pub(crate) name: String,
    /// Prompt description.
    pub(crate) description: String,
    /// Declared prompt arguments.
    pub(crate) arguments: Vec<McpPromptArgument>,
}

/// Registered tool descriptor paired with its async handler.
#[derive(Clone)]
pub(crate) struct RegisteredTool {
    /// Public metadata advertised by `tools/list`.
    pub(crate) meta: McpTool,
    /// Async invocation handler.
    pub(crate) handler: McpToolHandler,
}

/// Registered resource descriptor paired with its async reader.
#[derive(Clone)]
pub(crate) struct RegisteredResource {
    /// Public metadata advertised by `resources/list`.
    pub(crate) meta: McpResource,
    /// Async content reader.
    pub(crate) handler: McpResourceHandler,
}

/// Registered prompt descriptor paired with its async renderer.
#[derive(Clone)]
pub(crate) struct RegisteredPrompt {
    /// Public metadata advertised by `prompts/list`.
    pub(crate) meta: McpPrompt,
    /// Async prompt renderer.
    pub(crate) handler: McpPromptHandler,
}

/// Thread-safe registry of MCP tools, resources, prompts, and sharded active client sessions.
#[derive(Clone)]
pub struct McpRegistry {
    /// Registered tools keyed by tool name (`XxHash3_64`).
    pub(crate) tools: HashMapXxHash3_64<String, RegisteredTool>,
    /// Registered resources keyed by resource URI (`XxHash3_64`).
    pub(crate) resources: HashMapXxHash3_64<String, RegisteredResource>,
    /// Registered prompts keyed by prompt name (`XxHash3_64`).
    pub(crate) prompts: HashMapXxHash3_64<String, RegisteredPrompt>,
    /// Sharded active session tables mapping session id to creation/activity timestamp.
    pub(crate) session_shards: Arc<Vec<RwLock<HashMapXxHash3_64<String, Instant>>>>,
    /// Lock-free active session count maintained across all shards.
    pub(crate) session_count: Arc<AtomicUsize>,
    /// Monotonic counter mixed with entropy to mint unpredictable session ids.
    pub(crate) next_session_seq: Arc<AtomicU64>,
    /// Per-registry entropy seed used when minting session identifiers.
    pub(crate) session_seed: u64,
}

/// Process-wide [`McpPlugin`] registry holding [`McpServerConfig`] and [`McpRegistry`].
#[derive(Clone)]
pub struct McpPlugin {
    /// Active MCP server configuration.
    pub(crate) config: McpServerConfig,
    /// Shared MCP tool/resource/prompt/session registry.
    pub(crate) registry: McpRegistry,
}

/// Opt-in [`ServerHook`] serving MCP Streamable HTTP (`POST` / `DELETE`) on the configured endpoint.
#[derive(Clone, Copy, Debug, Default)]
#[hyperlane::request_middleware(-60)]
pub struct McpHook;
