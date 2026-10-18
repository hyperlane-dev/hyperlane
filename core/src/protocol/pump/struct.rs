use super::*;

/// A response head that has been parsed and is ready to be framed.
#[derive(Clone, Debug, DisplayDebug)]
pub(crate) struct FramedHead {
    /// The response status code.
    pub(crate) status_code: usize,
    /// The protocol headers to send as the first frame of the stream.
    pub(crate) headers: ProtocolHeaders,
}
