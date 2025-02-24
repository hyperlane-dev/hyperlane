use crate::*;

impl ControllerData {
    #[inline]
    pub fn new() -> Self {
        ControllerData {
            stream: None,
            request: Request::default(),
            response: Response::default(),
            log: Log::default(),
        }
    }
}

impl ArcRwLockControllerData {
    #[inline]
    pub(crate) fn from_controller_data(controller_data: ControllerData) -> Self {
        Self(Arc::new(RwLock::new(controller_data)))
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
    pub async fn get_controller_data(&self) -> ControllerData {
        let controller_data: ControllerData = self.get_read_lock().await.clone();
        controller_data
    }

    #[inline]
    pub async fn get_stream(&self) -> OptionArcRwLockStream {
        let controller_data: ControllerData = self.get_controller_data().await;
        controller_data.get_stream().clone()
    }

    #[inline]
    pub async fn get_request(&self) -> Request {
        let controller_data: ControllerData = self.get_controller_data().await;
        controller_data.get_request().clone()
    }

    #[inline]
    pub async fn get_response(&self) -> Response {
        let controller_data: ControllerData = self.get_controller_data().await;
        controller_data.get_response().clone()
    }

    #[inline]
    pub async fn get_log(&self) -> Log {
        let controller_data: ControllerData = self.get_controller_data().await;
        controller_data.get_log().clone()
    }

    #[inline]
    pub async fn get_socket_addr(&self) -> Option<String> {
        let stream_result: OptionArcRwLockStream = self.get_stream().await;
        if stream_result.is_none() {
            return None;
        }
        let socket_addr: String = stream_result
            .unwrap()
            .get_read_lock()
            .await
            .peer_addr()
            .and_then(|host| Ok(host.to_string()))
            .unwrap_or_default();
        Some(socket_addr)
    }

    #[inline]
    pub async fn send_response<T: Into<ResponseBody>>(
        &self,
        status_code: usize,
        response_body: T,
    ) -> ResponseResult {
        let controller_data: RwLockWriteControllerData = self.get_write_lock().await;
        let mut response: Response = controller_data.get_response().clone();
        let body: ResponseBody = response_body.into();
        let stream_lock: ArcRwLockStream = controller_data.get_stream().clone().unwrap();
        let response_res: ResponseResult = response
            .set_body(body)
            .set_status_code(status_code)
            .send(&stream_lock)
            .await;
        response_res
    }

    #[inline]
    pub async fn send_response_once<T: Into<ResponseBody>>(
        &self,
        status_code: usize,
        response_body: T,
    ) -> ResponseResult {
        let controller_data: RwLockWriteControllerData = self.get_write_lock().await;
        let mut response: Response = controller_data.get_response().clone();
        let body: ResponseBody = response_body.into();
        let stream_lock: ArcRwLockStream = controller_data.get_stream().clone().unwrap();
        let response_res: ResponseResult = response
            .set_body(body)
            .set_status_code(status_code)
            .send(&stream_lock)
            .await;
        let _ = response.close(&stream_lock).await;
        response_res
    }

    #[inline]
    pub async fn send_response_body<T: Into<ResponseBody>>(
        &self,
        response_body: T,
    ) -> ResponseResult {
        let controller_data: RwLockWriteControllerData = self.get_write_lock().await;
        let mut response: Response = controller_data.get_response().clone();
        let body: ResponseBody = response_body.into();
        let stream_lock: ArcRwLockStream = controller_data.get_stream().clone().unwrap();
        let response_res: ResponseResult = response.set_body(body).send_body(&stream_lock).await;
        response_res
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
    pub async fn get_request_query(&self) -> RequestQuery {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let request: &Request = controller_data.get_request();
        request.get_query().clone()
    }

    #[inline]
    pub async fn get_request_header(&self, key: &str) -> Option<String> {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let request: &Request = controller_data.get_request();
        request
            .get_headers()
            .get(key)
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
    pub async fn get_request_headers(&self) -> RequestHeaders {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let request: &Request = controller_data.get_request();
        request.get_headers().clone()
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
        K: Into<RequestQueryKey>,
        V: Into<RequestQueryValue>,
    {
        let mut controller_data: RwLockWriteControllerData = self.get_write_lock().await;
        let request: &mut Request = controller_data.get_mut_request();
        let mut query: RequestQuery = request.get_query().clone();
        query.insert(key.into(), value.into());
        request.set_query(query);
        self
    }

    #[inline]
    pub async fn set_request_querys<T>(&self, query: T) -> &Self
    where
        T: Into<RequestQuery>,
    {
        let mut controller_data: RwLockWriteControllerData = self.get_write_lock().await;
        let request: &mut Request = controller_data.get_mut_request();
        request.set_query(query);
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
    pub async fn get_response_header(&self, key: &str) -> Option<String> {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let response: &Response = controller_data.get_response();
        response
            .get_headers()
            .get(key)
            .and_then(|data| Some(data.clone()))
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
    pub async fn close(&self) -> ResponseResult {
        let controller_data: RwLockWriteControllerData = self.get_write_lock().await;
        let mut response: Response = controller_data.get_response().clone();
        let stream_lock: ArcRwLockStream = controller_data.get_stream().clone().unwrap();
        response.close(&stream_lock).await
    }
}
