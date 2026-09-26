use super::*;

#[test]
fn test_sync_http_get_request() {
    let mut request = RequestBuilder::new()
        .get("http://code.ltpp.vip/")
        .timeout(6000)
        .build();
    match request.send() {
        Ok(response) => println!("HTTP GET => status={}", response.status_code),
        Err(e) => println!("HTTP GET Error => {e}"),
    }
}

#[test]
fn test_sync_https_get_request() {
    let mut request = RequestBuilder::new()
        .get("https://code.ltpp.vip/")
        .timeout(6000)
        .build();
    match request.send() {
        Ok(response) => println!("HTTPS GET => status={}", response.status_code),
        Err(e) => println!("HTTPS GET Error => {e}"),
    }
}

#[tokio::test]
async fn test_async_https_get_request() {
    let mut request = RequestBuilder::new()
        .get("https://code.ltpp.vip/")
        .timeout(10000)
        .build();
    match request.send_async().await {
        Ok(response) => println!("Async HTTPS GET => status={}", response.status_code),
        Err(e) => println!("Async HTTPS GET Error => {e}"),
    }
}

#[test]
fn test_sync_post_json_request() {
    let body = serde_json::json!({
        "code": "fn main() {\r\n    println!(\"hello world\");\r\n}",
        "language": "rust",
    });
    let mut request = RequestBuilder::new()
        .post("https://ide.ltpp.vip/?language=rust")
        .body_json(&body)
        .timeout(6000)
        .build();
    match request.send() {
        Ok(response) => println!("POST JSON => body_len={}", response.body.len()),
        Err(e) => println!("POST JSON Error => {e}"),
    }
}

#[test]
fn test_sync_post_text_request() {
    let mut request = RequestBuilder::new()
        .post("http://ide.ltpp.vip/?language=rust")
        .body_text("hello")
        .timeout(6000)
        .build();
    match request.send() {
        Ok(response) => println!("POST text => status={}", response.status_code),
        Err(e) => println!("POST text Error => {e}"),
    }
}

#[test]
fn test_sync_post_binary_request() {
    let mut request = RequestBuilder::new()
        .post("http://ide.ltpp.vip/?language=rust")
        .body_text("hello")
        .timeout(6000)
        .build();
    match request.send() {
        Ok(response) => println!("POST binary => status={}", response.status_code),
        Err(e) => println!("POST binary Error => {e}"),
    }
}

#[test]
fn test_case_insensitive_header_matching() {
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    headers.insert("User-Agent".to_string(), "test-agent".to_string());
    let _request = RequestBuilder::new()
        .get("http://ide.ltpp.vip/?language=rust")
        .headers(headers)
        .timeout(6000)
        .build();
    println!("Case-insensitive header test completed");
}

#[test]
fn test_case_insensitive_required_headers() {
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("host".to_string(), "custom-host.com".to_string());
    headers.insert("accept".to_string(), "application/xml".to_string());
    let _request = RequestBuilder::new()
        .get("http://ide.ltpp.vip/?language=rust")
        .headers(headers)
        .timeout(6000)
        .build();
    println!("Case-insensitive required headers test completed");
}

#[test]
fn test_http_proxy_get_request() {
    let mut request = RequestBuilder::new()
        .get("http://ide.ltpp.vip/?language=rust")
        .proxy(Proxy::http("127.0.0.1", 7890))
        .timeout(10000)
        .build();
    match request.send() {
        Ok(response) => println!("HTTP Proxy GET => status={}", response.status_code),
        Err(e) => println!("HTTP Proxy GET Error (expected) => {e}"),
    }
}

#[test]
fn test_socks5_proxy_get_request() {
    let mut request = RequestBuilder::new()
        .get("http://ide.ltpp.vip/?language=rust")
        .proxy(Proxy::socks5("127.0.0.1", 1080))
        .timeout(10000)
        .build();
    match request.send() {
        Ok(response) => println!("SOCKS5 Proxy GET => status={}", response.status_code),
        Err(e) => println!("SOCKS5 Proxy GET Error (expected) => {e}"),
    }
}

#[test]
fn test_readme_sync_get_request() {
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("header-key".to_string(), "header-value".to_string());
    let mut request = RequestBuilder::new()
        .get("https://ltpp.vip/")
        .headers(headers)
        .timeout(6000)
        .build();
    match request.send() {
        Ok(response) => println!("README GET => body_len={}", response.body.len()),
        Err(e) => println!("README GET Error => {e}"),
    }
}

#[test]
fn test_readme_sync_post_json_request() {
    let body = json!({ "test": 1 });
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("header-key".to_string(), "header-value".to_string());
    let mut request = RequestBuilder::new()
        .post("http://code.ltpp.vip")
        .body_json(&body)
        .headers(headers)
        .timeout(6000)
        .build();
    match request.send() {
        Ok(response) => println!("README POST JSON => status={}", response.status_code),
        Err(e) => println!("README POST JSON Error => {e}"),
    }
}
