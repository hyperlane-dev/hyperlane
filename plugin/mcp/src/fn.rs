use super::*;

/// Computes the shard index for an MCP session identifier using `XxHash3_64`.
///
/// # Arguments
///
/// - `&str` - Session identifier.
///
/// # Returns
///
/// - `usize` - Shard index in `0..MCP_SESSION_SHARD_COUNT`.
pub(crate) fn mcp_session_shard_index(session_id: &str) -> usize {
    let mut hasher: XxHash3_64 = XxHash3_64::default();
    hasher.write(session_id.as_bytes());
    (hasher.finish() as usize) & (MCP_SESSION_SHARD_COUNT - 1)
}

/// Applies one SplitMix64 avalanche step to `x`.
///
/// # Arguments
///
/// - `u64` - Input state value.
///
/// # Returns
///
/// - `u64` - Mixed 64-bit output.
pub(crate) fn mix_splitmix64(mut x: u64) -> u64 {
    x = (x ^ (x >> 30)).wrapping_mul(SPLITMIX_MUL1);
    x = (x ^ (x >> 27)).wrapping_mul(SPLITMIX_MUL2);
    x ^ (x >> 31)
}

/// Generates a process-unique 64-bit seed from wall-clock nanoseconds, PID, and stack ASLR entropy.
///
/// # Returns
///
/// - `u64` - Non-trivial per-registry entropy seed.
pub(crate) fn mcp_random_seed() -> u64 {
    let nanos: u128 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|dur: Duration| dur.as_nanos())
        .unwrap_or(0);
    let pid: u64 = process::id() as u64;
    let stack_probe: u64 = (&nanos as *const u128) as usize as u64;
    let mixed: u64 =
        (nanos as u64) ^ ((nanos >> 64) as u64).rotate_left(17) ^ pid.rotate_left(33) ^ stack_probe;
    mix_splitmix64(mixed.wrapping_add(SPLITMIX_GAMMA))
}

/// Mints an unpredictable 128-bit hexadecimal MCP session identifier.
///
/// # Arguments
///
/// - `u64` - Registry entropy seed.
/// - `u64` - Monotonic sequence number.
///
/// # Returns
///
/// - `String` - Session identifier prefixed with [`MCP_SESSION_PREFIX`].
pub(crate) fn mint_mcp_session_id(seed: u64, seq: u64) -> String {
    let nanos: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|dur: Duration| dur.as_nanos() as u64)
        .unwrap_or(0);
    let hi: u64 =
        mix_splitmix64(seed.wrapping_add(seq.wrapping_mul(SPLITMIX_GAMMA)) ^ nanos.rotate_left(13));
    let lo: u64 = mix_splitmix64(hi.wrapping_add(SPLITMIX_GAMMA) ^ seq.rotate_left(29));
    let mut out: String = String::with_capacity(MCP_SESSION_PREFIX.len() + 32);
    out.push_str(MCP_SESSION_PREFIX);
    let words: [u64; 2] = [hi, lo];
    for word in words {
        for shift in (0..16).rev() {
            let nibble: usize = ((word >> (shift * 4)) & 0x0F) as usize;
            out.push(MCP_HEX_DIGITS[nibble] as char);
        }
    }
    out
}

/// Builds a single-entry JSON object `{ key: value }`.
///
/// # Arguments
///
/// - `&str` - Property name.
/// - `Value` - Property value.
///
/// # Returns
///
/// - `Value` - Constructed JSON object value.
pub(crate) fn mcp_single_field_object(key: &str, value: Value) -> Value {
    let mut map: Map<String, Value> = Map::with_capacity(1);
    map.insert(key.to_owned(), value);
    Value::Object(map)
}

/// Builds a standard MCP text content block `{"type": "text", "text": text}`.
///
/// # Arguments
///
/// - `String` - Plain-text content payload.
///
/// # Returns
///
/// - `Value` - Constructed text content JSON object.
pub(crate) fn mcp_text_content(text: String) -> Value {
    let mut map: Map<String, Value> = Map::with_capacity(2);
    map.insert(KEY_TYPE.to_owned(), Value::String(KEY_TEXT.to_owned()));
    map.insert(KEY_TEXT.to_owned(), Value::String(text));
    Value::Object(map)
}

/// Builds the MCP `initialize` response payload for `config`.
///
/// # Arguments
///
/// - `&McpServerConfig` - Active server metadata and protocol version.
///
/// # Returns
///
/// - `Value` - Constructed `initialize` result JSON object.
pub(crate) fn mcp_initialize_result(config: &McpServerConfig) -> Value {
    let mut tools_cap: Map<String, Value> = Map::with_capacity(1);
    tools_cap.insert(KEY_LIST_CHANGED.to_owned(), Value::Bool(false));
    let mut resources_cap: Map<String, Value> = Map::with_capacity(2);
    resources_cap.insert(KEY_SUBSCRIBE.to_owned(), Value::Bool(false));
    resources_cap.insert(KEY_LIST_CHANGED.to_owned(), Value::Bool(false));
    let mut prompts_cap: Map<String, Value> = Map::with_capacity(1);
    prompts_cap.insert(KEY_LIST_CHANGED.to_owned(), Value::Bool(false));
    let mut capabilities: Map<String, Value> = Map::with_capacity(3);
    capabilities.insert(KEY_TOOLS.to_owned(), Value::Object(tools_cap));
    capabilities.insert(KEY_RESOURCES.to_owned(), Value::Object(resources_cap));
    capabilities.insert(KEY_PROMPTS.to_owned(), Value::Object(prompts_cap));
    let mut server_info: Map<String, Value> = Map::with_capacity(2);
    server_info.insert(
        KEY_NAME.to_owned(),
        Value::String(config.get_name().clone()),
    );
    server_info.insert(
        KEY_VERSION.to_owned(),
        Value::String(config.get_version().clone()),
    );
    let mut result: Map<String, Value> = Map::with_capacity(3);
    result.insert(
        KEY_PROTOCOL_VERSION.to_owned(),
        Value::String(config.get_protocol_version().clone()),
    );
    result.insert(KEY_CAPABILITIES.to_owned(), Value::Object(capabilities));
    result.insert(KEY_SERVER_INFO.to_owned(), Value::Object(server_info));
    Value::Object(result)
}

/// Formats a JSON-RPC 2.0 error envelope into UTF-8 JSON bytes.
///
/// # Arguments
///
/// - `Value` - Request correlation identifier (`Value::Null` if unavailable).
/// - `i64` - JSON-RPC error code.
/// - `&str` - Human-readable error message.
///
/// # Returns
///
/// - `Vec<u8>` - Serialized JSON-RPC error response.
pub(crate) fn mcp_json_error(id: Value, code: i64, message: &str) -> Vec<u8> {
    let mut err_obj: Map<String, Value> = Map::with_capacity(2);
    err_obj.insert(KEY_CODE.to_owned(), Value::from(code));
    err_obj.insert(KEY_MESSAGE.to_owned(), Value::String(message.to_owned()));
    let mut payload: Map<String, Value> = Map::with_capacity(3);
    payload.insert(
        KEY_JSONRPC.to_owned(),
        Value::String(MCP_JSONRPC_VERSION.to_owned()),
    );
    payload.insert(KEY_ID.to_owned(), id);
    payload.insert(KEY_ERROR.to_owned(), Value::Object(err_obj));
    to_vec(&Value::Object(payload)).unwrap_or_default()
}

/// Formats a JSON-RPC 2.0 success envelope into UTF-8 JSON bytes.
///
/// # Arguments
///
/// - `Value` - Request correlation identifier.
/// - `Value` - Result object value.
///
/// # Returns
///
/// - `Vec<u8>` - Serialized JSON-RPC success response.
pub(crate) fn mcp_json_result(id: Value, result: Value) -> Vec<u8> {
    let mut payload: Map<String, Value> = Map::with_capacity(3);
    payload.insert(
        KEY_JSONRPC.to_owned(),
        Value::String(MCP_JSONRPC_VERSION.to_owned()),
    );
    payload.insert(KEY_ID.to_owned(), id);
    payload.insert(KEY_RESULT.to_owned(), result);
    to_vec(&Value::Object(payload)).unwrap_or_default()
}

/// Validates an incoming `Origin` header against `allowed_origins` to mitigate DNS rebinding.
///
/// # Arguments
///
/// - `Option<&str>` - Incoming `Origin` header value.
/// - `&[String]` - Configured allowed origins (empty allows all origins).
///
/// # Returns
///
/// - `bool` - `true` when the request origin is permitted.
pub(crate) fn is_origin_allowed(origin: Option<&str>, allowed_origins: &[String]) -> bool {
    if allowed_origins.is_empty() {
        return true;
    }
    let Some(value) = origin else {
        return true;
    };
    allowed_origins
        .iter()
        .any(|allowed: &String| allowed.eq_ignore_ascii_case(value.trim()))
}
