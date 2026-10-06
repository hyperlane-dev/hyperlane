pub(crate) const SERVER_TYPE_KEY: &str = "Server";

/// The crate name segment checked when matching a `Context` parameter type.
pub(crate) const HYPERLANE_CRATE_NAME: &str = "hyperlane";

/// The type name segment checked when matching a `Context` parameter type.
pub(crate) const CONTEXT_TYPE_NAME: &str = "Context";

/// The type name segment checked when matching a `Stream` parameter type.
pub(crate) const STREAM_TYPE_NAME: &str = "Stream";

/// The error message used when a context parameter is not a plain identifier.
pub(crate) const EXPECTED_IDENTIFIER_FOR_CONTEXT_PARAMETER: &str =
    "expected identifier for context parameter";

/// The error message used when a function signature has no context parameter.
pub(crate) const EXPECTED_CONTEXT_PARAMETER: &str =
    "expected at least one parameter of type &::hyperlane::Context";

/// The error message used when a stream parameter is not a plain identifier.
pub(crate) const EXPECTED_IDENTIFIER_FOR_STREAM_PARAMETER: &str =
    "expected identifier for stream parameter";

/// The error message used when a function signature has no stream parameter.
pub(crate) const EXPECTED_STREAM_PARAMETER: &str =
    "expected at least one parameter of type &::hyperlane::Stream";

/// The error message used when a hook `order` literal is not a valid `isize`.
pub(crate) const CANNOT_PARSE_TO_ISIZE: &str = "Cannot parse to isize";

/// The registered name of the `closed` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_CLOSED: &str = "closed";

/// The registered name of the `filter` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_FILTER: &str = "filter";

/// The registered name of the `try_flush` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_TRY_FLUSH: &str = "try_flush";

/// The registered name of the `flush` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_FLUSH: &str = "flush";

/// The registered name of the `task_panic` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_TASK_PANIC: &str = "task_panic";

/// The registered name of the `request_error` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_REQUEST_ERROR: &str = "request_error";

/// The registered name of the `prologue_hooks` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_PROLOGUE_HOOKS: &str = "prologue_hooks";

/// The registered name of the `epilogue_hooks` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_EPILOGUE_HOOKS: &str = "epilogue_hooks";

/// The registered name of the `host` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_HOST: &str = "host";

/// The registered name of the `reject_host` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_REJECT_HOST: &str = "reject_host";

/// The registered name of the `hyperlane` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_HYPERLANE: &str = "hyperlane";

/// The registered name of the `methods` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_METHODS: &str = "methods";

/// The registered name of the `is_get_method` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_IS_GET_METHOD: &str = "is_get_method";

/// The registered name of the `is_post_method` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_IS_POST_METHOD: &str = "is_post_method";

/// The registered name of the `is_put_method` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_IS_PUT_METHOD: &str = "is_put_method";

/// The registered name of the `is_delete_method` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_IS_DELETE_METHOD: &str = "is_delete_method";

/// The registered name of the `is_patch_method` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_IS_PATCH_METHOD: &str = "is_patch_method";

/// The registered name of the `is_head_method` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_IS_HEAD_METHOD: &str = "is_head_method";

/// The registered name of the `is_options_method` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_IS_OPTIONS_METHOD: &str = "is_options_method";

/// The registered name of the `is_connect_method` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_IS_CONNECT_METHOD: &str = "is_connect_method";

/// The registered name of the `is_trace_method` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_IS_TRACE_METHOD: &str = "is_trace_method";

/// The registered name of the `is_unknown_method` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_IS_UNKNOWN_METHOD: &str = "is_unknown_method";

/// The registered name of the `referer` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_REFERER: &str = "referer";

/// The registered name of the `reject_referer` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_REJECT_REFERER: &str = "reject_referer";

/// The registered name of the `reject` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_REJECT: &str = "reject";

/// The registered name of the `request_body` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_REQUEST_BODY: &str = "request_body";

/// The registered name of the `request_body_json_result` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_REQUEST_BODY_JSON_RESULT: &str = "request_body_json_result";

/// The registered name of the `request_body_json` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_REQUEST_BODY_JSON: &str = "request_body_json";

/// The registered name of the `try_get_attribute` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_TRY_GET_ATTRIBUTE: &str = "try_get_attribute";

/// The registered name of the `attribute` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_ATTRIBUTE: &str = "attribute";

/// The registered name of the `attributes` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_ATTRIBUTES: &str = "attributes";

/// The registered name of the `try_get_task_panic_data` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_TRY_GET_TASK_PANIC_DATA: &str = "try_get_task_panic_data";

/// The registered name of the `task_panic_data` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_TASK_PANIC_DATA: &str = "task_panic_data";

/// The registered name of the `try_get_request_error_data` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_TRY_GET_REQUEST_ERROR_DATA: &str =
    "try_get_request_error_data";

/// The registered name of the `request_error_data` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_REQUEST_ERROR_DATA: &str = "request_error_data";

/// The registered name of the `try_get_route_param` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_TRY_GET_ROUTE_PARAM: &str = "try_get_route_param";

/// The registered name of the `route_param` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_ROUTE_PARAM: &str = "route_param";

/// The registered name of the `route_params` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_ROUTE_PARAMS: &str = "route_params";

/// The registered name of the `try_get_request_query` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_TRY_GET_REQUEST_QUERY: &str = "try_get_request_query";

/// The registered name of the `request_query` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_REQUEST_QUERY: &str = "request_query";

/// The registered name of the `request_querys` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_REQUEST_QUERYS: &str = "request_querys";

/// The registered name of the `try_get_request_header` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_TRY_GET_REQUEST_HEADER: &str = "try_get_request_header";

/// The registered name of the `request_header` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_REQUEST_HEADER: &str = "request_header";

/// The registered name of the `request_headers` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_REQUEST_HEADERS: &str = "request_headers";

/// The registered name of the `try_get_request_cookie` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_TRY_GET_REQUEST_COOKIE: &str = "try_get_request_cookie";

/// The registered name of the `request_cookie` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_REQUEST_COOKIE: &str = "request_cookie";

/// The registered name of the `request_cookies` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_REQUEST_COOKIES: &str = "request_cookies";

/// The registered name of the `request_version` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_REQUEST_VERSION: &str = "request_version";

/// The registered name of the `request_path` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_REQUEST_PATH: &str = "request_path";

/// The registered name of the `request_middleware` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_REQUEST_MIDDLEWARE: &str = "request_middleware";

/// The registered name of the `response_status_code` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_RESPONSE_STATUS_CODE: &str = "response_status_code";

/// The registered name of the `response_reason_phrase` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_RESPONSE_REASON_PHRASE: &str = "response_reason_phrase";

/// The registered name of the `response_header` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_RESPONSE_HEADER: &str = "response_header";

/// The registered name of the `response_body` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_RESPONSE_BODY: &str = "response_body";

/// The registered name of the `clear_response_headers` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_CLEAR_RESPONSE_HEADERS: &str = "clear_response_headers";

/// The registered name of the `response_version` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_RESPONSE_VERSION: &str = "response_version";

/// The registered name of the `response_middleware` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_RESPONSE_MIDDLEWARE: &str = "response_middleware";

/// The registered name of the `route` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_ROUTE: &str = "route";

/// The registered name of the `try_send` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_TRY_SEND: &str = "try_send";

/// The registered name of the `send` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_SEND: &str = "send";

/// The registered name of the `try_get_http_request` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_TRY_GET_HTTP_REQUEST: &str = "try_get_http_request";

/// The registered name of the `try_get_websocket_request` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_TRY_GET_WEBSOCKET_REQUEST: &str =
    "try_get_websocket_request";

/// The registered name of the `is_ws_upgrade_type` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_IS_WS_UPGRADE_TYPE: &str = "is_ws_upgrade_type";

/// The registered name of the `is_h2c_upgrade_type` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_IS_H2C_UPGRADE_TYPE: &str = "is_h2c_upgrade_type";

/// The registered name of the `is_tls_upgrade_type` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_IS_TLS_UPGRADE_TYPE: &str = "is_tls_upgrade_type";

/// The registered name of the `is_unknown_upgrade_type` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_IS_UNKNOWN_UPGRADE_TYPE: &str = "is_unknown_upgrade_type";

/// The registered name of the `is_http0_9_version` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_IS_HTTP0_9_VERSION: &str = "is_http0_9_version";

/// The registered name of the `is_http1_0_version` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_IS_HTTP1_0_VERSION: &str = "is_http1_0_version";

/// The registered name of the `is_http1_1_version` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_IS_HTTP1_1_VERSION: &str = "is_http1_1_version";

/// The registered name of the `is_http2_version` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_IS_HTTP2_VERSION: &str = "is_http2_version";

/// The registered name of the `is_http3_version` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_IS_HTTP3_VERSION: &str = "is_http3_version";

/// The registered name of the `is_http1_1_or_higher_version` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_IS_HTTP1_1_OR_HIGHER_VERSION: &str =
    "is_http1_1_or_higher_version";

/// The registered name of the `is_http_version` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_IS_HTTP_VERSION: &str = "is_http_version";

/// The registered name of the `is_unknown_version` injectable macro.
pub(crate) const INJECTABLE_MACRO_NAME_IS_UNKNOWN_VERSION: &str = "is_unknown_version";
