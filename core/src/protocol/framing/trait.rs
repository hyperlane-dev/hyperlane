use super::*;

/// The write side of a multiplexed protocol stream, as the pump sees it.
///
/// The pump is protocol agnostic: it decides when the head is complete and
/// which bytes are body, and the implementation decides how that becomes
/// frames. HTTP/2 and HTTP/3 differ in how a data chunk is framed and how the
/// stream ends, but not in when those points occur.
///
/// The pump never calls `send_data` for a response that forbids a body, so an
/// implementation does not have to re-derive the `HEAD` and no-content rule.
/// A head arrives exactly once and always before any data frame, because the
/// pump withholds the frames until the head has been validated; an
/// implementation may therefore assume the order it sees.
///
/// The frame types are boxed so the futures stay small enough to be spawned
/// without each implementation erasing into a large one.
pub(crate) trait ResponseFramer {
    /// Sends the response head as the first frame of the stream.
    ///
    /// Called once per stream, before any data frame. The head carries no
    /// hop by hop header the protocol may not carry and no length field on a
    /// response that has no body; the pump already removed both.
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
    ) -> Pin<Box<dyn Future<Output = Result<(), ResponseError>> + Send + 'a>>;

    /// Sends one body chunk as a data frame.
    ///
    /// Never called with an empty slice, and never called for a response that
    /// forbids a body. The bytes are the payload the handler produced, already
    /// free of any framing bytes.
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
    ) -> Pin<Box<dyn Future<Output = Result<(), ResponseError>> + Send + 'a>>;

    /// Closes the stream with the end of stream flag.
    ///
    /// Reached only when the body ended and matched the length the head
    /// announced. A response that was truncated or overlong fails the pump
    /// without calling this, so the stream is left unfinished for the caller to
    /// reset.
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
    ) -> Pin<Box<dyn Future<Output = Result<(), ResponseError>> + Send + 'a>>;
}
