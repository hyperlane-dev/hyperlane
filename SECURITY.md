# Vulnerability coverage matrix

Every row names a bug class that has been exploited in a published HTTP
framework, the behaviour this project now has, and the test that proves it.
The test column names a function that exists in this tree; run it with
`cargo test -p <crate> <test name>`.

## Server request parsing (HTTP/1.1)

Crate `http-type`, parser `type/src/request/impl.rs`, entry point
`Stream::try_get_http_request`.

| Bug class | Behaviour now | Test |
| --- | --- | --- |
| CL.TE request smuggling | `Transfer-Encoding` on a request is refused, so no request body is framed by a coding this server does not implement | `type/tests/stream/fn.rs::test_http_request_rejects_transfer_encoding` |
| TE.CL request smuggling | A repeated `Content-Length` is refused instead of the last one winning | `type/tests/stream/fn.rs::test_http_request_rejects_duplicate_content_length` |
| Conflicting length list | `Content-Length: 5, 5` is refused | `type/tests/stream/fn.rs::test_http_request_rejects_content_length_list` |
| Length parse confusion | `Content-Length` must be decimal digits; `5x`, `+5`, `-5` are refused | `type/tests/stream/fn.rs::test_http_request_rejects_non_decimal_content_length` |
| Length integer overflow | A length beyond `usize` is refused rather than wrapped | `type/tests/stream/fn.rs::test_http_request_rejects_content_length_overflow` |
| Header folding (obs-fold) | A header line that starts with whitespace is refused | `type/tests/stream/fn.rs::test_http_request_rejects_folded_headers` |
| Whitespace before the colon | `Foo : bar` is refused, so a proxy and an origin cannot read different field names | `type/tests/stream/fn.rs::test_http_request_rejects_whitespace_before_colon` |
| Header value smuggling | A control byte in a header value is refused | `type/tests/stream/fn.rs::test_http_request_rejects_control_bytes_in_header_values` |
| Leading and trailing control bytes | The same check covers the edges of a value, which a naive trim would hide | `type/tests/stream/fn.rs::test_http_request_rejects_control_bytes_at_value_edges` |
| Malformed field name | A field name that is not a token is refused | `type/tests/stream/fn.rs::test_http_request_rejects_malformed_header_name` |
| Duplicate `Host` | A second `Host` is refused | `type/tests/stream/fn.rs::test_http_request_rejects_duplicate_host` |
| Missing `Host` on HTTP/1.1 | A request without `Host` is refused | `type/tests/stream/fn.rs::test_http_request_rejects_http_1_1_without_host` |
| Missing `Host` after keep-alive | The rule is applied to every request on a reused connection | `type/tests/stream/fn.rs::test_http_request_rejects_missing_host_after_keep_alive` |
| Version-dependent `Host` rule | HTTP/1.0 without `Host` is still accepted, so legacy clients keep working | `type/tests/stream/fn.rs::test_http_request_accepts_http_1_0_without_host` |
| Request line field count | A request line with more or fewer than three fields is refused | `type/tests/stream/fn.rs::test_http_request_rejects_request_line_field_count` |
| Unknown HTTP version | `HTTP/9.9` is refused instead of being treated as an unknown version | `type/tests/stream/fn.rs::test_http_request_rejects_unknown_version` |
| Unterminated header section | A head that ends without a blank line is refused | `type/tests/stream/fn.rs::test_http_request_rejects_unterminated_header_section` |
| Pipelines with mixed versions | The `Host` rule follows the version of each request on a pipeline | `type/tests/stream/fn.rs::test_http_request_requires_host_after_http_1_0_pipeline`, `type/tests/stream/fn.rs::test_http_request_allows_http_1_0_after_http_1_1_pipeline` |

## Client request construction

Crate `http-request`, builder `request/src/request/http_request/impl.rs`, entry
points `HttpRequest::send` and `HttpRequest::send_async`.

| Bug class | Behaviour now | Test |
| --- | --- | --- |
| Response splitting through a request header | A header value carrying CR or LF is refused before a connection is opened | `request/tests/request_api/fn.rs::client_rejects_control_bytes_in_header_values_before_sending`, `request/tests/request_api/fn.rs::client_header_injection_is_rejected_without_opening_sync_or_async_connections` |
| Header name injection | A header name that is not a token is refused | `request/tests/request_api/fn.rs::client_rejects_invalid_header_names_before_sending` |
| Request line injection | A request target carrying a control byte or a space is refused | `request/tests/request_api/fn.rs::client_rejects_request_target_controls_and_spaces` |
| Non-HTTP scheme | A scheme other than `http` or `https` is refused before a connection is attempted | `request/tests/request_api/fn.rs::client_rejects_non_http_or_missing_schemes_before_connecting` |
| Header-unsafe host | A host that is empty or carries a control byte is refused | `request/tests/request_api/fn.rs::client_rejects_missing_or_header_unsafe_hosts_before_connecting` |
| Userinfo confusion | `user:pass@host` never reaches the `Host` header or the request target | `request/tests/request_api/fn.rs::client_userinfo_never_enters_the_host_header_or_request_target` |
| Valid requests still work | Tokens, tabs and case-insensitive defaults are accepted | `request/tests/request_api/fn.rs::client_accepts_header_tokens_tabs_and_case_insensitive_defaults` |

## Client response parsing

Crate `http-request`, parser `request/src/request/parser/fn.rs`.

| Bug class | Behaviour now | Test |
| --- | --- | --- |
| Prefixed field name collision | `X-Content-Length` is not read as `Content-Length`, and a field in the body is not read as a header | `request/tests/parser/fn.rs::response_header_lookup_ignores_prefixed_names_and_body_fields` |
| Length parse confusion | A `Content-Length` that is not decimal digits is refused | `request/tests/parser/fn.rs::response_content_length_rejects_non_decimal_values` |
| Length integer overflow | A length beyond `usize` is refused rather than wrapped | `request/tests/parser/fn.rs::response_content_length_rejects_overflow_without_panicking` |
| TE.CL response smuggling | A response carrying both `Transfer-Encoding` and `Content-Length` is refused | `request/tests/parser/fn.rs::response_framing_rejects_transfer_encoding_with_content_length` |
| Unsupported transfer coding | A `Transfer-Encoding` that is not `chunked` is refused rather than treated as identity | `request/tests/parser/fn.rs::response_framing_rejects_unsupported_transfer_codings` |
| Duplicate framing fields | A repeated `Content-Length` or `Transfer-Encoding` is refused | `request/tests/parser/fn.rs::response_framing_rejects_duplicate_length_and_transfer_fields` |
| Response header smuggling | A malformed field name or a control byte in a response header is refused | `request/tests/parser/fn.rs::response_headers_reject_invalid_names_and_control_values` |
| Chunk size overflow | A chunk size that is not hexadecimal or exceeds the bound is refused | `request/tests/parser/fn.rs::chunked_body_rejects_invalid_or_excessive_chunk_sizes` |
| Chunk framing confusion | A chunk that does not end at its declared boundary, or a missing final delimiter, is refused | `request/tests/parser/fn.rs::chunked_body_requires_data_and_final_trailer_delimiters` |
| Redirect target confusion | The redirect target is read from an exact `Location` field | `request/tests/parser/fn.rs::response_redirect_uses_exact_location_header` |
| Framing errors reach the caller | A framing error is reported by both the synchronous and the asynchronous send | `request/tests/request_api/fn.rs::client_propagates_response_framing_errors_through_sync_and_async_sends` |
| Valid responses still work | A fragmented fixed-length response and a chunked response are both accepted | `request/tests/request_api/fn.rs::client_accepts_valid_fragmented_fixed_length_and_chunked_responses` |

## HTTP/2 and HTTP/3 request head

Crate `hyperlane-core`, `core/src/protocol/request/fn.rs`, entry point
`build_protocol_request`.

| Bug class | Behaviour now | Test |
| --- | --- | --- |
| `:authority` and `Host` disagreement | A request whose `:authority` names a different host than the `Host` header is refused | `core/tests/framing/fn.rs::an_authority_that_disagrees_with_the_host_header_is_rejected` |
| The same confusion over the wire | A real HTTP/2 request carrying a mismatched `Host` header never reaches the route | `core/tests/protocol/http2/assert.rs::http2_rejects_a_host_header_that_disagrees_with_the_authority` |
| Authority userinfo | `:authority` carrying `user@host` is refused | `core/tests/framing/fn.rs::an_authority_that_carries_userinfo_is_rejected` |
| `Host` header userinfo | A `Host` header carrying `user@host` is refused | `core/tests/framing/fn.rs::a_host_header_that_carries_userinfo_is_rejected` |
| Duplicate `Host` in HTTP/2 | A second `Host` header is refused | `core/tests/framing/fn.rs::two_host_headers_are_rejected` |
| Port confusion | An authority and a `Host` that name different ports are refused | `core/tests/framing/fn.rs::an_authority_and_a_host_header_that_name_different_ports_are_rejected` |
| Normalization done wrong | A host that differs only by case, or only by the default port, is accepted | `core/tests/framing/fn.rs::an_authority_that_differs_from_the_host_header_only_by_case_is_accepted`, `core/tests/framing/fn.rs::a_default_port_matches_an_authority_that_carries_no_port` |
| Legitimate requests still work | A request with no `Host` header beside the pseudo header is accepted | `core/tests/framing/fn.rs::a_request_without_a_host_header_is_accepted` |
| A matching `Host` still works | A real HTTP/2 request whose `Host` matches the authority reaches the route | `core/tests/protocol/http2/assert.rs::http2_accepts_a_host_header_that_agrees_with_the_authority` |

## Cookie parsing and serialization

Crate `http-type`, `type/src/cookie/impl.rs`, entry points
`CookieBuilder::build`, `CookieBuilder::try_build` and `Cookie::parse`.

| Bug class | Behaviour now | Test |
| --- | --- | --- |
| Response splitting through a cookie | A cookie value carrying a control byte is refused, so it cannot end the header line | `type/tests/cookie/fn.rs::test_cookie_builder_refuses_a_value_that_carries_a_control_byte` |
| Cookie attribute injection | A cookie value carrying a semicolon is refused, so it cannot start another attribute | `type/tests/cookie/fn.rs::test_cookie_builder_refuses_a_value_that_would_end_the_cookie_pair` |
| Cookie name injection | A cookie name that is not a token is refused | `type/tests/cookie/fn.rs::test_cookie_builder_refuses_a_name_that_is_not_a_token` |
| Attribute injection through a path | A path carrying a control byte is refused | `type/tests/cookie/fn.rs::test_cookie_builder_refuses_an_attribute_that_carries_a_control_byte` |
| Attribute injection through a domain | A domain carrying a semicolon is refused | `type/tests/cookie/fn.rs::test_cookie_builder_refuses_an_attribute_that_would_start_another_attribute` |
| Cookie parsing of a hostile `Set-Cookie` | A `Set-Cookie` whose name is not a token, or whose value carries a control byte, is not accepted | `type/tests/cookie/fn.rs::test_cookie_builder_parse_skips_a_name_that_is_not_a_token`, `type/tests/cookie/fn.rs::test_cookie_builder_parse_skips_a_value_that_carries_a_control_byte` |
| Cookie parsing of a hostile attribute | A `Set-Cookie` attribute carrying a control byte is skipped | `type/tests/cookie/fn.rs::test_cookie_builder_parse_skips_an_attribute_that_carries_a_control_byte` |
| Cookie request header smuggling | A `Cookie` request header pair carrying a control byte is skipped | `type/tests/cookie/fn.rs::test_cookie_parse_skips_a_pair_that_carries_a_control_byte` |
| Cookie request header name smuggling | A `Cookie` request header pair whose name is not a token is skipped | `type/tests/cookie/fn.rs::test_cookie_parse_skips_a_pair_whose_name_is_not_a_token` |
| Valid cookies still work | A cookie whose parts all satisfy the grammar is built unchanged | `type/tests/cookie/fn.rs::test_cookie_builder_builds_a_cookie_whose_parts_are_all_valid` |

## Not covered

- WebSocket frame parsing has its own length bounds in
  `type/src/websocket_frame/impl.rs` and is not driven by the HTTP header
  parser, so the header rules above do not reach it.
- HTTP/2 and HTTP/3 stream-level flow control and the HTTP/1.1 upgrade dance
  are outside the request head rules covered here.
