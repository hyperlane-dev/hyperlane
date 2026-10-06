use super::*;

/// Static array of all injectable macros.
///
/// This array contains all the macro handlers that can be injected using the `inject` macro.
pub(crate) static INJECTABLE_MACROS: &[InjectableMacro] = &[
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_CLOSED,
        handler: Handler::NoAttrPosition(closed_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_FILTER,
        handler: Handler::WithAttrPosition(filter_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_TRY_FLUSH,
        handler: Handler::NoAttrPosition(try_flush_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_FLUSH,
        handler: Handler::NoAttrPosition(flush_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_TASK_PANIC,
        handler: Handler::WithAttr(task_panic_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_REQUEST_ERROR,
        handler: Handler::WithAttr(request_error_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_PROLOGUE_HOOKS,
        handler: Handler::WithAttrPosition(prologue_hooks_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_EPILOGUE_HOOKS,
        handler: Handler::WithAttrPosition(epilogue_hooks_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_HOST,
        handler: Handler::WithAttrPosition(host_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_REJECT_HOST,
        handler: Handler::WithAttrPosition(reject_host_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_HYPERLANE,
        handler: Handler::WithAttr(hyperlane_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_METHODS,
        handler: Handler::WithAttrPosition(methods_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_IS_GET_METHOD,
        handler: Handler::NoAttrPosition(is_get_method_handler),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_IS_POST_METHOD,
        handler: Handler::NoAttrPosition(is_post_method_handler),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_IS_PUT_METHOD,
        handler: Handler::NoAttrPosition(is_put_method_handler),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_IS_DELETE_METHOD,
        handler: Handler::NoAttrPosition(is_delete_method_handler),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_IS_PATCH_METHOD,
        handler: Handler::NoAttrPosition(is_patch_method_handler),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_IS_HEAD_METHOD,
        handler: Handler::NoAttrPosition(is_head_method_handler),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_IS_OPTIONS_METHOD,
        handler: Handler::NoAttrPosition(is_options_method_handler),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_IS_CONNECT_METHOD,
        handler: Handler::NoAttrPosition(is_connect_method_handler),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_IS_TRACE_METHOD,
        handler: Handler::NoAttrPosition(is_trace_method_handler),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_IS_UNKNOWN_METHOD,
        handler: Handler::NoAttrPosition(is_unknown_method_handler),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_REFERER,
        handler: Handler::WithAttrPosition(referer_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_REJECT_REFERER,
        handler: Handler::WithAttrPosition(reject_referer_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_REJECT,
        handler: Handler::WithAttrPosition(reject_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_REQUEST_BODY,
        handler: Handler::WithAttrPosition(request_body_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_REQUEST_BODY_JSON_RESULT,
        handler: Handler::WithAttrPosition(request_body_json_result_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_REQUEST_BODY_JSON,
        handler: Handler::WithAttrPosition(request_body_json_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_TRY_GET_ATTRIBUTE,
        handler: Handler::WithAttrPosition(try_get_attribute_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_ATTRIBUTE,
        handler: Handler::WithAttrPosition(attribute_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_ATTRIBUTES,
        handler: Handler::WithAttrPosition(attributes_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_TRY_GET_TASK_PANIC_DATA,
        handler: Handler::WithAttrPosition(try_get_task_panic_data_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_TASK_PANIC_DATA,
        handler: Handler::WithAttrPosition(task_panic_data_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_TRY_GET_REQUEST_ERROR_DATA,
        handler: Handler::WithAttrPosition(try_get_request_error_data_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_REQUEST_ERROR_DATA,
        handler: Handler::WithAttrPosition(request_error_data_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_TRY_GET_ROUTE_PARAM,
        handler: Handler::WithAttrPosition(try_get_route_param_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_ROUTE_PARAM,
        handler: Handler::WithAttrPosition(route_param_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_ROUTE_PARAMS,
        handler: Handler::WithAttrPosition(route_params_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_TRY_GET_REQUEST_QUERY,
        handler: Handler::WithAttrPosition(try_get_request_query_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_REQUEST_QUERY,
        handler: Handler::WithAttrPosition(request_query_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_REQUEST_QUERYS,
        handler: Handler::WithAttrPosition(request_querys_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_TRY_GET_REQUEST_HEADER,
        handler: Handler::WithAttrPosition(try_get_request_header_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_REQUEST_HEADER,
        handler: Handler::WithAttrPosition(request_header_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_REQUEST_HEADERS,
        handler: Handler::WithAttrPosition(request_headers_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_TRY_GET_REQUEST_COOKIE,
        handler: Handler::WithAttrPosition(try_get_request_cookie_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_REQUEST_COOKIE,
        handler: Handler::WithAttrPosition(request_cookie_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_REQUEST_COOKIES,
        handler: Handler::WithAttrPosition(request_cookies_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_REQUEST_VERSION,
        handler: Handler::WithAttrPosition(request_version_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_REQUEST_PATH,
        handler: Handler::WithAttrPosition(request_path_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_REQUEST_MIDDLEWARE,
        handler: Handler::WithAttr(request_middleware_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_RESPONSE_STATUS_CODE,
        handler: Handler::WithAttrPosition(response_status_code_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_RESPONSE_REASON_PHRASE,
        handler: Handler::WithAttrPosition(response_reason_phrase_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_RESPONSE_HEADER,
        handler: Handler::WithAttrPosition(response_header_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_RESPONSE_BODY,
        handler: Handler::WithAttrPosition(response_body_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_CLEAR_RESPONSE_HEADERS,
        handler: Handler::NoAttrPosition(clear_response_headers_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_RESPONSE_VERSION,
        handler: Handler::WithAttrPosition(response_version_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_RESPONSE_MIDDLEWARE,
        handler: Handler::WithAttr(response_middleware_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_ROUTE,
        handler: Handler::WithAttr(route_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_TRY_SEND,
        handler: Handler::WithAttrPosition(try_send_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_SEND,
        handler: Handler::WithAttrPosition(send_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_TRY_GET_HTTP_REQUEST,
        handler: Handler::WithAttr(try_get_http_request_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_TRY_GET_WEBSOCKET_REQUEST,
        handler: Handler::WithAttr(try_get_websocket_request_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_IS_WS_UPGRADE_TYPE,
        handler: Handler::NoAttrPosition(is_ws_upgrade_type_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_IS_H2C_UPGRADE_TYPE,
        handler: Handler::NoAttrPosition(is_h2c_upgrade_type_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_IS_TLS_UPGRADE_TYPE,
        handler: Handler::NoAttrPosition(is_tls_upgrade_type_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_IS_UNKNOWN_UPGRADE_TYPE,
        handler: Handler::NoAttrPosition(is_unknown_upgrade_type_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_IS_HTTP0_9_VERSION,
        handler: Handler::NoAttrPosition(is_http0_9_version_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_IS_HTTP1_0_VERSION,
        handler: Handler::NoAttrPosition(is_http1_0_version_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_IS_HTTP1_1_VERSION,
        handler: Handler::NoAttrPosition(is_http1_1_version_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_IS_HTTP2_VERSION,
        handler: Handler::NoAttrPosition(is_http2_version_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_IS_HTTP3_VERSION,
        handler: Handler::NoAttrPosition(is_http3_version_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_IS_HTTP1_1_OR_HIGHER_VERSION,
        handler: Handler::NoAttrPosition(is_http1_1_or_higher_version_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_IS_HTTP_VERSION,
        handler: Handler::NoAttrPosition(is_http_version_macro),
    },
    InjectableMacro {
        name: INJECTABLE_MACRO_NAME_IS_UNKNOWN_VERSION,
        handler: Handler::NoAttrPosition(is_unknown_version_macro),
    },
];
