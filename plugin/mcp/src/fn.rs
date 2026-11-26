use super::*;

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
