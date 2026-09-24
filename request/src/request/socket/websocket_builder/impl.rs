use super::*;

impl WebSocketBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn connect(&mut self, url: &str) -> &mut Self {
        self.websocket.url = Arc::new(url.to_owned());
        self
    }

    pub fn headers<K, V>(&mut self, header: HashMapXxHash3_64<K, V>) -> &mut Self
    where
        K: ToString,
        V: ToString,
    {
        if let Some(tmp_header) = Arc::get_mut(&mut self.websocket.header) {
            for (key, value) in header {
                let key_str: String = key.to_string();
                let value_str: String = value.to_string();
                let mut found_existing: bool = false;
                let mut existing_key: Option<String> = None;
                for existing_key_ref in tmp_header.keys() {
                    if existing_key_ref.eq_ignore_ascii_case(&key_str) {
                        existing_key = Some(existing_key_ref.clone());
                        found_existing = true;
                        break;
                    }
                }
                if found_existing && let Some(existing_key) = existing_key {
                    tmp_header.remove(&existing_key);
                }
                let mut value_deque: VecDeque<String> = VecDeque::new();
                value_deque.push_front(value_str);
                tmp_header.insert(key_str, value_deque);
            }
        }
        self
    }

    pub fn timeout(&mut self, timeout: u64) -> &mut Self {
        if let Ok(mut config) = self.websocket.config.write() {
            config.timeout = timeout;
        }
        self
    }

    pub fn buffer(&mut self, buffer: usize) -> &mut Self {
        if let Ok(mut config) = self.websocket.config.write() {
            config.buffer = buffer;
        }
        self
    }

    pub fn protocols(&mut self, protocols: &[&str]) -> &mut Self {
        if let Ok(mut config) = self.websocket.config.write() {
            config.protocols = protocols
                .iter()
                .map(|protocol: &&str| protocol.to_string())
                .collect();
        }
        self
    }

    pub fn http_proxy(&mut self, host: &str, port: u16) -> &mut Self {
        if let Ok(mut config) = self.websocket.config.write() {
            config.proxy = Some(ProxyConfig {
                proxy_type: ProxyType::Http,
                host: host.to_string(),
                port,
                username: None,
                password: None,
            });
        }
        self
    }

    pub fn https_proxy(&mut self, host: &str, port: u16) -> &mut Self {
        if let Ok(mut config) = self.websocket.config.write() {
            config.proxy = Some(ProxyConfig {
                proxy_type: ProxyType::Https,
                host: host.to_string(),
                port,
                username: None,
                password: None,
            });
        }
        self
    }

    pub fn socks5_proxy(&mut self, host: &str, port: u16) -> &mut Self {
        if let Ok(mut config) = self.websocket.config.write() {
            config.proxy = Some(ProxyConfig {
                proxy_type: ProxyType::Socks5,
                host: host.to_string(),
                port,
                username: None,
                password: None,
            });
        }
        self
    }

    pub fn http_proxy_auth(
        &mut self,
        host: &str,
        port: u16,
        username: &str,
        password: &str,
    ) -> &mut Self {
        if let Ok(mut config) = self.websocket.config.write() {
            config.proxy = Some(ProxyConfig {
                proxy_type: ProxyType::Http,
                host: host.to_string(),
                port,
                username: Some(username.to_string()),
                password: Some(password.to_string()),
            });
        }
        self
    }

    pub fn https_proxy_auth(
        &mut self,
        host: &str,
        port: u16,
        username: &str,
        password: &str,
    ) -> &mut Self {
        if let Ok(mut config) = self.websocket.config.write() {
            config.proxy = Some(ProxyConfig {
                proxy_type: ProxyType::Https,
                host: host.to_string(),
                port,
                username: Some(username.to_string()),
                password: Some(password.to_string()),
            });
        }
        self
    }

    pub fn socks5_proxy_auth(
        &mut self,
        host: &str,
        port: u16,
        username: &str,
        password: &str,
    ) -> &mut Self {
        if let Ok(mut config) = self.websocket.config.write() {
            config.proxy = Some(ProxyConfig {
                proxy_type: ProxyType::Socks5,
                host: host.to_string(),
                port,
                username: Some(username.to_string()),
                password: Some(password.to_string()),
            });
        }
        self
    }

    pub fn build_sync(&mut self) -> WebSocket {
        self.builder = self.websocket.clone();
        self.websocket = WebSocket::default();
        self.builder.clone()
    }

    pub fn build_async(&mut self) -> WebSocket {
        self.builder = self.websocket.clone();
        self.websocket = WebSocket::default();
        self.builder.clone()
    }
}
