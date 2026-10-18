use super::*;

impl H2Framer {
    /// Creates a framer over one request stream.
    ///
    /// # Arguments
    ///
    /// - `h2::server::SendResponse<Bytes>` - The response half of the stream.
    ///
    pub(crate) fn new(send_response: h2::server::SendResponse<Bytes>) -> Self {
        Self {
            send_response: Some(send_response),
            send_stream: None,
            finished: false,
        }
    }

    /// Resets the real stream after the framing pump failed.
    ///
    /// Dropping a half written stream without a reset leaves the peer waiting
    /// for the frames that will never arrive, so the failure is reported to the
    /// client instead of being swallowed. The reset is queued rather than
    /// awaited: polling it would wait for the peer to acknowledge a stream the
    /// peer may already have abandoned, and this task must not linger.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The `value` argument.
    ///
    pub(crate) fn reset(&mut self) {
        if let Some(mut send_response) = self.get_mut_send_response().take() {
            send_response.send_reset(h2::Reason::INTERNAL_ERROR);
        }
        if let Some(mut send_stream) = self.get_mut_send_stream().take() {
            send_stream.send_reset(h2::Reason::INTERNAL_ERROR);
        }
    }
}

impl ResponseFramer for H2Framer {
    /// Sends the response head as the first HEADERS frame of the stream.
    ///
    /// # Arguments
    ///
    /// - `&'a mut self` - The `value` argument.
    /// - `&'a FramedHead` - The parsed head to frame.
    /// - `bool` - Whether the response forbids a body.
    ///
    /// # Returns
    ///
    /// - `Pin<Box<dyn Future<Output = Result<(), ResponseError>> + Send + 'a>>` - Resolves once the operation has completed.
    fn send_head<'a>(
        &'a mut self,
        head: &'a FramedHead,
        bodyless: bool,
    ) -> Pin<Box<dyn Future<Output = Result<(), ResponseError>> + Send + 'a>> {
        Box::pin(async move {
            let nominated: Vec<String> = connection_nominated_headers(&head.headers);
            let mut builder: http::response::Builder =
                http::Response::builder().status(head.status_code as u16);
            for (name, value) in head.headers.iter() {
                let key: &str = name.as_str();
                let nominated_by_connection: bool = nominated
                    .iter()
                    .any(|nominated: &String| nominated.as_str() == key);
                if key.starts_with(PSEUDO_HEADER_PREFIX)
                    || is_hop_by_hop(key)
                    || nominated_by_connection
                {
                    continue;
                }
                if let Ok(value) = http::HeaderValue::from_bytes(value.as_bytes()) {
                    builder = builder.header(name.clone(), value);
                }
            }
            let response: http::Response<()> = builder
                .body(())
                .map_err(|error: http::Error| ResponseError::Send(error.to_string()))?;
            let mut send_response: h2::server::SendResponse<Bytes> = self
                .get_mut_send_response()
                .take()
                .ok_or(ResponseError::Terminated)?;
            if bodyless {
                self.set_finished(true);
                send_response
                    .send_response(response, true)
                    .map_err(|error: h2::Error| ResponseError::Send(error.to_string()))?;
                return Ok(());
            }
            let send_stream: h2::SendStream<Bytes> =
                send_response
                    .send_response(response, false)
                    .map_err(|error: h2::Error| ResponseError::Send(error.to_string()))?;
            self.set_send_stream(Some(send_stream));
            Ok(())
        })
    }

    /// Sends one body chunk as a data frame sized by the granted capacity.
    ///
    /// # Arguments
    ///
    /// - `&'a mut self` - The `value` argument.
    /// - `&'a [u8]` - The body bytes to frame.
    ///
    /// # Returns
    ///
    /// - `Pin<Box<dyn Future<Output = Result<(), ResponseError>> + Send + 'a>>` - Resolves once the operation has completed.
    fn send_data<'a>(
        &'a mut self,
        data: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = Result<(), ResponseError>> + Send + 'a>> {
        Box::pin(async move {
            if data.is_empty() || self.get_finished() {
                return Ok(());
            }
            let Some(mut send_stream) = self.get_mut_send_stream().take() else {
                return Err(ResponseError::Terminated);
            };
            let mut rest: &[u8] = data;
            while !rest.is_empty() {
                let granted: usize = next_h2_capacity(&mut send_stream, rest.len()).await?;
                let take: usize = granted.min(rest.len());
                let chunk: Bytes = Bytes::copy_from_slice(&rest[..take]);
                rest = &rest[take..];
                send_stream
                    .send_data(chunk, false)
                    .map_err(|error: h2::Error| ResponseError::Send(error.to_string()))?;
            }
            self.set_send_stream(Some(send_stream));
            Ok(())
        })
    }

    /// Closes the stream with the end of stream flag.
    ///
    /// # Arguments
    ///
    /// - `&'a mut self` - The `value` argument.
    ///
    /// # Returns
    ///
    /// - `Pin<Box<dyn Future<Output = Result<(), ResponseError>> + Send + 'a>>` - Resolves once the operation has completed.
    fn finish<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<(), ResponseError>> + Send + 'a>> {
        Box::pin(async move {
            if self.get_finished() {
                return Ok(());
            }
            self.set_finished(true);
            let Some(mut send_stream) = self.get_mut_send_stream().take() else {
                return Ok(());
            };
            send_stream
                .send_data(Bytes::new(), true)
                .map_err(|error: h2::Error| ResponseError::Send(error.to_string()))?;
            Ok(())
        })
    }
}
