use crate::*;

#[cfg(test)]
async fn assert_panic_message_contains<F, Fut>(future_factory: F, expected_msg: &str)
where
    F: Fn() -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    let result: Result<(), JoinError> = spawn(future_factory()).await;
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
        s
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
            let _server: &Server = Server::new().route::<TestRoute>(EMPTY_STR);
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
                .route::<TestRoute>(ROOT_PATH)
                .route::<TestRoute>(ROOT_PATH);
        },
        &RouteError::DuplicatePattern(ROOT_PATH.to_string()).to_string(),
    )
    .await;
}

#[test]
fn get_route() {
    let server: Server = Server::new();
    server
        .route::<TestRoute>(ROOT_PATH)
        .route::<TestRoute>("/dynamic/{routing}")
        .route::<TestRoute>("/regex/{file:^.*$}");
    let route_matcher: RouteMatcher = server.get_route_matcher();
    for key in route_matcher.get_static_route().keys() {
        println!("Static route: {key}");
    }
    for value in route_matcher.get_dynamic_route().values() {
        for (route_pattern, _) in value {
            println!("Dynamic route: {route_pattern}");
        }
    }
    for value in route_matcher.get_regex_route().values() {
        for (route_pattern, _) in value {
            println!("Regex route: {route_pattern}");
        }
    }
}

#[test]
fn segment_count_optimization() {
    let server: Server = Server::new();
    server.route::<TestRoute>("/users/{id}");
    server.route::<TestRoute>("/users/{id}/posts");
    server.route::<TestRoute>("/users/{id}/posts/{post_id}");
    server.route::<TestRoute>("/api/v1/users/{id}");
    let route_matcher: RouteMatcher = server.get_route_matcher();
    assert!(
        route_matcher.get_dynamic_route().contains_key(&2),
        "Should have 2-segment routes"
    );
    assert!(
        route_matcher.get_dynamic_route().contains_key(&3),
        "Should have 3-segment routes"
    );
    assert!(
        route_matcher.get_dynamic_route().contains_key(&4),
        "Should have 4-segment routes"
    );
    assert_eq!(route_matcher.get_dynamic_route().get(&2).unwrap().len(), 1);
    assert_eq!(route_matcher.get_dynamic_route().get(&3).unwrap().len(), 1);
    assert_eq!(route_matcher.get_dynamic_route().get(&4).unwrap().len(), 2);
}

#[test]
fn regex_route_segment_count() {
    let server: Server = Server::new();
    server.route::<TestRoute>("/files/{path:.*}");
    server.route::<TestRoute>("/api/{version:\\d+}/users");
    server.route::<TestRoute>("/api/{version:\\d+}/posts/{id:\\d+}");
    let route_matcher: RouteMatcher = server.get_route_matcher();
    assert!(
        route_matcher.get_regex_route().contains_key(&2),
        "Should have 2-segment regex routes"
    );
    assert!(
        route_matcher.get_regex_route().contains_key(&3),
        "Should have 3-segment regex routes"
    );
    assert!(
        route_matcher.get_regex_route().contains_key(&4),
        "Should have 4-segment regex routes"
    );
}

#[test]
fn mixed_route_types() {
    let server: Server = Server::new();
    server.route::<TestRoute>("/");
    server.route::<TestRoute>("/about");
    server.route::<TestRoute>("/users/{id}");
    server.route::<TestRoute>("/posts/{slug}");
    server.route::<TestRoute>("/files/{path:.*}");
    let route_matcher: RouteMatcher = server.get_route_matcher();
    assert_eq!(route_matcher.get_static_route().len(), 2);
    assert!(route_matcher.get_dynamic_route().contains_key(&2));
    assert!(route_matcher.get_regex_route().contains_key(&2));
}

#[test]
fn large_dynamic_routes() {
    const ROUTE_COUNT: u32 = 1000;
    let server: Server = Server::new();
    let start_insert: Instant = Instant::now();
    for i in 0..ROUTE_COUNT {
        let path: String = format!("/api/resource{i}/{{id}}");
        server.route::<TestRoute>(&path);
    }
    let insert_duration: Duration = start_insert.elapsed();
    println!(
        "Inserted {} dynamic routes in: {:?}",
        ROUTE_COUNT, insert_duration
    );
    let route_matcher: RouteMatcher = server.get_route_matcher();
    assert!(!route_matcher.get_dynamic_route().is_empty());
    let ctx: Context = Context::default();
    let start_match: Instant = Instant::now();
    for i in 0..ROUTE_COUNT {
        let path: String = format!("/api/resource{i}/123");
        let _ = route_matcher.try_resolve_route(&ctx, &path);
    }
    let match_duration: Duration = start_match.elapsed();
    println!(
        "Matched {} dynamic routes in: {:?}",
        ROUTE_COUNT, match_duration
    );
    println!(
        "Average per dynamic route match: {:?}",
        match_duration / ROUTE_COUNT
    );
}

#[test]
fn large_regex_routes() {
    const ROUTE_COUNT: u32 = 1000;
    let server: Server = Server::new();
    let start_insert: Instant = Instant::now();
    for i in 0..ROUTE_COUNT {
        let path: String = format!("/api/resource{i}/{{id:[0-9]+}}");
        server.route::<TestRoute>(&path);
    }
    let insert_duration: Duration = start_insert.elapsed();
    println!(
        "Inserted {} regex routes in: {:?}",
        ROUTE_COUNT, insert_duration
    );
    let route_matcher: RouteMatcher = server.get_route_matcher();
    assert!(!route_matcher.get_regex_route().is_empty());
    let ctx: Context = Context::default();
    let start_match: Instant = Instant::now();
    for i in 0..ROUTE_COUNT {
        let path: String = format!("/api/resource{i}/123");
        let _ = route_matcher.try_resolve_route(&ctx, &path);
    }
    let match_duration: Duration = start_match.elapsed();
    println!(
        "Matched {} regex routes in: {:?}",
        ROUTE_COUNT, match_duration
    );
    println!(
        "Average per regex route match: {:?}",
        match_duration / ROUTE_COUNT
    );
}

#[test]
fn large_tail_regex_routes() {
    const ROUTE_COUNT: u32 = 1000;
    let server: Server = Server::new();
    let start_insert: Instant = Instant::now();
    for i in 0..ROUTE_COUNT {
        let path: String = format!("/api/resource{i}/{{path:.*}}");
        server.route::<TestRoute>(&path);
    }
    let insert_duration: Duration = start_insert.elapsed();
    println!(
        "Inserted {} tail regex routes in: {:?}",
        ROUTE_COUNT, insert_duration
    );
    let route_matcher: RouteMatcher = server.get_route_matcher();
    assert!(!route_matcher.get_regex_route().is_empty());
    let ctx: Context = Context::default();
    let start_match: Instant = Instant::now();
    for i in 0..ROUTE_COUNT {
        let path: String = format!("/api/resource{i}/some/nested/path");
        let _ = route_matcher.try_resolve_route(&ctx, &path);
    }
    let match_duration: Duration = start_match.elapsed();
    println!(
        "Matched {} tail regex routes in: {:?}",
        ROUTE_COUNT, match_duration
    );
    println!(
        "Average per tail regex route match: {:?}",
        match_duration / ROUTE_COUNT
    );
}
