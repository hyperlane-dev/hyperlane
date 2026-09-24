use super::*;

#[tokio::test]
async fn test_async_http_get_request() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("header-key", "header-value");

    let mut request_builder: BoxAsyncRequestTrait = RequestBuilder::new()
        .get("https://ide.ltpp.vip/?language=rust")
        .headers(header)
        .timeout(10000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .build_async();

    match request_builder.send().await {
        Ok(response) => {
            println!("Async GET ResponseTrait => {:?}", response.text());
        }
        Err(e) => println!("Async GET Error => {e}"),
    }
}

#[test]
fn test_http_post_request() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("Accept", "*/*");
    header.insert("Content-Type", "application/json");
    header.insert("Connection", "keep-alive");
    header.insert("Accept-Encoding", "gzip, deflate");
    let body: Value = serde_json::json!({
        "code": "fn main() {\r\n    println!(\"hello world\");\r\n}",
        "language": "rust",
        "testin": ""
    });
    let mut request_builder: BoxRequestTrait = RequestBuilder::new()
        .post("https://ide.ltpp.vip/?language=rust")
        .json(body)
        .headers(header)
        .timeout(6000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .build_sync();
    request_builder
        .send()
        .map(|response: BoxResponseTrait| {
            println!("ResponseTrait => {:?}", response.text());
        })
        .unwrap_or_else(|error: RequestError| println!("Error => {error}"));
}

#[test]
fn test_http_get_request() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("header-key", "header-value");
    let mut body: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    body.insert("body-key", "body-value");
    let mut request_builder: BoxRequestTrait = RequestBuilder::new()
        .get("http://ide.ltpp.vip/?language=rust")
        .headers(header)
        .timeout(6000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .build_sync();
    request_builder
        .send()
        .map(|response: BoxResponseTrait| {
            println!("ResponseTrait => {:?}", response.text());
        })
        .unwrap_or_else(|error: RequestError| println!("Error => {error}"));
}

#[test]
fn test_https_post_request() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("Accept", "*/*");
    header.insert("Content-Type", "application/json");
    header.insert("Connection", "keep-alive");
    header.insert("Accept-Encoding", "gzip, deflate");
    let body: Value = serde_json::json!({
        "code": "fn main() {\r\n    println!(\"hello world\");\r\n}",
        "language": "rust",
        "testin": ""
    });
    let mut request_builder: BoxRequestTrait = RequestBuilder::new()
        .post("https://code.ltpp.vip/")
        .json(body)
        .headers(header)
        .timeout(4000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .build_sync();
    request_builder
        .send()
        .map(|response: BoxResponseTrait| {
            println!("ResponseTrait => {:?}", response.text());
        })
        .unwrap_or_else(|error: RequestError| println!("Error => {error}"));
}

#[test]
fn test_https_get_request() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("header-key", "header-value");
    let mut body: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    body.insert("body-key", "body-value");
    let mut request_builder: BoxRequestTrait = RequestBuilder::new()
        .get("https://code.ltpp.vip/")
        .headers(header)
        .timeout(4000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .build_sync();
    request_builder
        .send()
        .map(|response: BoxResponseTrait| {
            println!("ResponseTrait => {:?}", response.text());
        })
        .unwrap_or_else(|error: RequestError| println!("Error => {error}"));
}

#[test]
fn test_http_post_text_request() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("Accept", "*/*");
    header.insert("Content-Type", "application/json");
    let mut request_builder: BoxRequestTrait = RequestBuilder::new()
        .post("http://code.ltpp.vip")
        .text("hello")
        .headers(header)
        .timeout(6000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .build_sync();
    request_builder
        .send()
        .map(|response: BoxResponseTrait| {
            println!("ResponseTrait => {:?}", response.text());
        })
        .unwrap_or_else(|error: RequestError| println!("Error => {error}"));
}

#[test]
fn test_http_post_binary_request() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("Accept", "*/*");
    header.insert("Content-Type", "application/json");
    let mut request_builder: BoxRequestTrait = RequestBuilder::new()
        .post("http://code.ltpp.vip")
        .body("hello".as_bytes())
        .headers(header)
        .timeout(6000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .build_sync();
    request_builder
        .send()
        .map(|response: BoxResponseTrait| {
            println!("ResponseTrait => {:?}", response.text());
        })
        .unwrap_or_else(|error: RequestError| println!("Error => {error}"));
}

#[test]
fn test_auto_gzip_get() {
    let mut request_builder: BoxRequestTrait = RequestBuilder::new()
        .get("https://ide.ltpp.vip/?language=rust")
        .timeout(4000)
        .redirect()
        .max_redirect_times(8)
        .decode()
        .buffer(4096)
        .http1_1_only()
        .build_sync();
    request_builder
        .send()
        .map(|response: BoxResponseTrait| {
            println!("ResponseTrait => {:?}", response.text());
        })
        .unwrap_or_else(|error: RequestError| println!("Error => {error}"));
}

#[test]
fn test_gzip_get() {
    let mut request_builder: BoxRequestTrait = RequestBuilder::new()
        .get("https://ide.ltpp.vip/?language=rust")
        .timeout(4000)
        .redirect()
        .max_redirect_times(8)
        .buffer(4096)
        .http1_1_only()
        .build_sync();
    request_builder
        .send()
        .map(|response: BoxResponseTrait| {
            println!("ResponseTrait => {:?}", response.decode(4096).text());
        })
        .unwrap_or_else(|error: RequestError| println!("Error => {error}"));
}

#[test]
fn test_unredirect_get() {
    let mut request_builder: BoxRequestTrait = RequestBuilder::new()
        .post("https://ide.ltpp.vip/?language=rust")
        .timeout(4000)
        .max_redirect_times(8)
        .buffer(4096)
        .unredirect()
        .http1_1_only()
        .build_sync();
    request_builder
        .send()
        .map(|response: BoxResponseTrait| {
            println!("ResponseTrait => {response:?}");
        })
        .unwrap_or_else(|error: RequestError| println!("Error => {error}"));
}

#[test]
fn test_thread_https_get_request() {
    let header_key: &str = "header-key";
    let header_value: &str = "header-value";
    let body_key: &str = "body-key";
    let body_value: &str = "body-value";
    let mut body: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    body.insert(body_key, body_value);
    let num_threads: i32 = 10;
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert(header_key, header_value);
    let request_builder: Arc<Mutex<BoxRequestTrait>> = Arc::new(Mutex::new(
        RequestBuilder::new()
            .get("https://code.ltpp.vip/")
            .headers(header.clone())
            .timeout(4000)
            .redirect()
            .buffer(4096)
            .max_redirect_times(8)
            .http1_1_only()
            .build_sync(),
    ));
    for _ in 0..num_threads {
        let request_builder: Arc<Mutex<BoxRequestTrait>> = Arc::clone(&request_builder);
        let handle: JoinHandle<()> = spawn(move || {
            let start_time: Instant = Instant::now();
            match request_builder.lock().unwrap().send() {
                Ok(response) => {
                    let duration: Duration = start_time.elapsed();
                    let response_text: HttpResponseText = response.text();
                    println!("Thread finished in: {duration:?}");
                    println!("ResponseTrait => {response_text:?}");
                }
                Err(e) => {
                    let duration: Duration = start_time.elapsed();
                    println!("Thread finished in: {duration:?}");
                    println!("Error => {e}");
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_thread_http_get_request() {
    let num_threads: i32 = 10;
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let request_builder: Arc<Mutex<BoxRequestTrait>> = Arc::new(Mutex::new(
        RequestBuilder::new()
            .get("http://127.0.0.1:7890/")
            .timeout(10)
            .redirect()
            .buffer(100)
            .max_redirect_times(0)
            .http2_only()
            .build_sync(),
    ));
    for _ in 0..num_threads {
        let request_builder: Arc<Mutex<BoxRequestTrait>> = Arc::clone(&request_builder);
        let handle: JoinHandle<()> = spawn(move || {
            let start_time: Instant = Instant::now();
            match request_builder.lock().unwrap().send() {
                Ok(response) => {
                    let duration: Duration = start_time.elapsed();
                    let response_text: HttpResponseText = response.text();
                    println!("Thread finished in: {duration:?}");
                    println!("ResponseTrait => {response_text:?}");
                }
                Err(e) => {
                    let duration: Duration = start_time.elapsed();
                    println!("Thread finished in: {duration:?}");
                    println!("Error => {e}");
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_readme_sync_get_request() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("header-key", "header-value");
    let mut request_builder: BoxRequestTrait = RequestBuilder::new()
        .get("https://ltpp.vip/")
        .headers(header)
        .timeout(6000)
        .redirect()
        .max_redirect_times(8)
        .http1_1_only()
        .buffer(4096)
        .decode()
        .build_sync();
    request_builder
        .send()
        .map(|response: BoxResponseTrait| {
            println!("{:?}", response.text());
        })
        .unwrap_or_else(|error: RequestError| println!("Error => {error}"));
}

#[test]
fn test_readme_sync_post_json_request() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("header-key", "header-value");
    let body: Value = serde_json::json!({
        "test": 1
    });
    let mut request_builder: BoxRequestTrait = RequestBuilder::new()
        .post("http://code.ltpp.vip")
        .json(body)
        .headers(header)
        .timeout(6000)
        .redirect()
        .max_redirect_times(8)
        .http1_1_only()
        .buffer(4096)
        .build_sync();
    request_builder
        .send()
        .map(|response: BoxResponseTrait| {
            println!("{:?}", response.decode(4096).text());
        })
        .unwrap_or_else(|error: RequestError| println!("Error => {error}"));
}

#[test]
fn test_readme_sync_post_text_request() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("header-key", "header-value");
    let mut request_builder: BoxRequestTrait = RequestBuilder::new()
        .post("http://ide.ltpp.vip/?language=rust")
        .text("hello")
        .headers(header)
        .timeout(6000)
        .redirect()
        .max_redirect_times(8)
        .http1_1_only()
        .buffer(4096)
        .decode()
        .build_sync();
    request_builder
        .send()
        .map(|response: BoxResponseTrait| {
            println!("{:?}", response.text());
        })
        .unwrap_or_else(|error: RequestError| println!("Error => {error}"));
}

#[test]
fn test_readme_sync_post_binary_request() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("header-key", "header-value");
    let mut request_builder: BoxRequestTrait = RequestBuilder::new()
        .post("http://ide.ltpp.vip/?language=rust")
        .body("hello".as_bytes())
        .headers(header)
        .timeout(6000)
        .redirect()
        .max_redirect_times(8)
        .http1_1_only()
        .buffer(4096)
        .build_sync();
    request_builder
        .send()
        .map(|response: BoxResponseTrait| {
            println!("{:?}", response.decode(4096).text());
        })
        .unwrap_or_else(|error: RequestError| println!("Error => {error}"));
}

#[tokio::test]
async fn test_async_websocket_connection() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("Authorization", "Bearer test-token");

    let mut websocket_builder: WebSocket = WebSocketBuilder::new()
        .connect("ws://127.0.0.1:60006/api/ws?uuid=1")
        .headers(header)
        .timeout(10000)
        .buffer(4096)
        .protocols(&["chat", "superchat"])
        .build_async();

    match websocket_builder.send_text_async("Hello WebSocket!").await {
        Ok(_) => {
            println!("Async WebSocket text message sent successfully");
            match websocket_builder.send_binary_async(b"binary data").await {
                Ok(_) => {
                    println!("Async WebSocket binary message sent successfully");
                    match websocket_builder.receive_async().await {
                        Ok(message) => match message {
                            WebSocketMessage::Text(text) => println!("Received text: {text}"),
                            WebSocketMessage::Binary(data) => {
                                println!("Received binary: {data:?}")
                            }
                            WebSocketMessage::Close => println!("Connection closed"),
                            _ => println!("Received other message type"),
                        },
                        Err(e) => println!("Error receiving message: {e}"),
                    }
                }
                Err(e) => println!("Error sending binary: {e}"),
            }
        }
        Err(e) => println!("Error sending text: {e}"),
    }

    websocket_builder
        .close_async_method()
        .await
        .unwrap_or_else(|error: WebSocketError| println!("Error closing: {error}"));
}

#[test]
fn test_sync_websocket_connection() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("Authorization", "Bearer test-token");

    let mut websocket_builder: WebSocket = WebSocketBuilder::new()
        .connect("ws://127.0.0.1:60006/api/ws?uuid=1")
        .headers(header)
        .timeout(10000)
        .buffer(4096)
        .protocols(&["chat", "superchat"])
        .build_sync();

    websocket_builder
        .send_text("Hello WebSocket!")
        .and_then(|_| {
            println!("Sync WebSocket text message sent successfully");
            websocket_builder.send_binary(b"binary data")
        })
        .map(|_| {
            println!("Sync WebSocket binary message sent successfully");
            match websocket_builder.receive() {
                Ok(message) => match message {
                    WebSocketMessage::Text(text) => println!("Received text: {text}"),
                    WebSocketMessage::Binary(data) => println!("Received binary: {data:?}"),
                    WebSocketMessage::Close => println!("Connection closed"),
                    _ => println!("Received other message type"),
                },
                Err(e) => println!("Error receiving message: {e}"),
            }
        })
        .and_then(|_| websocket_builder.close())
        .unwrap_or_else(|error: WebSocketError| println!("Error => {error}"));
}

#[test]
fn test_websocket_with_http_proxy() {
    let mut websocket_builder: WebSocket = WebSocketBuilder::new()
        .connect("ws://127.0.0.1:60006/api/ws?uuid=1")
        .timeout(10000)
        .buffer(4096)
        .http_proxy("127.0.0.1", 7890)
        .build_sync();

    match websocket_builder.send_text("Hello WebSocket with HTTP proxy!") {
        Ok(_) => println!("WebSocket HTTP proxy test unexpectedly succeeded"),
        Err(e) => {
            println!("WebSocket HTTP proxy test correctly failed: {e}");
        }
    }
}

#[test]
fn test_websocket_with_https_proxy() {
    let mut websocket_builder: WebSocket = WebSocketBuilder::new()
        .connect("ws://127.0.0.1:60006/api/ws?uuid=1")
        .timeout(10000)
        .buffer(4096)
        .https_proxy("127.0.0.1", 7890)
        .build_sync();

    match websocket_builder.send_text("Hello WebSocket with HTTPS proxy!") {
        Ok(_) => println!("WebSocket HTTPS proxy test unexpectedly succeeded"),
        Err(e) => {
            println!("WebSocket HTTPS proxy test correctly failed: {e}");
        }
    }
}

#[test]
fn test_websocket_with_socks5_proxy() {
    let mut websocket_builder: WebSocket = WebSocketBuilder::new()
        .connect("ws://127.0.0.1:60006/api/ws?uuid=1")
        .timeout(10000)
        .buffer(4096)
        .socks5_proxy("127.0.0.1", 1080)
        .build_sync();

    match websocket_builder.send_text("Hello WebSocket with SOCKS5 proxy!") {
        Ok(_) => println!("WebSocket SOCKS5 proxy test unexpectedly succeeded"),
        Err(e) => {
            println!("WebSocket SOCKS5 proxy test correctly failed: {e}");
        }
    }
}

#[test]
fn test_websocket_with_http_proxy_auth() {
    let mut websocket_builder: WebSocket = WebSocketBuilder::new()
        .connect("ws://127.0.0.1:60006/api/ws?uuid=1")
        .timeout(10000)
        .buffer(4096)
        .http_proxy_auth("127.0.0.1", 7890, "username", "password")
        .build_sync();

    match websocket_builder.send_text("Hello WebSocket with HTTP proxy auth!") {
        Ok(_) => println!("WebSocket HTTP proxy auth test unexpectedly succeeded"),
        Err(e) => {
            println!("WebSocket HTTP proxy auth test correctly failed: {e}");
        }
    }
}

#[tokio::test]
async fn test_websocket_with_socks5_proxy_auth_async() {
    let mut websocket_builder: WebSocket = WebSocketBuilder::new()
        .connect("ws://127.0.0.1:60006/api/ws?uuid=1")
        .timeout(10000)
        .buffer(4096)
        .socks5_proxy_auth("127.0.0.1", 1080, "username", "password")
        .build_async();

    match websocket_builder
        .send_text_async("Hello WebSocket with SOCKS5 proxy auth!")
        .await
    {
        Ok(_) => println!("WebSocket SOCKS5 proxy auth async test unexpectedly succeeded"),
        Err(e) => {
            println!("WebSocket SOCKS5 proxy auth async test correctly failed: {e}");
        }
    }
}

#[tokio::test]
async fn test_websocket_with_https_proxy_auth_async() {
    let mut websocket_builder: WebSocket = WebSocketBuilder::new()
        .connect("ws://127.0.0.1:60006/api/ws?uuid=1")
        .timeout(10000)
        .buffer(4096)
        .https_proxy_auth("127.0.0.1", 7890, "username", "password")
        .build_async();

    match websocket_builder
        .send_text_async("Hello WebSocket with HTTPS proxy auth!")
        .await
    {
        Ok(_) => println!("WebSocket HTTPS proxy auth async test unexpectedly succeeded"),
        Err(e) => {
            println!("WebSocket HTTPS proxy auth async test correctly failed: {e}");
        }
    }
}

#[tokio::test]
async fn test_readme_async_get_request() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("header-key", "header-value");
    let mut request_builder: BoxAsyncRequestTrait = RequestBuilder::new()
        .get("https://ltpp.vip/")
        .headers(header)
        .timeout(6000)
        .redirect()
        .max_redirect_times(8)
        .http1_1_only()
        .buffer(4096)
        .decode()
        .build_async();
    match request_builder.send().await {
        Ok(response) => {
            println!("{:?}", response.text());
        }
        Err(e) => println!("Error => {e}"),
    }
}

#[test]
fn test_case_insensitive_header_matching() {
    let mut header1: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header1.insert("Content-Type", "application/json");
    header1.insert("User-Agent", "test-agent");

    let mut header2: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header2.insert("content-type", "text/html");
    header2.insert("HOST", "example.com");

    let _request_builder: BoxRequestTrait = RequestBuilder::new()
        .get("http://ide.ltpp.vip/?language=rust")
        .headers(header1)
        .headers(header2)
        .timeout(6000)
        .build_sync();

    println!("Case insensitive header test completed");
}

#[test]
fn test_case_insensitive_required_headers() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("host", "custom-host.com");
    header.insert("CONTENT-LENGTH", "100");
    header.insert("Accept", "application/xml");
    header.insert("user-agent", "custom-agent");

    let _request_builder: BoxRequestTrait = RequestBuilder::new()
        .get("http://ide.ltpp.vip/?language=rust")
        .headers(header)
        .timeout(6000)
        .build_sync();

    println!("Case insensitive required headers test completed");
}

#[test]
fn test_http_proxy_get_request() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("header-key", "header-value");
    let mut request_builder: BoxRequestTrait = RequestBuilder::new()
        .get("http://ide.ltpp.vip/?language=rust")
        .headers(header)
        .timeout(10000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .http_proxy("127.0.0.1", 7890)
        .build_sync();

    match request_builder.send() {
        Ok(response) => {
            println!("HTTP Proxy GET Response => {:?}", response.text());
        }
        Err(e) => println!("HTTP Proxy GET Error (expected) => {e}"),
    }
}

#[test]
fn test_http_proxy_auth_get_request() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("header-key", "header-value");
    let mut request_builder: BoxRequestTrait = RequestBuilder::new()
        .get("http://ide.ltpp.vip/?language=rust")
        .headers(header)
        .timeout(10000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .http_proxy_auth("127.0.0.1", 7890, "username", "password")
        .build_sync();

    match request_builder.send() {
        Ok(response) => {
            println!("HTTP Proxy Auth GET Response => {:?}", response.text());
        }
        Err(e) => println!("HTTP Proxy Auth GET Error (expected) => {e}"),
    }
}

#[test]
fn test_socks5_proxy_get_request() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("header-key", "header-value");
    let mut request_builder: BoxRequestTrait = RequestBuilder::new()
        .get("http://ide.ltpp.vip/?language=rust")
        .headers(header)
        .timeout(10000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .socks5_proxy("127.0.0.1", 1080)
        .build_sync();

    match request_builder.send() {
        Ok(response) => {
            println!("SOCKS5 Proxy GET Response => {:?}", response.text());
        }
        Err(e) => println!("SOCKS5 Proxy GET Error (expected) => {e}"),
    }
}

#[tokio::test]
async fn test_async_http_proxy_get_request() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("header-key", "header-value");

    let mut request_builder: BoxAsyncRequestTrait = RequestBuilder::new()
        .get("http://ide.ltpp.vip/?language=rust")
        .headers(header)
        .timeout(10000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .http_proxy("127.0.0.1", 7890)
        .build_async();

    match request_builder.send().await {
        Ok(response) => {
            println!("Async HTTP Proxy GET Response => {:?}", response.text());
        }
        Err(e) => println!("Async HTTP Proxy GET Error (expected) => {e}"),
    }
}

#[tokio::test]
async fn test_async_socks5_proxy_auth_get_request() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("header-key", "header-value");

    let mut request_builder: BoxAsyncRequestTrait = RequestBuilder::new()
        .get("http://ide.ltpp.vip/?language=rust")
        .headers(header)
        .timeout(10000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .socks5_proxy_auth("127.0.0.1", 1080, "username", "password")
        .build_async();

    match request_builder.send().await {
        Ok(response) => {
            println!(
                "Async SOCKS5 Proxy Auth GET Response => {:?}",
                response.text()
            );
        }
        Err(e) => println!("Async SOCKS5 Proxy Auth GET Error (expected) => {e}"),
    }
}

#[tokio::test]
async fn test_readme_async_post_json_request() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("header-key", "header-value");
    let body: Value = serde_json::json!({
        "test": 1
    });
    let mut request_builder: BoxAsyncRequestTrait = RequestBuilder::new()
        .post("http://code.ltpp.vip")
        .json(body)
        .headers(header)
        .timeout(6000)
        .redirect()
        .max_redirect_times(8)
        .http1_1_only()
        .buffer(4096)
        .build_async();
    match request_builder.send().await {
        Ok(response) => {
            println!("{:?}", response.decode(4096).text());
        }
        Err(e) => println!("Error => {e}"),
    }
}

#[tokio::test]
async fn test_readme_async_post_text_request() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("header-key", "header-value");
    let mut request_builder: BoxAsyncRequestTrait = RequestBuilder::new()
        .post("http://ide.ltpp.vip/?language=rust")
        .text("hello")
        .headers(header)
        .timeout(6000)
        .redirect()
        .max_redirect_times(8)
        .http1_1_only()
        .buffer(4096)
        .decode()
        .build_async();
    match request_builder.send().await {
        Ok(response) => {
            println!("{:?}", response.text());
        }
        Err(e) => println!("Error => {e}"),
    }
}

#[tokio::test]
async fn test_readme_async_post_binary_request() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("header-key", "header-value");
    let mut request_builder: BoxAsyncRequestTrait = RequestBuilder::new()
        .post("http://ide.ltpp.vip/?language=rust")
        .body("hello".as_bytes())
        .headers(header)
        .timeout(6000)
        .redirect()
        .max_redirect_times(8)
        .http1_1_only()
        .buffer(4096)
        .build_async();
    match request_builder.send().await {
        Ok(response) => {
            println!("{:?}", response.decode(4096).text());
        }
        Err(e) => println!("Error => {e}"),
    }
}

#[tokio::test]
async fn test_https_over_http_proxy_async() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("User-Agent", "test-agent");

    let mut request_builder: BoxAsyncRequestTrait = RequestBuilder::new()
        .get("https://ide.ltpp.vip/?language=rust")
        .headers(header)
        .timeout(10000)
        .http_proxy("127.0.0.1", 7890)
        .build_async();

    match request_builder.send().await {
        Ok(response) => {
            println!(
                "HTTPS over HTTP proxy test passed: {}",
                response.binary().get_status_code()
            );
        }
        Err(e) => {
            println!("HTTPS over HTTP proxy test failed (expected): {e}");
        }
    }
}

#[tokio::test]
async fn test_https_over_socks5_proxy_async() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("User-Agent", "test-agent");

    let mut request_builder: BoxAsyncRequestTrait = RequestBuilder::new()
        .get("https://ide.ltpp.vip/?language=rust")
        .headers(header)
        .timeout(10000)
        .socks5_proxy("127.0.0.1", 1080)
        .build_async();

    match request_builder.send().await {
        Ok(response) => {
            println!(
                "HTTPS over SOCKS5 proxy test passed: {}",
                response.binary().get_status_code()
            );
        }
        Err(e) => {
            println!("HTTPS over SOCKS5 proxy test failed (expected): {e}");
        }
    }
}

#[test]
fn test_https_over_http_proxy_sync() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("User-Agent", "test-agent");

    let mut request_builder: BoxRequestTrait = RequestBuilder::new()
        .get("https://ide.ltpp.vip/?language=rust")
        .headers(header)
        .timeout(10000)
        .http_proxy("127.0.0.1", 7890)
        .build_sync();

    match request_builder.send() {
        Ok(response) => {
            println!(
                "Sync HTTPS over HTTP proxy test passed: {}",
                response.binary().get_status_code()
            );
        }
        Err(e) => {
            println!("Sync HTTPS over HTTP proxy test failed (expected): {e}");
        }
    }
}

#[test]
fn test_https_over_socks5_proxy_sync() {
    let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
    header.insert("User-Agent", "test-agent");

    let mut request_builder: BoxRequestTrait = RequestBuilder::new()
        .get("https://ide.ltpp.vip/?language=rust")
        .headers(header)
        .timeout(10000)
        .socks5_proxy("127.0.0.1", 1080)
        .build_sync();

    match request_builder.send() {
        Ok(response) => {
            println!(
                "Sync HTTPS over SOCKS5 proxy test passed: {}",
                response.binary().get_status_code()
            );
        }
        Err(e) => {
            println!("Sync HTTPS over SOCKS5 proxy test failed (expected): {e}");
        }
    }
}
