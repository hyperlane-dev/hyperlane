use super::*;

/// Inherent implementation of [`H3Framer`].
impl H3Framer {
    /// Creates a framer over one request stream.
    ///
    /// # Arguments
    ///
    /// - `RequestH3Stream` - The request stream to frame onto.
    ///
    pub(crate) fn new(request_stream: RequestH3Stream) -> Self {
        Self {
            request_stream: Some(request_stream),
            finished: false,
        }
    }

    /// Resets the real stream after the framing pump failed.
    ///
    /// Dropping a half written stream without a reset leaves the peer waiting
    /// for the frames that will never arrive, so the failure is reported to the
    /// client instead of being swallowed. The stream is dropped here, which is
    /// what stops this task from holding the request open.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The `value` argument.
    ///
    pub(crate) fn reset(&mut self) {
        let Some(mut request_stream) = self.get_mut_request_stream().take() else {
            return;
        };
        request_stream.stop_stream(Code::H3_INTERNAL_ERROR);
    }
}

/// Implements [`ResponseFramer`] for [`H3Framer`].
impl ResponseFramer for H3Framer {
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
        _bodyless: bool,
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
            let response: ProtocolResponse<()> = builder
                .body(())
                .map_err(|error: ProtocolBuildError| ResponseError::Send(error.to_string()))?;
            let Some(mut request_stream) = self.get_mut_request_stream().take() else {
                return Err(ResponseError::Terminated);
            };
            request_stream
                .send_response(response)
                .await
                .map_err(|error: h3::error::StreamError| ResponseError::Send(error.to_string()))?;
            self.set_request_stream(Some(request_stream));
            Ok(())
        })
    }

    /// Sends one body chunk as a data frame.
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
            let Some(mut request_stream) = self.get_mut_request_stream().take() else {
                return Err(ResponseError::Terminated);
            };
            request_stream
                .send_data(Bytes::copy_from_slice(data))
                .await
                .map_err(|error: h3::error::StreamError| ResponseError::Send(error.to_string()))?;
            self.set_request_stream(Some(request_stream));
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
            let Some(mut request_stream) = self.get_mut_request_stream().take() else {
                return Ok(());
            };
            request_stream
                .finish()
                .await
                .map_err(|error: h3::error::StreamError| ResponseError::Send(error.to_string()))?;
            Ok(())
        })
    }
}
