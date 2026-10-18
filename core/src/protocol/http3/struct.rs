use super::*;

/// Frames a response onto an HTTP/3 request stream as the pump produces it.
///
/// The framing code moves the QUIC stream out of the field with `take` before
/// it can write on it, so the stream field is reached through its mutable
/// accessor rather than a shared one.
#[derive(Data)]
pub(crate) struct H3Framer {
    /// The request stream the frames are written to.
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) request_stream: Option<RequestH3Stream>,
    /// Whether the end of stream has already been sent.
    #[get(type(copy), pub(crate))]
    #[get_mut(skip)]
    #[set(pub(crate))]
    pub(super) finished: bool,
}
