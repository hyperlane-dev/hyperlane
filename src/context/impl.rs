use crate::*;

impl Context {
    pub(crate) fn from_internal_context(ctx: InnerContext) -> Self {
        Self(arc_rwlock(ctx))
    }

    pub(crate) fn create_context(stream: &ArcRwLockStream, request: &Request) -> Context {
        let mut internal_ctx: InnerContext = InnerContext::default();
        internal_ctx
            .set_stream(Some(stream.clone()))
            .set_request(request.clone());
        let ctx: Context = Context::from_internal_context(internal_ctx);
        ctx
    }

    async fn read(&self) -> RwLockReadInnerContext {
        self.0.read().await
    }

    async fn write(&self) -> RwLockWriteInnerContext {
        self.0.write().await
    }

    pub async fn get_stream(&self) -> OptionArcRwLockStream {
        self.read().await.get_stream().clone()
    }

    pub async fn get_request(&self) -> Request {
        self.read().await.get_request().clone()
    }

    pub async fn get_response(&self) -> Response {
        self.read().await.get_response().clone()
    }

    pub async fn get_request_string(&self) -> String {
        self.read().await.get_request().get_string()
    }

    pub async fn get_response_string(&self) -> String {
        self.read().await.get_response().get_string()
    }

    pub async fn get_socket_addr(&self) -> OptionSocketAddr {
        let stream_result: OptionArcRwLockStream = self.get_stream().await;
        if stream_result.is_none() {
            return None;
        }
        let socket_addr_opt: OptionSocketAddr =
            stream_result.unwrap().read().await.peer_addr().ok();
        socket_addr_opt
    }

    pub async fn get_socket_addr_or_default(&self) -> SocketAddr {
        let stream_result: OptionArcRwLockStream = self.get_stream().await;
        if stream_result.is_none() {
            return DEFAULT_SOCKET_ADDR;
        }
        let socket_addr: SocketAddr = stream_result
            .unwrap()
            .read()
            .await
            .peer_addr()
            .unwrap_or(DEFAULT_SOCKET_ADDR);
        socket_addr
    }

    pub async fn get_socket_addr_string(&self) -> OptionString {
        self.get_socket_addr().await.map(|data| data.to_string())
    }

    pub async fn get_socket_addr_or_default_string(&self) -> String {
        self.get_socket_addr_or_default().await.to_string()
    }

    pub async fn get_socket_host(&self) -> OptionSocketHost {
        self.get_socket_addr()
            .await
            .map(|socket_addr: SocketAddr| socket_addr.ip())
    }

    pub async fn get_route_params(&self) -> RouteParams {
        self.read().await.get_route_params().clone()
    }

    pub async fn get_route_param(&self, name: &str) -> OptionString {
        self.read().await.get_route_params().get(name).cloned()
    }

    pub(crate) async fn set_route_params(&self, params: RouteParams) -> &Self {
        self.write().await.set_route_params(params);
        self
    }

    pub async fn get_socket_port(&self) -> OptionSocketPort {
        self.get_socket_addr()
            .await
            .map(|socket_addr: SocketAddr| socket_addr.port())
    }

    async fn internal_send_handler(&self, upgrade_ws: bool) -> ResponseResult {
        if self.get_closed().await {
            return Err(ResponseError::ConnectionClosed);
        }
        if let Some(stream) = self.get_stream().await {
            let is_ws: bool = self.get_request_upgrade_type().await.is_ws();
            if !upgrade_ws && is_ws {
                return Err(ResponseError::MethodNotSupported(
                    "websocket does not support calling this method".to_owned(),
                ));
            }
            let response_res: ResponseData = self.write().await.get_mut_response().build();
            return stream.send(&response_res).await;
        }
        Err(ResponseError::NotFoundStream)
    }

    pub async fn send(&self) -> ResponseResult {
        self.internal_send_handler(false).await
    }

    pub async fn send_once(&self) -> ResponseResult {
        let res: ResponseResult = self.send().await;
        self.closed().await;
        res
    }

    pub async fn send_body(&self) -> ResponseResult {
        if self.get_closed().await {
            return Err(ResponseError::ConnectionClosed);
        }
        if let Some(stream) = self.get_stream().await {
            let is_ws: bool = self.get_request_upgrade_type().await.is_ws();
            let response_body: ResponseBody = self.get_response_body().await;
            return stream.send_body_conditional(&response_body, is_ws).await;
        }
        Err(ResponseError::NotFoundStream)
    }

    pub async fn send_once_body(&self) -> ResponseResult {
        let res: ResponseResult = self.send_body().await;
        self.closed().await;
        res
    }

    pub async fn flush(&self) -> ResponseResult {
        if let Some(stream) = self.get_stream().await {
            stream.flush().await;
            return Ok(());
        }
        Err(ResponseError::NotFoundStream)
    }

    pub async fn get_request_method(&self) -> RequestMethod {
        self.read().await.get_request().get_method().clone()
    }

    pub async fn get_request_host(&self) -> RequestHost {
        self.read().await.get_request().get_host().clone()
    }

    pub async fn get_request_path(&self) -> RequestPath {
        self.read().await.get_request().get_path().clone()
    }

    pub async fn get_request_querys(&self) -> RequestQuerys {
        self.read().await.get_request().get_querys().clone()
    }

    pub async fn get_request_query<T>(&self, key: T) -> OptionRequestQuerysValue
    where
        T: Into<RequestHeadersKey>,
    {
        self.read().await.get_request().get_query(key)
    }

    pub async fn get_request_body(&self) -> RequestBody {
        self.read().await.get_request().get_body().clone()
    }

    pub async fn get_request_body_string(&self) -> String {
        self.read().await.get_request().get_body_string()
    }

    pub async fn get_request_body_json<T>(&self) -> ResultJsonError<T>
    where
        T: DeserializeOwned,
    {
        self.read().await.get_request().get_body_json()
    }

    pub async fn get_request_header<K>(&self, key: K) -> OptionRequestHeadersValue
    where
        K: Into<RequestHeadersKey>,
    {
        self.read().await.get_request().get_header(key)
    }

    pub async fn get_request_headers(&self) -> RequestHeaders {
        self.read().await.get_request().get_headers().clone()
    }

    pub async fn get_request_header_front<K>(&self, key: K) -> OptionRequestHeadersValueItem
    where
        K: Into<RequestHeadersKey>,
    {
        self.read().await.get_request().get_header_front(key)
    }

    pub async fn get_request_header_back<K>(&self, key: K) -> OptionRequestHeadersValueItem
    where
        K: Into<RequestHeadersKey>,
    {
        self.read().await.get_request().get_header_back(key)
    }

    pub async fn get_request_header_len<K>(&self, key: K) -> usize
    where
        K: Into<RequestHeadersKey>,
    {
        self.read().await.get_request().get_header_len(key)
    }

    pub async fn get_request_headers_values_len(&self) -> usize {
        self.read().await.get_request().get_headers_values_len()
    }

    pub async fn get_request_headers_len(&self) -> usize {
        self.read().await.get_request().get_headers_len()
    }

    pub async fn has_request_header<K>(&self, key: K) -> bool
    where
        K: Into<RequestHeadersKey>,
    {
        self.read().await.get_request().has_header(key)
    }

    pub async fn has_request_header_value<K, V>(&self, key: K, value: V) -> bool
    where
        K: Into<RequestHeadersKey>,
        V: Into<RequestHeadersValueItem>,
    {
        self.read().await.get_request().has_header_value(key, value)
    }

    pub async fn get_request_cookies(&self) -> Cookies {
        self.get_request_header_front(COOKIE)
            .await
            .map(|data| Cookie::parse(&data))
            .unwrap_or_default()
    }

    pub async fn get_request_cookie<K>(&self, key: K) -> OptionCookiesValue
    where
        K: Into<CookieKey>,
    {
        self.get_request_cookies().await.get(&key.into()).cloned()
    }

    pub async fn get_request_upgrade_type(&self) -> UpgradeType {
        self.read().await.get_request().get_upgrade_type()
    }

    async fn set_request(&self, request_data: &Request) -> &Self {
        self.write().await.set_request(request_data.clone());
        self
    }

    pub async fn get_response_version(&self) -> ResponseVersion {
        self.read().await.get_response().get_version().clone()
    }

    pub async fn get_response_headers(&self) -> ResponseHeaders {
        self.read().await.get_response().get_headers().clone()
    }

    pub async fn get_response_header<K>(&self, key: K) -> OptionResponseHeadersValue
    where
        K: Into<ResponseHeadersKey>,
    {
        self.read().await.get_response().get_header(key)
    }

    pub async fn get_response_header_front<K>(&self, key: K) -> OptionResponseHeadersValueItem
    where
        K: Into<ResponseHeadersKey>,
    {
        self.read().await.get_response().get_header_front(key)
    }

    pub async fn get_response_header_back<K>(&self, key: K) -> OptionResponseHeadersValueItem
    where
        K: Into<ResponseHeadersKey>,
    {
        self.read().await.get_response().get_header_back(key)
    }

    pub async fn get_response_has_header<K>(&self, key: K) -> bool
    where
        K: Into<ResponseHeadersKey>,
    {
        self.read().await.get_response().has_header(key)
    }

    pub async fn has_response_header_value<K, V>(&self, key: K, value: V) -> bool
    where
        K: Into<ResponseHeadersKey>,
        V: Into<ResponseHeadersValueItem>,
    {
        self.read()
            .await
            .get_response()
            .has_header_value(key, value)
    }

    pub async fn get_response_headers_len(&self) -> usize {
        self.read().await.get_response().get_headers_len()
    }

    pub async fn get_response_header_len<K>(&self, key: K) -> usize
    where
        K: Into<ResponseHeadersKey>,
    {
        self.read().await.get_response().get_header_len(key)
    }

    pub async fn get_response_headers_values_len(&self) -> usize {
        self.read().await.get_response().get_headers_values_len()
    }

    pub async fn get_response_cookies(&self) -> Cookies {
        self.get_response_header_front(COOKIE)
            .await
            .map(|data| Cookie::parse(&data))
            .unwrap_or_default()
    }

    pub async fn get_response_cookie<K>(&self, key: K) -> OptionCookiesValue
    where
        K: Into<CookieKey>,
    {
        self.get_response_cookies().await.get(&key.into()).cloned()
    }

    pub async fn get_response_body(&self) -> ResponseBody {
        self.read().await.get_response().get_body().clone()
    }

    pub async fn get_response_body_string(&self) -> String {
        self.read().await.get_response().get_body_string()
    }

    pub async fn get_response_body_json<T>(&self) -> ResultJsonError<T>
    where
        T: DeserializeOwned,
    {
        self.read().await.get_response().get_body_json()
    }

    pub async fn get_response_reason_phrase(&self) -> ResponseReasonPhrase {
        self.read().await.get_response().get_reason_phrase().clone()
    }

    pub async fn get_response_status_code(&self) -> ResponseStatusCode {
        self.read().await.get_response().get_status_code().clone()
    }

    pub async fn set_response(&self, response: Response) -> &Self {
        self.write().await.set_response(response);
        self
    }

    pub async fn set_response_version(&self, version: ResponseVersion) -> &Self {
        self.write().await.get_mut_response().set_version(version);
        self
    }

    pub async fn set_response_header<K, V>(&self, key: K, value: V) -> &Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.write().await.get_mut_response().set_header(key, value);
        self
    }

    pub async fn replace_response_header<K, V>(&self, key: K, value: V) -> &Self
    where
        K: Into<ResponseHeadersKey>,
        V: Into<String>,
    {
        self.write()
            .await
            .get_mut_response()
            .replace_header(key, value);
        self
    }

    pub async fn remove_response_header<K>(&self, key: K) -> &Self
    where
        K: Into<ResponseHeadersKey>,
    {
        self.write().await.get_mut_response().remove_header(key);
        self
    }

    pub async fn remove_response_header_value<K, V>(&self, key: K, value: V) -> &Self
    where
        K: Into<ResponseHeadersKey>,
        V: Into<String>,
    {
        self.write()
            .await
            .get_mut_response()
            .remove_header_value(key, value);
        self
    }

    pub async fn clear_response_headers(&self) -> &Self {
        self.write().await.get_mut_response().clear_headers();
        self
    }

    pub async fn set_response_body<T>(&self, body: T) -> &Self
    where
        T: Into<ResponseBody>,
    {
        self.write().await.get_mut_response().set_body(body);
        self
    }

    pub async fn set_response_reason_phrase<T>(&self, reason_phrase: T) -> &Self
    where
        T: Into<ResponseReasonPhrase>,
    {
        self.write()
            .await
            .get_mut_response()
            .set_reason_phrase(reason_phrase);
        self
    }

    pub async fn set_response_status_code(&self, status_code: ResponseStatusCode) -> &Self {
        self.write()
            .await
            .get_mut_response()
            .set_status_code(status_code);
        self
    }

    pub async fn is_enable_keep_alive(&self) -> bool {
        self.get_request().await.is_enable_keep_alive()
    }

    pub async fn upgrade_to_ws(&self) -> ResponseResult {
        if let Some(key) = &self.get_request_header_back(SEC_WEBSOCKET_KEY).await {
            let accept_key: String = WebSocketFrame::generate_accept_key(key);
            return self
                .set_response_status_code(101)
                .await
                .set_response_header(UPGRADE, WEBSOCKET)
                .await
                .set_response_header(CONNECTION, UPGRADE)
                .await
                .set_response_header(SEC_WEBSOCKET_ACCEPT, accept_key)
                .await
                .internal_send_handler(true)
                .await;
        }
        Err(ResponseError::WebSocketHandShake(format!(
            "missing {} header",
            SEC_WEBSOCKET_KEY
        )))
    }

    pub async fn set_attribute<T>(&self, key: &str, value: T) -> &Self
    where
        T: AnySendSyncClone,
    {
        self.write()
            .await
            .get_mut_attributes()
            .insert(key.to_owned(), Arc::new(value));
        self
    }

    pub async fn get_attributes(&self) -> HashMapArcAnySendSync {
        self.read().await.get_attributes().clone()
    }

    pub async fn get_attribute<T>(&self, key: &str) -> Option<T>
    where
        T: AnySendSyncClone,
    {
        self.read()
            .await
            .get_attributes()
            .get(key)
            .and_then(|arc| arc.downcast_ref::<T>())
            .cloned()
    }

    pub async fn remove_attribute(&self, key: &str) -> &Self {
        self.write().await.get_mut_attributes().remove(key);
        self
    }

    pub async fn clear_attribute(&self) -> &Self {
        self.write().await.get_mut_attributes().clear();
        self
    }

    pub async fn get_aborted(&self) -> bool {
        *self.write().await.get_aborted()
    }

    pub async fn set_aborted(&self, aborted: bool) -> &Self {
        self.write().await.set_aborted(aborted);
        self
    }

    pub async fn aborted(&self) -> &Self {
        self.set_aborted(true).await;
        self
    }

    pub async fn cancel_aborted(&self) -> &Self {
        self.set_aborted(false).await;
        self
    }

    pub async fn get_closed(&self) -> bool {
        *self.write().await.get_closed()
    }

    pub async fn set_closed(&self, closed: bool) -> &Self {
        self.write().await.set_closed(closed);
        self
    }

    pub async fn closed(&self) -> &Self {
        self.set_closed(true).await;
        self
    }

    pub async fn cancel_closed(&self) -> &Self {
        self.set_closed(false).await;
        self
    }

    pub async fn reset_response_body(&self) -> &Self {
        self.set_response_body(ResponseBody::default()).await;
        self
    }

    pub async fn http_from_stream(&self, buffer_size: usize) -> RequestReaderHandleResult {
        self.reset_response_body().await;
        if self.get_aborted().await {
            return Err(RequestError::RequestAborted);
        }
        if let Some(stream) = self.get_stream().await.as_ref() {
            let request_res: RequestReaderHandleResult =
                Request::http_from_stream(stream, buffer_size).await;
            if let Ok(request) = request_res.as_ref() {
                self.set_request(request).await;
            }
            return request_res;
        };
        Err(RequestError::GetTcpStream)
    }

    pub async fn ws_from_stream(&self, buffer_size: usize) -> RequestReaderHandleResult {
        self.reset_response_body().await;
        if self.get_aborted().await {
            return Err(RequestError::RequestAborted);
        }
        if let Some(stream) = self.get_stream().await.as_ref() {
            let mut last_request: Request = self.get_request().await;
            let request_res: RequestReaderHandleResult =
                Request::ws_from_stream(stream, buffer_size, &mut last_request).await;
            match request_res.as_ref() {
                Ok(request) => {
                    self.set_request(&request).await;
                }
                Err(_) => {
                    self.set_request(&last_request).await;
                }
            }
            return request_res;
        };
        Err(RequestError::GetTcpStream)
    }

    pub(crate) async fn should_abort(&self, lifecycle: &mut Lifecycle) {
        let keep_alive: bool = !self.get_closed().await
            && matches!(
                lifecycle,
                Lifecycle::Continue(true) | Lifecycle::Abort(true)
            );
        *lifecycle = if self.get_aborted().await {
            Lifecycle::Abort(keep_alive)
        } else {
            Lifecycle::Continue(keep_alive)
        };
    }
}
