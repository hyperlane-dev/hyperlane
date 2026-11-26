use super::*;

#[tokio::test]
async fn test_rpc_router_single_batch_notification_and_dos_bounds() {
    let mut router: RpcRouter = RpcRouter::new();
    router.register("add", |params: Option<Value>| async move {
        let Some(Value::Array(arr)) = params else {
            return Err(RpcError::invalid_params(None));
        };
        let a: i64 = arr
            .first()
            .and_then(Value::as_i64)
            .ok_or_else(|| RpcError::invalid_params(None))?;
        let b: i64 = arr
            .get(1)
            .and_then(Value::as_i64)
            .ok_or_else(|| RpcError::invalid_params(None))?;
        Ok(json!(a + b))
    });
    assert!(router.has_method("add"));
    let mut cfg: RpcConfig = RpcConfig::default();
    cfg.set_max_batch_size(2);
    let single_raw: &[u8] = br#"{"jsonrpc":"2.0","method":"add","params":[3,4],"id":1}"#;
    let single_out: Vec<u8> = router.dispatch_bytes(single_raw, &cfg).await.unwrap();
    let single_resp: RpcResponse = from_slice(&single_out).unwrap();
    assert_eq!(single_resp.get_result(), Some(json!(7)));
    assert_eq!(single_resp.get_id(), &RpcId::Number(1));
    let notif_raw: &[u8] = br#"{"jsonrpc":"2.0","method":"add","params":[1,2]}"#;
    assert!(router.dispatch_bytes(notif_raw, &cfg).await.is_none());
    let batch_raw: &[u8] = br#"[
        {"jsonrpc":"2.0","method":"add","params":[10,5],"id":"a"},
        {"jsonrpc":"2.0","method":"missing","id":"b"}
    ]"#;
    let batch_out: Vec<u8> = router.dispatch_bytes(batch_raw, &cfg).await.unwrap();
    let batch_resps: Vec<RpcResponse> = from_slice(&batch_out).unwrap();
    assert_eq!(batch_resps.len(), 2usize);
    assert_eq!(batch_resps[0].get_result(), Some(json!(15)));
    assert_eq!(
        batch_resps[1].get_error().map(|e: RpcError| e.get_code()),
        Some(RPC_METHOD_NOT_FOUND)
    );
    let huge_batch: &[u8] = br#"[
        {"jsonrpc":"2.0","method":"add","params":[1,1],"id":1},
        {"jsonrpc":"2.0","method":"add","params":[1,1],"id":2},
        {"jsonrpc":"2.0","method":"add","params":[1,1],"id":3}
    ]"#;
    let huge_out: Vec<u8> = router.dispatch_bytes(huge_batch, &cfg).await.unwrap();
    let huge_resp: RpcResponse = from_slice(&huge_out).unwrap();
    assert_eq!(
        huge_resp.get_error().map(|e: RpcError| e.get_code()),
        Some(RPC_INVALID_REQUEST)
    );
    let bad_json_out: Vec<u8> = router.dispatch_bytes(b"{broken", &cfg).await.unwrap();
    let bad_json_resp: RpcResponse = from_slice(&bad_json_out).unwrap();
    assert_eq!(
        bad_json_resp.get_error().map(|e: RpcError| e.get_code()),
        Some(RPC_PARSE_ERROR)
    );
    let empty_batch_out: Vec<u8> = router.dispatch_bytes(b"[]", &cfg).await.unwrap();
    let empty_batch_resp: RpcResponse = from_slice(&empty_batch_out).unwrap();
    assert_eq!(
        empty_batch_resp.get_error().map(|e: RpcError| e.get_code()),
        Some(RPC_INVALID_REQUEST)
    );
}
