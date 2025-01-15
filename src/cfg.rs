#[tokio::test]
async fn test_server_basic_usage() {
    use crate::*;

    fn println(data: &str) {
        let binding: String = current_time();
        let mut time_output_builder: OutputBuilder<'_> = OutputBuilder::new();
        let mut text_output_builder: OutputBuilder<'_> = OutputBuilder::new();
        let time_output: Output<'_> = time_output_builder
            .text(&binding)
            .blod(true)
            .bg_color(ColorType::Use(Color::Yellow))
            .color(ColorType::Rgb(255, 255, 255))
            .build();
        let text_output: Output<'_> = text_output_builder
            .text(data)
            .blod(true)
            .bg_color(ColorType::Use(Color::Green))
            .color(ColorType::Rgb(255, 255, 255))
            .endl(true)
            .build();
        OutputListBuilder::new()
            .add(time_output)
            .add(text_output)
            .run();
    }

    fn common_log(log_data: &String) -> String {
        println(&log_data);
        let write_data: String = format!("{}: {}\n", current_time(), log_data);
        write_data.clone()
    }

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

    fn panic_route(_controller_data: ArcRwLock<ControllerData>) {
        panic!("test panic");
    }

    async fn run_server() {
        let mut server: Server = Server::new();
        server.host("0.0.0.0");
        server.port(60000);
        server.thread_pool_size(1);
        server.log_dir("./logs");
        server.log_size(1_024_000);

        server.middleware(|arc_lock_controller_data| {
            let mut controller_data: RwLockWriteControllerData =
                arc_lock_controller_data.write().unwrap();
            let request: Request = controller_data.get_request().clone();
            let stream: ArcTcpStream = controller_data.get_stream().clone().unwrap();
            let host: String = stream
                .peer_addr()
                .and_then(|host| Ok(host.to_string()))
                .unwrap_or("Unknown".to_owned());
            controller_data.get_log().log_debug(
                format!("Request host => {}\n{:#?}", host, request),
                common_log,
            );
            controller_data
                .get_mut_request()
                .set_header("middleware", "crate");
        });

        server.router("/", |arc_lock_controller_data| {
            let controller_data: RwLockWriteControllerData =
                arc_lock_controller_data.write().unwrap();
            controller_data
                .get_log()
                .log_info("visit path /", common_log);
            let mut response: Response = controller_data.get_response().clone();
            let body: Vec<u8> = "404 Not Found".as_bytes().to_vec();
            let stream: ArcTcpStream = controller_data.get_stream().clone().unwrap();
            let res: ResponseResult = response
                .set_body(body)
                .set_status_code(404)
                .set_header("server", "hyperlane")
                .send(&stream);
            controller_data.get_log().log_info(
                format!("Response => {:?}", String::from_utf8_lossy(&res.unwrap())),
                common_log,
            );
        });

        server
            .async_router("/async", |arc_lock_controller_data| async move {
                let controller_data: RwLockWriteControllerData =
                    arc_lock_controller_data.write().unwrap();
                controller_data
                    .get_log()
                    .log_info("visit path /", common_log);
                let mut response: Response = controller_data.get_response().clone();
                let body: Vec<u8> = "Async".as_bytes().to_vec();
                let stream: ArcTcpStream = controller_data.get_stream().clone().unwrap();
                let res: ResponseResult = response
                    .set_body(body)
                    .set_status_code(200)
                    .set_header("server", "hyperlane")
                    .send(&stream);
                controller_data.get_log().log_info(
                    format!("Response => {:?}", String::from_utf8_lossy(&res.unwrap())),
                    common_log,
                );
            })
            .await;

        server.router("/request", |arc_lock_controller_data| {
            let controller_data: RwLockWriteControllerData =
                arc_lock_controller_data.write().unwrap();
            controller_data
                .get_log()
                .log_info("visit path /request", common_log);
            let mut response: Response = controller_data.get_response().clone();
            let body: Vec<u8> = send_request();
            let stream: ArcTcpStream = controller_data.get_stream().clone().unwrap();
            let res: ResponseResult = response
                .set_body(body)
                .set_status_code(200)
                .set_header("server", "hyperlane")
                .set_header(CONTENT_TYPE, APPLICATION_JSON)
                .send(&stream);
            controller_data.get_log().log_info(
                format!("Response => {:?}", String::from_utf8_lossy(&res.unwrap())),
                common_log,
            );
        });

        server.router("/hello", |arc_lock_controller_data| {
            let controller_data: RwLockWriteControllerData =
                arc_lock_controller_data.write().unwrap();
            controller_data
                .get_log()
                .log_info("visit path /hello", common_log);
            let mut response: Response = controller_data.get_response().clone();
            let body: Vec<u8> = "hello world!".as_bytes().to_vec();
            let stream: ArcTcpStream = controller_data.get_stream().clone().unwrap();
            let res: ResponseResult = response
                .set_body(body)
                .set_status_code(200)
                .set_header("server", "hyperlane")
                .send(&stream);
            controller_data.get_log().log_info(
                format!("Response => {:?}", String::from_utf8_lossy(&res.unwrap())),
                common_log,
            );
        });

        server.router("/panic", panic_route);
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
    run_server().await;
    async_recoverable_spawn(run_server);
    std::thread::sleep(std::time::Duration::from_secs(2));
    recoverable_spawn(run_test);
    std::thread::sleep(std::time::Duration::from_secs(4));
}
