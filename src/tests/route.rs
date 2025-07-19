use crate::*;

#[tokio::test]
async fn test_empty_route() {
    assert_panic_message_contains(
        || async {
            let _server: ServerBuilder = ServerBuilder::new().route("", |_| async move {});
        },
        "Route pattern cannot be empty",
    )
    .await;
}

#[tokio::test]
async fn test_duplicate_route() {
    assert_panic_message_contains(
        || async {
            let _server: ServerBuilder = ServerBuilder::new()
                .route("/", |_| async move {})
                .route("/", |_| async move {});
        },
        "Route pattern already exists: /",
    )
    .await;
}

// 注释掉这些测试，因为新的无锁架构不支持运行时配置修改
// 这些功能需要在构建时通过 ServerBuilder 配置

/*
#[tokio::test]
async fn test_disable_http_hook_empty_route() {
    // 这个功能现在需要在 ServerBuilder 中实现
}

#[tokio::test]
async fn test_disable_http_hook_duplicate_route() {
    // 这个功能现在需要在 ServerBuilder 中实现
}

#[tokio::test]
async fn test_disable_ws_hook_empty_route() {
    // 这个功能现在需要在 ServerBuilder 中实现
}

#[tokio::test]
async fn test_disable_ws_hook_duplicate_route() {
    // 这个功能现在需要在 ServerBuilder 中实现
}
*/
