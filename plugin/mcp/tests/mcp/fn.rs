use super::*;

#[tokio::test]
async fn test_mcp_initialize_tools_resources_prompts_sessions_and_origin_guard() {
    let mut registry: McpRegistry = McpRegistry::new();
    registry.register_tool(
        McpTool {
            name: "sum".to_owned(),
            description: "Adds two integers".to_owned(),
            input_schema: json!({"type":"object"}),
        },
        |args: Value| async move {
            let a: i64 = args.get("a").and_then(Value::as_i64).unwrap_or(0);
            let b: i64 = args.get("b").and_then(Value::as_i64).unwrap_or(0);
            Ok(McpToolResult::text(format!("{}", a + b)))
        },
    );
    registry.register_resource(
        McpResource {
            uri: "hyperlane://status".to_owned(),
            name: "status".to_owned(),
            description: "Server status".to_owned(),
            mime_type: "application/json".to_owned(),
        },
        |uri: String| async move {
            Ok(McpResourceContent {
                uri,
                mime_type: "application/json".to_owned(),
                text: "{\"healthy\":true}".to_owned(),
            })
        },
    );
    registry.register_prompt(
        McpPrompt {
            name: "greet".to_owned(),
            description: "Greeting prompt".to_owned(),
            arguments: vec![McpPromptArgument {
                name: "user".to_owned(),
                description: "Username".to_owned(),
                required: true,
            }],
        },
        |args: Value| async move {
            let user: &str = args.get("user").and_then(Value::as_str).unwrap_or("guest");
            Ok(json!({
                "messages": [{"role": "user", "content": {"type": "text", "text": format!("Hello {user}")}}]
            }))
        },
    );
    let mut cfg: McpServerConfig = McpServerConfig::default();
    cfg.set_max_sessions(1)
        .set_allowed_origins(vec!["https://trusted.example".to_owned()]);
    assert!(is_origin_allowed(
        Some("https://trusted.example"),
        cfg.get_allowed_origins()
    ));
    assert!(!is_origin_allowed(
        Some("https://evil.example"),
        cfg.get_allowed_origins()
    ));
    let init_req: &[u8] = br#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
    let init_out: McpDispatchOutcome = registry.handle_json_bytes(init_req, None, &cfg).await;
    let session_id: String = match init_out {
        McpDispatchOutcome::Response {
            body,
            new_session_id: Some(sid),
        } => {
            let v: Value = from_slice(&body).unwrap();
            assert_eq!(v["result"]["protocolVersion"], DEFAULT_MCP_PROTOCOL_VERSION);
            sid
        }
        other => panic!("unexpected outcome: {other:?}"),
    };
    assert_eq!(registry.session_count(), 1usize);
    assert_eq!(
        registry.handle_json_bytes(init_req, None, &cfg).await,
        McpDispatchOutcome::SessionLimitReached
    );
    let bad_sid_out: McpDispatchOutcome = registry
        .handle_json_bytes(
            br#"{"jsonrpc":"2.0","id":2,"method":"ping"}"#,
            Some("unknown-session"),
            &cfg,
        )
        .await;
    assert_eq!(bad_sid_out, McpDispatchOutcome::UnknownSession);
    let call_req: &[u8] =
        br#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"sum","arguments":{"a":9,"b":11}}}"#;
    let call_out: McpDispatchOutcome = registry
        .handle_json_bytes(call_req, Some(&session_id), &cfg)
        .await;
    let McpDispatchOutcome::Response {
        body: call_body, ..
    } = call_out
    else {
        panic!("expected tool response");
    };
    let call_json: Value = from_slice(&call_body).unwrap();
    assert_eq!(call_json["result"]["content"][0]["text"], "20");
    let read_req: Vec<u8> = to_vec(&json!({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "resources/read",
        "params": {
            "uri": "hyperlane://status"
        }
    }))
    .unwrap();
    let McpDispatchOutcome::Response {
        body: read_body, ..
    } = registry
        .handle_json_bytes(&read_req, Some(&session_id), &cfg)
        .await
    else {
        panic!("expected resource response");
    };
    let read_json: Value = from_slice(&read_body).unwrap();
    assert_eq!(
        read_json["result"]["contents"][0]["text"],
        "{\"healthy\":true}"
    );
    let prompt_req: &[u8] =
        br#"{"jsonrpc":"2.0","id":5,"method":"prompts/get","params":{"name":"greet","arguments":{"user":"ada"}}}"#;
    let McpDispatchOutcome::Response {
        body: prompt_body, ..
    } = registry
        .handle_json_bytes(prompt_req, Some(&session_id), &cfg)
        .await
    else {
        panic!("expected prompt response");
    };
    let prompt_json: Value = from_slice(&prompt_body).unwrap();
    assert_eq!(
        prompt_json["result"]["messages"][0]["content"]["text"],
        "Hello ada"
    );
    assert!(registry.close_session(&session_id));
    assert_eq!(registry.session_count(), 0usize);
}
