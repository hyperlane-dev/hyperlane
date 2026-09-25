use super::*;

/// A marker type representing HTTP protocol variants.
///
/// This struct serves as a namespace for protocol-related utility functions,
/// providing methods to identify HTTP and HTTPS protocols and retrieve
/// their default port numbers.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, New, PartialEq, Serialize)]
pub struct Protocol;
