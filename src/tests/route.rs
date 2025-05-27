use crate::*;

#[tokio::test]
async fn test_empty_route() {
    assert_panic_message_contains(
        || async {
            let server = Server::new();
            server.route("", |_| async move {}).await;
        },
        "Route pattern cannot be empty",
    )
    .await;
}

#[tokio::test]
async fn test_duplicate_route() {
    assert_panic_message_contains(
        || async {
            let server: Server = Server::new();
            server.route("/", |_| async move {}).await;
            server.route("/", |_| async move {}).await;
        },
        "Route pattern already exists: /",
    )
    .await;
}

#[tokio::test]
async fn test_disable_inner_http_handle_empty_route() {
    assert_panic_message_contains(
        || async {
            let server: Server = Server::new();
            server.disable_inner_http_handle("").await;
        },
        "Route pattern cannot be empty",
    )
    .await;
}

#[tokio::test]
async fn test_disable_inner_http_handle_duplicate_route() {
    assert_panic_message_contains(
        || async {
            let server: Server = Server::new();
            server.disable_inner_http_handle("/").await;
            server.disable_inner_http_handle("/").await;
        },
        "Route pattern already exists: /",
    )
    .await;
}

#[tokio::test]
async fn test_disable_inner_websocket_handle_empty_route() {
    assert_panic_message_contains(
        || async {
            let server: Server = Server::new();
            server.disable_inner_websocket_handle("").await;
        },
        "Route pattern cannot be empty",
    )
    .await;
}

#[tokio::test]
async fn test_disable_inner_websocket_handle_duplicate_route() {
    assert_panic_message_contains(
        || async {
            let server: Server = Server::new();
            server.disable_inner_websocket_handle("/").await;
            server.disable_inner_websocket_handle("/").await;
        },
        "Route pattern already exists: /",
    )
    .await;
}
