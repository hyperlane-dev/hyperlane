use crate::*;

#[tokio::test]
async fn test_empty_route() {
    assert_panic_message_contains(
        || async {
            let _server: ServerBuilder = ServerBuilder::new().route(EMPTY_STR, |_| async move {});
        },
        &RouteError::EmptyPattern.to_string(),
    )
    .await;
}

#[tokio::test]
async fn test_duplicate_route() {
    assert_panic_message_contains(
        || async {
            let _server: ServerBuilder = ServerBuilder::new()
                .route(ROOT_PATH, |_| async move {})
                .route(ROOT_PATH, |_| async move {});
        },
        &crate::RouteError::DuplicatePattern(ROOT_PATH.to_string()).to_string(),
    )
    .await;
}
