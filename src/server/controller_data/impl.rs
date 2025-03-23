use crate::*;

impl ControllerData {
    #[inline]
    pub(crate) fn from_controller_data(controller_data: InnerControllerData) -> Self {
        Self(arc_rwlock(controller_data))
    }

    #[inline]
    pub async fn get(&self) -> InnerControllerData {
        self.get_read_lock().await.clone()
    }

    #[inline]
    pub async fn get_read_lock(&self) -> RwLockReadInnerControllerData {
        self.0.read().await
    }

    #[inline]
    pub async fn get_write_lock(&self) -> RwLockWriteInnerControllerData {
        self.0.write().await
    }

    #[inline]
    pub async fn get_stream(&self) -> OptionArcRwLockStream {
        self.get_read_lock().await.get_stream().clone()
    }

    #[inline]
    pub async fn get_request(&self) -> Request {
        self.get_read_lock().await.get_request().clone()
    }

    #[inline]
    pub async fn get_response(&self) -> Response {
        self.get_read_lock().await.get_response().clone()
    }

    #[inline]
    pub async fn get_request_string(&self) -> String {
        self.get_read_lock().await.get_request().get_string()
    }

    #[inline]
    pub async fn get_response_string(&self) -> String {
        self.get_read_lock().await.get_response().get_string()
    }

    #[inline]
    pub async fn get_log(&self) -> Log {
        self.get_read_lock().await.get_log().clone()
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
    pub async fn get_socket_addr_or_default(&self) -> SocketAddr {
        let stream_result: OptionArcRwLockStream = self.get_stream().await;
        if stream_result.is_none() {
            return DEFAULT_SOCKET_ADDR;
        }
        let socket_addr: SocketAddr = stream_result
            .unwrap()
            .get_read_lock()
            .await
            .peer_addr()
            .unwrap_or(DEFAULT_SOCKET_ADDR);
        socket_addr
    }

    #[inline]
    pub async fn get_socket_addr_string(&self) -> Option<String> {
        self.get_socket_addr().await.map(|data| data.to_string())
    }

    #[inline]
    pub async fn get_socket_addr_or_default_string(&self) -> String {
        self.get_socket_addr_or_default().await.to_string()
    }

    #[inline]
    pub async fn get_socket_host(&self) -> OptionSocketHost {
        self.get_socket_addr()
            .await
            .map(|socket_addr: SocketAddr| socket_addr.ip())
    }

    #[inline]
    pub async fn get_socket_port(&self) -> OptionSocketPort {
        self.get_socket_addr()
            .await
            .map(|socket_addr: SocketAddr| socket_addr.port())
    }

    #[inline]
    fn inner_is_websocket(&self, controller_data: &RwLockWriteInnerControllerData) -> bool {
        return controller_data
            .get_request()
            .get_upgrade_type()
            .is_websocket();
    }

    #[inline]
    async fn inner_send_response<T: Into<ResponseBody>>(
        &self,
        status_code: usize,
        response_body: T,
        handle_websocket: bool,
    ) -> ResponseResult {
        if let Some(stream_lock) = self.get_stream().await {
            let mut controller_data: RwLockWriteInnerControllerData = self.get_write_lock().await;
            if !handle_websocket && self.inner_is_websocket(&controller_data) {
                return Err(ResponseError::NotSupportUseThisMethod);
            }
            let body: ResponseBody = response_body.into();
            let response_res: ResponseResult = controller_data
                .get_mut_response()
                .set_body(body)
                .set_status_code(status_code)
                .send(&stream_lock)
                .await;
            return response_res;
        }
        Err(ResponseError::NotFoundStream)
    }

    #[inline]
    pub async fn send_response<T: Into<ResponseBody>>(
        &self,
        status_code: usize,
        response_body: T,
    ) -> ResponseResult {
        self.inner_send_response(status_code, response_body, false)
            .await
    }

    #[inline]
    pub async fn send(&self) -> ResponseResult {
        let status_code: ResponseStatusCode = self.get_response_status_code().await;
        let response_body: ResponseBody = self.get_response_body().await;
        self.send_response(status_code, response_body).await
    }

    #[inline]
    pub async fn send_response_once<T: Into<ResponseBody>>(
        &self,
        status_code: usize,
        response_body: T,
    ) -> ResponseResult {
        if let Some(stream_lock) = self.get_stream().await {
            let mut controller_data: RwLockWriteInnerControllerData = self.get_write_lock().await;
            if self.inner_is_websocket(&controller_data) {
                return Err(ResponseError::NotSupportUseThisMethod);
            }
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
        Err(ResponseError::NotFoundStream)
    }

    #[inline]
    pub async fn send_once(&self) -> ResponseResult {
        let status_code: ResponseStatusCode = self.get_response_status_code().await;
        let response_body: ResponseBody = self.get_response_body().await;
        self.send_response_once(status_code, response_body).await
    }

    #[inline]
    pub async fn send_response_body<T: Into<ResponseBody>>(
        &self,
        response_body: T,
    ) -> ResponseResult {
        if let Some(stream_lock) = self.get_stream().await {
            let is_websocket: bool = self.get_request_upgrade_type().await.is_websocket();
            let response_res: ResponseResult = self
                .get_write_lock()
                .await
                .get_mut_response()
                .set_body(response_body)
                .send_body(&stream_lock, is_websocket)
                .await;
            return response_res;
        }
        Err(ResponseError::NotFoundStream)
    }

    #[inline]
    pub async fn send_body(&self) -> ResponseResult {
        let body: ResponseBody = self.get_response_body().await;
        self.send_response_body(body).await
    }

    #[inline]
    pub async fn close(&self) -> ResponseResult {
        if let Some(stream_lock) = self.get_stream().await {
            return self
                .get_write_lock()
                .await
                .get_mut_response()
                .close(&stream_lock)
                .await;
        }
        Err(ResponseError::NotFoundStream)
    }

    #[inline]
    pub async fn flush(&self) -> ResponseResult {
        if let Some(stream_lock) = self.get_stream().await {
            return self
                .get_write_lock()
                .await
                .get_mut_response()
                .flush(&stream_lock)
                .await;
        }
        Err(ResponseError::NotFoundStream)
    }

    #[inline]
    pub async fn log_info<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.get_read_lock().await.get_log().info(data, func);
        self
    }

    #[inline]
    pub async fn log_debug<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.get_read_lock().await.get_log().debug(data, func);
        self
    }

    #[inline]
    pub async fn log_error<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.get_read_lock().await.get_log().error(data, func);
        self
    }

    #[inline]
    pub async fn get_request_method(&self) -> RequestMethod {
        self.get_read_lock()
            .await
            .get_request()
            .get_method()
            .clone()
    }

    #[inline]
    pub async fn get_request_host(&self) -> RequestHost {
        self.get_read_lock().await.get_request().get_host().clone()
    }

    #[inline]
    pub async fn get_request_path(&self) -> RequestPath {
        self.get_read_lock().await.get_request().get_path().clone()
    }

    #[inline]
    pub async fn get_request_querys(&self) -> RequestQuerys {
        self.get_read_lock()
            .await
            .get_request()
            .get_querys()
            .clone()
    }

    #[inline]
    pub async fn get_request_query<T: Into<RequestHeadersKey>>(
        &self,
        key: T,
    ) -> Option<RequestQuerysValue> {
        self.get_read_lock()
            .await
            .get_request()
            .get_querys()
            .get(&key.into())
            .map(|data| data.clone())
    }

    #[inline]
    pub async fn get_request_body(&self) -> RequestBody {
        self.get_read_lock().await.get_request().get_body().clone()
    }

    #[inline]
    pub async fn get_request_body_string(&self) -> String {
        String::from_utf8_lossy(self.get_read_lock().await.get_request().get_body()).to_string()
    }

    #[inline]
    pub async fn get_request_header<K>(&self, key: K) -> Option<RequestHeadersValue>
    where
        K: Into<RequestHeadersKey>,
    {
        self.get_read_lock().await.get_request().get_header(key)
    }

    #[inline]
    pub async fn get_request_headers(&self) -> RequestHeaders {
        self.get_read_lock()
            .await
            .get_request()
            .get_headers()
            .clone()
    }

    #[inline]
    pub async fn get_request_upgrade_type(&self) -> UpgradeType {
        self.get_read_lock()
            .await
            .get_request()
            .get_upgrade_type()
            .clone()
    }

    #[inline]
    pub async fn set_request(&self, request_data: Request) -> &Self {
        *self.get_write_lock().await.get_mut_request() = request_data;
        self
    }

    #[inline]
    pub async fn set_request_method<T>(&self, method: T) -> &Self
    where
        T: Into<RequestMethod>,
    {
        self.get_write_lock()
            .await
            .get_mut_request()
            .set_method(method);
        self
    }

    #[inline]
    pub async fn set_request_host<T>(&self, host: T) -> &Self
    where
        T: Into<RequestHost>,
    {
        self.get_write_lock().await.get_mut_request().set_host(host);
        self
    }

    #[inline]
    pub async fn set_request_path<T>(&self, path: T) -> &Self
    where
        T: Into<RequestPath>,
    {
        self.get_write_lock().await.get_mut_request().set_path(path);
        self
    }

    #[inline]
    pub async fn set_request_query<K, V>(&self, key: K, value: V) -> &Self
    where
        K: Into<RequestQuerysKey>,
        V: Into<RequestQuerysValue>,
    {
        self.get_write_lock()
            .await
            .get_mut_request()
            .set_query(key, value);
        self
    }

    #[inline]
    pub async fn set_request_querys<T>(&self, querys: T) -> &Self
    where
        T: Into<RequestQuerys>,
    {
        self.get_write_lock()
            .await
            .get_mut_request()
            .set_querys(querys.into());
        self
    }

    #[inline]
    pub async fn set_request_header<K, V>(&self, key: K, value: V) -> &Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.get_write_lock()
            .await
            .get_mut_request()
            .set_header(key, value);
        self
    }

    #[inline]
    pub async fn set_request_headers(&self, headers: RequestHeaders) -> &Self {
        self.get_write_lock()
            .await
            .get_mut_request()
            .set_headers(headers);
        self
    }

    #[inline]
    pub async fn set_request_body<T: Into<RequestBody>>(&self, body: T) -> &Self {
        self.get_write_lock().await.get_mut_request().set_body(body);
        self
    }

    #[inline]
    pub async fn get_response_headers(&self) -> ResponseHeaders {
        self.get_read_lock()
            .await
            .get_response()
            .get_headers()
            .clone()
    }

    #[inline]
    pub async fn get_response_header<K>(&self, key: K) -> Option<ResponseHeadersValue>
    where
        K: Into<ResponseHeadersKey>,
    {
        self.get_read_lock().await.get_response().get_header(key)
    }

    #[inline]
    pub async fn get_response_body(&self) -> ResponseBody {
        self.get_read_lock().await.get_response().get_body().clone()
    }

    #[inline]
    pub async fn get_response_body_string(&self) -> String {
        String::from_utf8_lossy(self.get_read_lock().await.get_response().get_body()).to_string()
    }

    #[inline]
    pub async fn get_response_reason_phrase(&self) -> ResponseReasonPhrase {
        self.get_read_lock()
            .await
            .get_response()
            .get_reason_phrase()
            .clone()
    }

    #[inline]
    pub async fn get_response_status_code(&self) -> ResponseStatusCode {
        self.get_read_lock()
            .await
            .get_response()
            .get_status_code()
            .clone()
    }

    #[inline]
    pub async fn set_response_header<K, V>(&self, key: K, value: V) -> &Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.get_write_lock()
            .await
            .get_mut_response()
            .set_header(key, value);
        self
    }

    #[inline]
    pub async fn set_response_headers(&self, headers: ResponseHeaders) -> &Self {
        self.get_write_lock()
            .await
            .get_mut_response()
            .set_headers(headers);
        self
    }

    #[inline]
    pub async fn set_response_body<T: Into<ResponseBody>>(&self, body: T) -> &Self {
        self.get_write_lock()
            .await
            .get_mut_response()
            .set_body(body);
        self
    }

    #[inline]
    pub async fn set_response_reason_phrase<T: Into<ResponseReasonPhrase>>(
        &self,
        reason_phrase: T,
    ) -> &Self {
        self.get_write_lock()
            .await
            .get_mut_response()
            .set_reason_phrase(reason_phrase);
        self
    }

    #[inline]
    pub async fn set_response_status_code(&self, status_code: ResponseStatusCode) -> &Self {
        self.get_write_lock()
            .await
            .get_mut_response()
            .set_status_code(status_code);
        self
    }

    #[inline]
    pub async fn judge_enable_keep_alive(&self) -> bool {
        let controller_data: RwLockReadInnerControllerData = self.get_read_lock().await;
        let headers: &RequestHeaders = controller_data.get_request().get_headers();
        if let Some(enable_keep_alive) = headers.iter().find_map(|tem_header| {
            let (key, value) = tem_header.pair();
            if key.eq_ignore_ascii_case(CONNECTION) {
                if value.eq_ignore_ascii_case(CONNECTION_KEEP_ALIVE) {
                    Some(true)
                } else if value.eq_ignore_ascii_case(CONNECTION_CLOSE) {
                    Some(false)
                } else {
                    None
                }
            } else {
                None
            }
        }) {
            return enable_keep_alive;
        }
        let enable_keep_alive: bool = controller_data
            .get_request()
            .get_version()
            .is_http1_1_or_higher();
        return enable_keep_alive;
    }

    #[inline]
    pub async fn judge_unenable_keep_alive(&self) -> bool {
        !self.judge_enable_keep_alive().await
    }

    #[inline]
    pub async fn judge_enable_websocket(&self) -> bool {
        self.get_read_lock()
            .await
            .get_request()
            .get_headers()
            .iter()
            .any(|tem_header| {
                let (key, value) = tem_header.pair();
                key.eq_ignore_ascii_case(UPGRADE) && value.eq_ignore_ascii_case(WEBSOCKET)
            })
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
                .set_response_header(UPGRADE, WEBSOCKET)
                .await
                .set_response_header(CONNECTION, UPGRADE)
                .await
                .set_response_header(SEC_WEB_SOCKET_ACCEPT, accept_key)
                .await
                .inner_send_response(101, "", true)
                .await
                .map(|_| {
                    *is_handshake = true;
                });
        }
        Err(ResponseError::WebSocketHandShakeError)
    }
}
