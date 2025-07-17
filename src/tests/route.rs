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
async fn test_disable_http_hook_empty_route() {
    assert_panic_message_contains(
        || async {
            let server: Server = Server::new();
            server.disable_http_hook("").await;
        },
        "Route pattern cannot be empty",
    )
    .await;
}

#[tokio::test]
async fn test_disable_http_hook_duplicate_route() {
    assert_panic_message_contains(
        || async {
            let server: Server = Server::new();
            server.disable_http_hook("/").await;
            server.disable_http_hook("/").await;
        },
        "Route pattern already exists: /",
    )
    .await;
}

#[tokio::test]
async fn test_disable_ws_hook_empty_route() {
    assert_panic_message_contains(
        || async {
            let server: Server = Server::new();
            server.disable_ws_hook("").await;
        },
        "Route pattern cannot be empty",
    )
    .await;
}

#[tokio::test]
async fn test_disable_ws_hook_duplicate_route() {
    assert_panic_message_contains(
        || async {
            let server: Server = Server::new();
            server.disable_ws_hook("/").await;
            server.disable_ws_hook("/").await;
        },
        "Route pattern already exists: /",
    )
    .await;
}
