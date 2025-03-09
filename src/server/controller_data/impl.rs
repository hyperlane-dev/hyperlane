use crate::*;

impl InnerControllerData {
    #[inline]
    pub fn new() -> Self {
        InnerControllerData {
            stream: None,
            request: Request::default(),
            response: Response::default(),
            log: Log::default(),
        }
    }
}

impl ControllerData {
    #[inline]
    pub(crate) fn from_controller_data(controller_data: InnerControllerData) -> Self {
        Self(Arc::new(RwLock::new(controller_data)))
    }

    #[inline]
    pub async fn get(&self) -> InnerControllerData {
        let controller_data: InnerControllerData = self.get_read_lock().await.clone();
        controller_data
    }

    #[inline]
    pub async fn get_read_lock(&self) -> RwLockReadControllerData {
        let controller_data: RwLockReadControllerData = self.0.read().await;
        controller_data
    }

    #[inline]
    pub async fn get_write_lock(&self) -> RwLockWriteControllerData {
        let controller_data: RwLockWriteControllerData = self.0.write().await;
        controller_data
    }

    #[inline]
    pub async fn get_stream(&self) -> OptionArcRwLockStream {
        let controller_data: InnerControllerData = self.get().await;
        controller_data.get_stream().clone()
    }

    #[inline]
    pub async fn get_request(&self) -> Request {
        let controller_data: InnerControllerData = self.get().await;
        controller_data.get_request().clone()
    }

    #[inline]
    pub async fn get_response(&self) -> Response {
        let controller_data: InnerControllerData = self.get().await;
        controller_data.get_response().clone()
    }

    #[inline]
    pub async fn get_log(&self) -> Log {
        let controller_data: InnerControllerData = self.get().await;
        controller_data.get_log().clone()
    }

    #[inline]
    pub async fn get_socket_addr(&self) -> OptionSocketAddr {
        let stream_result: OptionArcRwLockStream = self.get_stream().await;
        if stream_result.is_none() {
            return None;
        }
        let socket_addr_opt: OptionSocketAddr = stream_result
            .unwrap()
            .get_read_lock()
            .await
            .peer_addr()
            .ok();
        socket_addr_opt
    }

    #[inline]
    pub async fn get_socket_host(&self) -> OptionSocketHost {
        let addr: OptionSocketAddr = self.get_socket_addr().await;
        let socket_host_opt: OptionSocketHost =
            addr.map(|socket_addr: SocketAddr| socket_addr.ip());
        socket_host_opt
    }

    #[inline]
    pub async fn get_socket_port(&self) -> OptionSocketPort {
        let addr: OptionSocketAddr = self.get_socket_addr().await;
        let socket_port_opt: OptionSocketPort =
            addr.map(|socket_addr: SocketAddr| socket_addr.port());
        socket_port_opt
    }

    #[inline]
    pub async fn send_response<T: Into<ResponseBody>>(
        &self,
        status_code: usize,
        response_body: T,
    ) -> ResponseResult {
        if let Some(stream_lock) = self.get_stream().await {
            let mut controller_data: RwLockWriteControllerData = self.get_write_lock().await;
            let response: &mut Response = controller_data.get_mut_response();
            let body: ResponseBody = response_body.into();
            let response_res: ResponseResult = response
                .set_body(body)
                .set_status_code(status_code)
                .send(&stream_lock)
                .await;
            return response_res;
        }
        Err(ResponseError::Unknown)
    }

    #[inline]
    pub async fn send_response_once<T: Into<ResponseBody>>(
        &self,
        status_code: usize,
        response_body: T,
    ) -> ResponseResult {
        if let Some(stream_lock) = self.get_stream().await {
            let mut controller_data: RwLockWriteControllerData = self.get_write_lock().await;
            let response: &mut Response = controller_data.get_mut_response();
            let body: ResponseBody = response_body.into();
            let response_res: ResponseResult = response
                .set_body(body)
                .set_status_code(status_code)
                .send(&stream_lock)
                .await;
            let _ = response.close(&stream_lock).await;
            return response_res;
        }
        Err(ResponseError::Unknown)
    }

    #[inline]
    pub async fn send_response_body<T: Into<ResponseBody>>(
        &self,
        response_body: T,
    ) -> ResponseResult {
        let body: ResponseBody = response_body.into();
        let is_websocket: bool = self.get_request_upgrade_type().await.is_websocket();
        let body_list: Vec<ResponseBody> = if is_websocket {
            WebSocketFrame::create_response_frame_list(&body)
        } else {
            vec![body.clone()]
        };
        if let Some(stream_lock) = self.get_stream().await {
            let mut controller_data: RwLockWriteControllerData = self.get_write_lock().await;
            let response: &mut Response = controller_data.get_mut_response();
            for tmp_body in body_list {
                let response_res: ResponseResult =
                    response.set_body(tmp_body).send_body(&stream_lock).await;
                if response_res.is_err() {
                    if is_websocket {
                        response.set_body(body.clone());
                    }
                    return response_res;
                }
            }
            response.set_body(body.clone());
            return Ok(());
        }
        Err(ResponseError::Unknown)
    }

    #[inline]
    pub async fn close(&self) -> ResponseResult {
        if let Some(stream_lock) = self.get_stream().await {
            let mut controller_data: RwLockWriteControllerData = self.get_write_lock().await;
            let response: &mut Response = controller_data.get_mut_response();
            return response.close(&stream_lock).await;
        }
        Err(ResponseError::Unknown)
    }

    #[inline]
    pub async fn log_info<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let log: &Log = controller_data.get_log();
        log.info(data, func);
        self
    }

    #[inline]
    pub async fn log_debug<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let log: &Log = controller_data.get_log();
        log.debug(data, func);
        self
    }

    #[inline]
    pub async fn log_error<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let log: &Log = controller_data.get_log();
        log.error(data, func);
        self
    }

    #[inline]
    pub async fn get_request_method(&self) -> RequestMethod {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let request: &Request = controller_data.get_request();
        request.get_method().clone()
    }

    #[inline]
    pub async fn get_request_host(&self) -> RequestHost {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let request: &Request = controller_data.get_request();
        request.get_host().clone()
    }

    #[inline]
    pub async fn get_request_path(&self) -> RequestPath {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let request: &Request = controller_data.get_request();
        request.get_path().clone()
    }

    #[inline]
    pub async fn get_request_querys(&self) -> RequestQuerys {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let request: &Request = controller_data.get_request();
        request.get_querys().clone()
    }

    #[inline]
    pub async fn get_request_query<T: Into<RequestHeadersKey>>(
        &self,
        key: T,
    ) -> Option<RequestQuerysValue> {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let request: &Request = controller_data.get_request();
        request
            .get_querys()
            .get(&key.into())
            .and_then(|data| Some(data.clone()))
    }

    #[inline]
    pub async fn get_request_body(&self) -> RequestBody {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let request: &Request = controller_data.get_request();
        request.get_body().clone()
    }

    #[inline]
    pub async fn get_request_body_string(&self) -> String {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let request: &Request = controller_data.get_request();
        String::from_utf8_lossy(request.get_body()).to_string()
    }

    #[inline]
    pub async fn get_request_header<K>(&self, key: K) -> Option<RequestHeadersValue>
    where
        K: Into<RequestHeadersKey>,
    {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let request: &Request = controller_data.get_request();
        request.get_header(key)
    }

    #[inline]
    pub async fn get_request_headers(&self) -> RequestHeaders {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let request: &Request = controller_data.get_request();
        request.get_headers().clone()
    }

    #[inline]
    pub async fn get_request_upgrade_type(&self) -> UpgradeType {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let request: &Request = controller_data.get_request();
        request.get_upgrade_type().clone()
    }

    #[inline]
    pub async fn set_request(&self, request_data: Request) -> &Self {
        let mut controller_data: RwLockWriteControllerData = self.get_write_lock().await;
        let request: &mut Request = controller_data.get_mut_request();
        *request = request_data;
        self
    }

    #[inline]
    pub async fn set_request_method<T>(&self, method: T) -> &Self
    where
        T: Into<RequestMethod>,
    {
        let mut controller_data: RwLockWriteControllerData = self.get_write_lock().await;
        let request: &mut Request = controller_data.get_mut_request();
        request.set_method(method);
        self
    }

    #[inline]
    pub async fn set_request_host<T>(&self, host: T) -> &Self
    where
        T: Into<RequestHost>,
    {
        let mut controller_data: RwLockWriteControllerData = self.get_write_lock().await;
        let request: &mut Request = controller_data.get_mut_request();
        request.set_host(host);
        self
    }

    #[inline]
    pub async fn set_request_path<T>(&self, path: T) -> &Self
    where
        T: Into<RequestPath>,
    {
        let mut controller_data: RwLockWriteControllerData = self.get_write_lock().await;
        let request: &mut Request = controller_data.get_mut_request();
        request.set_path(path);
        self
    }

    #[inline]
    pub async fn set_request_query<K, V>(&self, key: K, value: V) -> &Self
    where
        K: Into<RequestQuerysKey>,
        V: Into<RequestQuerysValue>,
    {
        let mut controller_data: RwLockWriteControllerData = self.get_write_lock().await;
        let request: &mut Request = controller_data.get_mut_request();
        request.set_query(key, value);
        self
    }

    #[inline]
    pub async fn set_request_querys<T>(&self, querys: T) -> &Self
    where
        T: Into<RequestQuerys>,
    {
        let mut controller_data: RwLockWriteControllerData = self.get_write_lock().await;
        let request: &mut Request = controller_data.get_mut_request();
        request.set_querys(querys.into());
        self
    }

    #[inline]
    pub async fn set_request_header<K, V>(&self, key: K, value: V) -> &Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        let mut controller_data: RwLockWriteControllerData = self.get_write_lock().await;
        let request: &mut Request = controller_data.get_mut_request();
        request.set_header(key, value);
        self
    }

    #[inline]
    pub async fn set_request_headers(&self, headers: RequestHeaders) -> &Self {
        let mut controller_data: RwLockWriteControllerData = self.get_write_lock().await;
        let request: &mut Request = controller_data.get_mut_request();
        request.set_headers(headers);
        self
    }

    #[inline]
    pub async fn set_request_body<T: Into<RequestBody>>(&self, body: T) -> &Self {
        let mut controller_data: RwLockWriteControllerData = self.get_write_lock().await;
        let request: &mut Request = controller_data.get_mut_request();
        request.set_body(body);
        self
    }

    #[inline]
    pub async fn get_response_headers(&self) -> ResponseHeaders {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let response: &Response = controller_data.get_response();
        response.get_headers().clone()
    }

    #[inline]
    pub async fn get_response_header<K>(&self, key: K) -> Option<ResponseHeadersValue>
    where
        K: Into<ResponseHeadersKey>,
    {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let response: &Response = controller_data.get_response();
        response.get_header(key)
    }

    #[inline]
    pub async fn get_response_body(&self) -> ResponseBody {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let response: &Response = controller_data.get_response();
        response.get_body().clone()
    }

    #[inline]
    pub async fn get_response_body_string(&self) -> String {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let response: &Response = controller_data.get_response();
        String::from_utf8_lossy(response.get_body()).to_string()
    }

    #[inline]
    pub async fn get_response_reason_phrase(&self) -> ResponseReasonPhrase {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let response: &Response = controller_data.get_response();
        response.get_reason_phrase().clone()
    }

    #[inline]
    pub async fn get_response_status_code(&self) -> ResponseStatusCode {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let response: &Response = controller_data.get_response();
        response.get_status_code().clone()
    }

    #[inline]
    pub async fn set_response_header<K, V>(&self, key: K, value: V) -> &Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        let mut controller_data: RwLockWriteControllerData = self.get_write_lock().await;
        let response: &mut Response = controller_data.get_mut_response();
        response.set_header(key, value);
        self
    }

    #[inline]
    pub async fn set_response_headers(&self, headers: ResponseHeaders) -> &Self {
        let mut controller_data: RwLockWriteControllerData = self.get_write_lock().await;
        let response: &mut Response = controller_data.get_mut_response();
        response.set_headers(headers);
        self
    }

    #[inline]
    pub async fn set_response_body<T: Into<ResponseBody>>(&self, body: T) -> &Self {
        let mut controller_data: RwLockWriteControllerData = self.get_write_lock().await;
        let response: &mut Response = controller_data.get_mut_response();
        response.set_body(body);
        self
    }

    #[inline]
    pub async fn set_response_reason_phrase<T: Into<ResponseReasonPhrase>>(
        &self,
        reason_phrase: T,
    ) -> &Self {
        let mut controller_data: RwLockWriteControllerData = self.get_write_lock().await;
        let response: &mut Response = controller_data.get_mut_response();
        response.set_reason_phrase(reason_phrase);
        self
    }

    #[inline]
    pub async fn set_response_status_code(&self, status_code: ResponseStatusCode) -> &Self {
        let mut controller_data: RwLockWriteControllerData = self.get_write_lock().await;
        let response: &mut Response = controller_data.get_mut_response();
        response.set_status_code(status_code);
        self
    }

    #[inline]
    pub async fn judge_enable_keep_alive(&self) -> bool {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        for tem in controller_data.get_request().get_headers().iter() {
            if tem.0.eq_ignore_ascii_case(CONNECTION) {
                if tem.1.eq_ignore_ascii_case(CONNECTION_KEEP_ALIVE) {
                    return true;
                } else if tem.1.eq_ignore_ascii_case(CONNECTION_CLOSE) {
                    return false;
                }
                break;
            }
        }
        let enable_keep_alive: bool = controller_data
            .get_request()
            .get_version()
            .is_http1_1_or_higher();
        return enable_keep_alive;
    }

    #[inline]
    pub async fn judge_unenable_keep_alive(&self) -> bool {
        self.judge_enable_keep_alive().await
    }

    #[inline]
    pub async fn judge_enable_websocket(&self) -> bool {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        for tem in controller_data.get_request().get_headers().iter() {
            if tem.0.eq_ignore_ascii_case(UPGRADE) {
                if tem.1.eq_ignore_ascii_case(WEBSOCKE) {
                    return true;
                }
                break;
            }
        }
        return false;
    }

    #[inline]
    pub(crate) async fn handle_websocket(&self, is_handshake: &mut bool) -> ResponseResult {
        if *is_handshake {
            return Ok(());
        }
        let key_opt: Option<String> = self.get_request_header(SEC_WEBSOCKET_KEY).await;
        if let Some(key) = key_opt {
            let accept_key: String = WebSocketFrame::generate_accept_key(&key);
            return self
                .set_response_header(UPGRADE, WEBSOCKE)
                .await
                .set_response_header(CONNECTION, UPGRADE)
                .await
                .set_response_header(SEC_WEB_SOCKET_ACCEPT, accept_key)
                .await
                .send_response(101, "")
                .await
                .map(|_| {
                    *is_handshake = true;
                });
        }
        Err(ResponseError::WebSocketHandShakeError)
    }
}
