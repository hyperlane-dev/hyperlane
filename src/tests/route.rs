use crate::*;

#[cfg(test)]
async fn assert_panic_message_contains<F, Fut>(future_factory: F, expected_msg: &str)
where
    F: Fn() -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    let result: TaskJoinResult<_> = spawn(future_factory()).await;
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

#[cfg(test)]
struct TestRoute {
    data: String,
}

#[cfg(test)]
impl ServerHook for TestRoute {
    async fn new(_ctx: &Context) -> Self {
        Self {
            data: String::new(),
        }
    }

    async fn handle(mut self, _ctx: &Context) {
        self.data = String::from("test");
    }
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

#[tokio::test]
async fn get_route() {
    let server: Server = Server::new().await;
    server
        .route::<TestRoute>(ROOT_PATH)
        .await
        .route::<TestRoute>("/dynamic/{routing}")
        .await
        .route::<TestRoute>("/regex/{file:^.*$}")
        .await;
    for (key1, _value1) in server.get_route_matcher().await.get_static_route() {
        println!("get_route key: {key1}");
    }
    for (_key1, value1) in server.get_route_matcher().await.get_dynamic_route() {
        for (key2, _value2) in value1 {
            println!("get_route key: {key2}");
        }
    }
    for (_key1, value1) in server.get_route_matcher().await.get_regex_route() {
        for (key2, _value2) in value1 {
            println!("get_route key: {key2}");
        }
    }
}
