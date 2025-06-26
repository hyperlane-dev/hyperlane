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
async fn test_disable_http_handler_empty_route() {
    assert_panic_message_contains(
        || async {
            let server: Server = Server::new();
            server.disable_http_handler("").await;
        },
        "Route pattern cannot be empty",
    )
    .await;
}

#[tokio::test]
async fn test_disable_http_handler_duplicate_route() {
    assert_panic_message_contains(
        || async {
            let server: Server = Server::new();
            server.disable_http_handler("/").await;
            server.disable_http_handler("/").await;
        },
        "Route pattern already exists: /",
    )
    .await;
}

#[tokio::test]
async fn test_disable_ws_handler_empty_route() {
    assert_panic_message_contains(
        || async {
            let server: Server = Server::new();
            server.disable_ws_handler("").await;
        },
        "Route pattern cannot be empty",
    )
    .await;
}

#[tokio::test]
async fn test_disable_ws_handler_duplicate_route() {
    assert_panic_message_contains(
        || async {
            let server: Server = Server::new();
            server.disable_ws_handler("/").await;
            server.disable_ws_handler("/").await;
        },
        "Route pattern already exists: /",
    )
    .await;
}
