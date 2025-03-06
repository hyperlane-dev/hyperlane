#[tokio::test]
async fn test_server_basic_usage() {
    use crate::*;

    async fn request_middleware(controller_data: ControllerData) {
        let socket_addr: String = controller_data
            .get_socket_addr()
            .await
            .unwrap_or(DEFAULT_SOCKET_ADDR)
            .to_string();
        controller_data
            .set_response_header(SERVER, "hyperlane")
            .await
            .set_response_header(CONNECTION, CONNECTION_KEEP_ALIVE)
            .await
            .set_response_header("SocketAddr", socket_addr)
            .await;
    }

    async fn response_middleware(controller_data: ControllerData) {
        let request: String = controller_data.get_request().await.to_string();
        let response: String = controller_data.get_response().await.to_string();
        controller_data
            .log_info(format!("Request => {}", request), log_handler)
            .await
            .log_info(format!("Response => {}", response), log_handler)
            .await;
    }

    async fn root_router(controller_data: ControllerData) {
        let _ = controller_data
            .send_response(200, "hello hyperlane => /")
            .await;
    }

    async fn run_server() {
        let mut server: Server = Server::new();
        server.host("0.0.0.0").await;
        server.port(60000).await;
        server.log_dir("./logs").await;
        server.log_size(100_024_000).await;
        server.log_interval_millis(1000).await;
        server.request_middleware(request_middleware).await;
        server.router("/", root_router).await;
        server.response_middleware(response_middleware).await;
        let test_string: String = "test".to_owned();
        server
            .router(
                "/test/panic",
                async_func!(test_string, |data| {
                    println_success!(test_string);
                    println_success!(format!("{:?}", data));
                    panic!("test panic");
                }),
            )
            .await;
        server.listen().await;
    }

    recoverable_spawn::r#async::recoverable_spawn(run_server);
    std::thread::sleep(std::time::Duration::from_secs(10));
}
