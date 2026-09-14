use super::*;

/// Static array of all injectable macros.
///
/// This array contains all the macro handlers that can be injected using the `inject` macro.
pub(crate) static INJECTABLE_MACROS: &[InjectableMacro] = &[
    InjectableMacro {
        name: "closed",
        handler: Handler::NoAttrPosition(closed_macro),
    },
    InjectableMacro {
        name: "filter",
        handler: Handler::WithAttrPosition(filter_macro),
    },
    InjectableMacro {
        name: "try_flush",
        handler: Handler::NoAttrPosition(try_flush_macro),
    },
    InjectableMacro {
        name: "flush",
        handler: Handler::NoAttrPosition(flush_macro),
    },
    InjectableMacro {
        name: "task_panic",
        handler: Handler::WithAttr(task_panic_macro),
    },
    InjectableMacro {
        name: "request_error",
        handler: Handler::WithAttr(request_error_macro),
    },
    InjectableMacro {
        name: "prologue_hooks",
        handler: Handler::WithAttrPosition(prologue_hooks_macro),
    },
    InjectableMacro {
        name: "epilogue_hooks",
        handler: Handler::WithAttrPosition(epilogue_hooks_macro),
    },
    InjectableMacro {
        name: "host",
        handler: Handler::WithAttrPosition(host_macro),
    },
    InjectableMacro {
        name: "reject_host",
        handler: Handler::WithAttrPosition(reject_host_macro),
    },
    InjectableMacro {
        name: "hyperlane",
        handler: Handler::WithAttr(hyperlane_macro),
    },
    InjectableMacro {
        name: "methods",
        handler: Handler::WithAttrPosition(methods_macro),
    },
    InjectableMacro {
        name: "is_get_method",
        handler: Handler::NoAttrPosition(is_get_method_handler),
    },
    InjectableMacro {
        name: "is_post_method",
        handler: Handler::NoAttrPosition(is_post_method_handler),
    },
    InjectableMacro {
        name: "is_put_method",
        handler: Handler::NoAttrPosition(is_put_method_handler),
    },
    InjectableMacro {
        name: "is_delete_method",
        handler: Handler::NoAttrPosition(is_delete_method_handler),
    },
    InjectableMacro {
        name: "is_patch_method",
        handler: Handler::NoAttrPosition(is_patch_method_handler),
    },
    InjectableMacro {
        name: "is_head_method",
        handler: Handler::NoAttrPosition(is_head_method_handler),
    },
    InjectableMacro {
        name: "is_options_method",
        handler: Handler::NoAttrPosition(is_options_method_handler),
    },
    InjectableMacro {
        name: "is_connect_method",
        handler: Handler::NoAttrPosition(is_connect_method_handler),
    },
    InjectableMacro {
        name: "is_trace_method",
        handler: Handler::NoAttrPosition(is_trace_method_handler),
    },
    InjectableMacro {
        name: "is_unknown_method",
        handler: Handler::NoAttrPosition(is_unknown_method_handler),
    },
    InjectableMacro {
        name: "referer",
        handler: Handler::WithAttrPosition(referer_macro),
    },
    InjectableMacro {
        name: "reject_referer",
        handler: Handler::WithAttrPosition(reject_referer_macro),
    },
    InjectableMacro {
        name: "reject",
        handler: Handler::WithAttrPosition(reject_macro),
    },
    InjectableMacro {
        name: "request_body",
        handler: Handler::WithAttrPosition(request_body_macro),
    },
    InjectableMacro {
        name: "request_body_json_result",
        handler: Handler::WithAttrPosition(request_body_json_result_macro),
    },
    InjectableMacro {
        name: "request_body_json",
        handler: Handler::WithAttrPosition(request_body_json_macro),
    },
    InjectableMacro {
        name: "try_get_attribute",
        handler: Handler::WithAttrPosition(try_get_attribute_macro),
    },
    InjectableMacro {
        name: "attribute",
        handler: Handler::WithAttrPosition(attribute_macro),
    },
    InjectableMacro {
        name: "attributes",
        handler: Handler::WithAttrPosition(attributes_macro),
    },
    InjectableMacro {
        name: "try_get_task_panic_data",
        handler: Handler::WithAttrPosition(try_get_task_panic_data_macro),
    },
    InjectableMacro {
        name: "task_panic_data",
        handler: Handler::WithAttrPosition(task_panic_data_macro),
    },
    InjectableMacro {
        name: "try_get_request_error_data",
        handler: Handler::WithAttrPosition(try_get_request_error_data_macro),
    },
    InjectableMacro {
        name: "request_error_data",
        handler: Handler::WithAttrPosition(request_error_data_macro),
    },
    InjectableMacro {
        name: "try_get_route_param",
        handler: Handler::WithAttrPosition(try_get_route_param_macro),
    },
    InjectableMacro {
        name: "route_param",
        handler: Handler::WithAttrPosition(route_param_macro),
    },
    InjectableMacro {
        name: "route_params",
        handler: Handler::WithAttrPosition(route_params_macro),
    },
    InjectableMacro {
        name: "try_get_request_query",
        handler: Handler::WithAttrPosition(try_get_request_query_macro),
    },
    InjectableMacro {
        name: "request_query",
        handler: Handler::WithAttrPosition(request_query_macro),
    },
    InjectableMacro {
        name: "request_querys",
        handler: Handler::WithAttrPosition(request_querys_macro),
    },
    InjectableMacro {
        name: "try_get_request_header",
        handler: Handler::WithAttrPosition(try_get_request_header_macro),
    },
    InjectableMacro {
        name: "request_header",
        handler: Handler::WithAttrPosition(request_header_macro),
    },
    InjectableMacro {
        name: "request_headers",
        handler: Handler::WithAttrPosition(request_headers_macro),
    },
    InjectableMacro {
        name: "try_get_request_cookie",
        handler: Handler::WithAttrPosition(try_get_request_cookie_macro),
    },
    InjectableMacro {
        name: "request_cookie",
        handler: Handler::WithAttrPosition(request_cookie_macro),
    },
    InjectableMacro {
        name: "request_cookies",
        handler: Handler::WithAttrPosition(request_cookies_macro),
    },
    InjectableMacro {
        name: "request_version",
        handler: Handler::WithAttrPosition(request_version_macro),
    },
    InjectableMacro {
        name: "request_path",
        handler: Handler::WithAttrPosition(request_path_macro),
    },
    InjectableMacro {
        name: "request_middleware",
        handler: Handler::WithAttr(request_middleware_macro),
    },
    InjectableMacro {
        name: "response_status_code",
        handler: Handler::WithAttrPosition(response_status_code_macro),
    },
    InjectableMacro {
        name: "response_reason_phrase",
        handler: Handler::WithAttrPosition(response_reason_phrase_macro),
    },
    InjectableMacro {
        name: "response_header",
        handler: Handler::WithAttrPosition(response_header_macro),
    },
    InjectableMacro {
        name: "response_body",
        handler: Handler::WithAttrPosition(response_body_macro),
    },
    InjectableMacro {
        name: "clear_response_headers",
        handler: Handler::NoAttrPosition(clear_response_headers_macro),
    },
    InjectableMacro {
        name: "response_version",
        handler: Handler::WithAttrPosition(response_version_macro),
    },
    InjectableMacro {
        name: "response_middleware",
        handler: Handler::WithAttr(response_middleware_macro),
    },
    InjectableMacro {
        name: "route",
        handler: Handler::WithAttr(route_macro),
    },
    InjectableMacro {
        name: "try_send",
        handler: Handler::WithAttrPosition(try_send_macro),
    },
    InjectableMacro {
        name: "send",
        handler: Handler::WithAttrPosition(send_macro),
    },
    InjectableMacro {
        name: "try_get_http_request",
        handler: Handler::WithAttr(try_get_http_request_macro),
    },
    InjectableMacro {
        name: "try_get_websocket_request",
        handler: Handler::WithAttr(try_get_websocket_request_macro),
    },
    InjectableMacro {
        name: "is_ws_upgrade_type",
        handler: Handler::NoAttrPosition(is_ws_upgrade_type_macro),
    },
    InjectableMacro {
        name: "is_h2c_upgrade_type",
        handler: Handler::NoAttrPosition(is_h2c_upgrade_type_macro),
    },
    InjectableMacro {
        name: "is_tls_upgrade_type",
        handler: Handler::NoAttrPosition(is_tls_upgrade_type_macro),
    },
    InjectableMacro {
        name: "is_unknown_upgrade_type",
        handler: Handler::NoAttrPosition(is_unknown_upgrade_type_macro),
    },
    InjectableMacro {
        name: "is_http0_9_version",
        handler: Handler::NoAttrPosition(is_http0_9_version_macro),
    },
    InjectableMacro {
        name: "is_http1_0_version",
        handler: Handler::NoAttrPosition(is_http1_0_version_macro),
    },
    InjectableMacro {
        name: "is_http1_1_version",
        handler: Handler::NoAttrPosition(is_http1_1_version_macro),
    },
    InjectableMacro {
        name: "is_http2_version",
        handler: Handler::NoAttrPosition(is_http2_version_macro),
    },
    InjectableMacro {
        name: "is_http3_version",
        handler: Handler::NoAttrPosition(is_http3_version_macro),
    },
    InjectableMacro {
        name: "is_http1_1_or_higher_version",
        handler: Handler::NoAttrPosition(is_http1_1_or_higher_version_macro),
    },
    InjectableMacro {
        name: "is_http_version",
        handler: Handler::NoAttrPosition(is_http_version_macro),
    },
    InjectableMacro {
        name: "is_unknown_version",
        handler: Handler::NoAttrPosition(is_unknown_version_macro),
    },
];
