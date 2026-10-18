use super::*;

/// Frames a response onto an HTTP/2 request stream as the pump produces it.
///
/// The framing code moves a stream half out of the field with `take` before it
/// can write on it, so both stream fields are reached through their mutable
/// accessor rather than a shared one.
#[derive(Data)]
pub(crate) struct H2Framer {
    /// The response half, used only for the head.
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) send_response: Option<h2::server::SendResponse<Bytes>>,
    /// The stream half, used for every data frame.
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) send_stream: Option<h2::SendStream<Bytes>>,
    /// Whether the end of stream has already been sent.
    #[get(type(copy), pub(crate))]
    #[get_mut(skip)]
    #[set(pub(crate))]
    pub(super) finished: bool,
}
