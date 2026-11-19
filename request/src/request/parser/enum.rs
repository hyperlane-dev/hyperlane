/// The boundary of the final HTTP/1 response body.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ResponseFraming {
    /// The exact number of bytes following the field section.
    Fixed(usize),
    /// The body ends after the terminating chunk and trailer section.
    Chunked,
    /// The body ends when the transport reaches EOF.
    Close,
}
