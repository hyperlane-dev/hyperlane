use crate::*;

#[cfg(test)]
async fn assert_panic_message_contains<F, Fut>(future_factory: F, expected_msg: &str)
where
    F: Fn() -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    let result: ResultJoinError<_> = spawn(future_factory()).await;
    assert!(
        result.is_err(),
        "Expected panic, but task completed successfully"
    );
    let join_err: JoinError = result.unwrap_err();
    if !join_err.is_panic() {
        panic!("Task failed but was not a panic");
    }
    let panic_payload: Box<dyn Any + Send> = join_err.into_panic();
    let panic_msg: &str = if let Some(s) = panic_payload.downcast_ref::<&str>() {
        *s
    } else if let Some(s) = panic_payload.downcast_ref::<String>() {
        s.as_str()
    } else {
        "Unknown panic type"
    };
    assert!(
        panic_msg.contains(expected_msg),
        "Expected panic message to contain: '{}', but got: '{}'",
        expected_msg,
        panic_msg
    );
}

struct TestRoute;

impl ServerHook for TestRoute {
    async fn new(_ctx: Context) -> Self {
        Self
    }

    async fn handle(self, _ctx: Context) {}
}

#[tokio::test]
async fn empty_route() {
    assert_panic_message_contains(
        || async {
            let _server: &Server = Server::new().await.route::<TestRoute>(EMPTY_STR).await;
        },
        &RouteError::EmptyPattern.to_string(),
    )
    .await;
}

#[tokio::test]
async fn duplicate_route() {
    assert_panic_message_contains(
        || async {
            let _server: &Server = Server::new()
                .await
                .route::<TestRoute>(ROOT_PATH)
                .await
                .route::<TestRoute>(ROOT_PATH)
                .await;
        },
        &RouteError::DuplicatePattern(ROOT_PATH.to_string()).to_string(),
    )
    .await;
}
