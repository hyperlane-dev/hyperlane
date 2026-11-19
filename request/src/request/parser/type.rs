use super::*;

/// The end offset, body framing, and redirect location of a final response head.
pub(crate) type FinalResponseHead = (usize, ResponseFraming, Option<Vec<u8>>);

/// The body framing and optional redirect location of the final response.
pub(crate) type ResponseHeadFields = (ResponseFraming, Option<Vec<u8>>);
