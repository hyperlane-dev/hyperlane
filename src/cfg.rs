#[tokio::test]
async fn test_server_basic_usage() {
    use crate::*;

    fn send_request() -> Vec<u8> {
        let mut header: HashMap<&str, &str> = HashMap::new();
        header.insert(ACCEPT, ACCEPT_ANY);
        header.insert(CONTENT_TYPE, APPLICATION_JSON);
        header.insert(ACCEPT_ENCODING, ACCEPT_ENCODING_GZIP);
        let mut body: HashMap<&str, &str> = HashMap::new();
        body.insert("code", "fn main() {\r\n    println!(\"hello world\");\r\n}");
        body.insert("language", "rust");
        body.insert("testin", "");
        let mut _request_builder = RequestBuilder::new()
            .post("https://code.ltpp.vip/")
            .json(body)
            .headers(header)
            .timeout(10000)
            .redirect()
            .buffer(4096)
            .max_redirect_times(8)
            .http1_1_only()
            .build();
        _request_builder
            .send()
            .and_then(|response| Ok(response.binary().get_body()))
            .unwrap_or_default()
    }

    fn sync_middleware(arc_lock_controller_data: ArcRwLockControllerData) {
        let mut controller_data: RwLockWriteControllerData =
            get_rw_lock_write_controller_data(&arc_lock_controller_data);
        let response: &mut Response = controller_data.get_mut_response();
        response.set_header("server", "hyperlane");
    }

    async fn test_async_middleware(arc_lock_controller_data: ArcRwLockControllerData) {
        sync_middleware(arc_lock_controller_data);
    }

    fn sync_root_router(arc_lock_controller_data: ArcRwLockControllerData) {
        let send_res: ResponseResult =
            send_response(&arc_lock_controller_data, 200, "hello hyperlane => /index");
        let controller_data: ControllerData = get_controller_data(&arc_lock_controller_data);
        controller_data.get_log().info(
            format!("Response result => {:?}", send_res),
            log_debug_format_handler,
        );
    }

    fn sync_request_router(arc_lock_controller_data: ArcRwLockControllerData) {
        let controller_data: ControllerData = get_controller_data(&arc_lock_controller_data);
        let mut response: Response = controller_data.get_response().clone();
        let body: Vec<u8> = send_request();
        let stream: ArcTcpStream = controller_data.get_stream().clone().unwrap();
        let res: ResponseResult = response
            .set_body(body)
            .set_status_code(200)
            .set_header("server", "hyperlane")
            .set_header(CONTENT_TYPE, APPLICATION_JSON)
            .send(&stream);
        controller_data
            .get_log()
            .info(format!("Response result => {:?}", res), log_handler);
    }

    fn sync_hello_router(arc_lock_controller_data: ArcRwLockControllerData) {
        let controller_data: ControllerData = get_controller_data(&arc_lock_controller_data);
        controller_data
            .get_log()
            .info("visit path /hello", log_handler);
        let mut response: Response = controller_data.get_response().clone();
        let body: &str = "hello world!";
        let stream: ArcTcpStream = controller_data.get_stream().clone().unwrap();
        let res: ResponseResult = response
            .set_body(body.into())
            .set_status_code(200)
            .set_header("server", "hyperlane")
            .send(&stream);
        controller_data
            .get_log()
            .info(format!("Response result => {:?}", res), log_handler);
    }

    fn sync_panic_route(_controller_data: ArcRwLock<ControllerData>) {
        panic!("test panic");
    }

    async fn async_test_router(arc_lock_controller_data: ArcRwLockControllerData) {
        let controller_data: ControllerData = get_controller_data(&arc_lock_controller_data);
        controller_data.get_log().info("visit path /", log_handler);
        let mut response: Response = controller_data.get_response().clone();
        let body: &str = "Async";
        let stream: ArcTcpStream = controller_data.get_stream().clone().unwrap();
        let res: ResponseResult = response
            .set_body(body.into())
            .set_status_code(200)
            .set_header("server", "hyperlane")
            .send(&stream);
        controller_data
            .get_log()
            .info(format!("Response result => {:?}", res), log_handler);
    }

    async fn run_server() {
        let mut server: Server = Server::new();
        server.host("0.0.0.0");
        server.port(60000);
        server.log_dir("./logs");
        server.log_size(100_024_000);
        server.log_interval_millis(1000);
        server.middleware(sync_middleware);
        server.async_middleware(test_async_middleware).await;
        server.router("/", sync_root_router);
        server.router("/request", sync_request_router);
        server.router("/hello", sync_hello_router);
        server.router("/panic", sync_panic_route);
        server.async_router("/async/test", async_test_router).await;
        let test_string: String = "test".to_owned();
        server
            .async_router(
                "/test/async_func",
                async_func!(test_string, |data| {
                    println_success!(test_string);
                    println_success!(format!("{:?}", data));
                }),
            )
            .await;
        server.listen();
    }

    let run_test = || {
        let mut header: HashMap<&str, &str> = HashMap::new();
        header.insert(ACCEPT, ACCEPT_ANY);
        header.insert(CONTENT_TYPE, APPLICATION_JSON);
        header.insert(ACCEPT_ENCODING, ACCEPT_ENCODING_GZIP);
        let mut body: HashMap<&str, &str> = HashMap::new();
        body.insert("key", "value");
        let mut _request_builder = RequestBuilder::new()
            .post("http://127.0.0.1:60000/")
            .json(body)
            .headers(header)
            .timeout(10000)
            .redirect()
            .buffer(4096)
            .max_redirect_times(8)
            .http1_1_only()
            .build();
        _request_builder
            .send()
            .and_then(|response| Ok(response.binary().get_body()))
            .unwrap_or_default();
    };
    recoverable_spawn::r#async::recoverable_spawn(run_server);
    std::thread::sleep(std::time::Duration::from_secs(2));
    recoverable_spawn::r#sync::recoverable_spawn(run_test);
    std::thread::sleep(std::time::Duration::from_secs(4));
}
