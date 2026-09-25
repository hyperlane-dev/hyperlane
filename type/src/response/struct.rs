use super::*;

/// Represents a parsed HTTP response.
#[derive(
    Clone, Debug, Deserialize, DisplayDebug, Eq, Getter, GetterMut, PartialEq, Serialize, Setter,
)]
pub struct Response {
    /// The HTTP version used in the response.
    pub version: ResponseVersion,
    /// The HTTP status code.
    #[get(type(copy))]
    pub status_code: ResponseStatusCode,
    /// The reason phrase associated with the status code.
    #[set(type(AsRef<str>))]
    pub reason_phrase: ResponseReasonPhrase,
    /// The response headers as key-value pairs.
    pub headers: ResponseHeaders,
    /// The binary body content of the response.
    #[set(type(AsRef<[u8]>))]
    pub body: ResponseBody,
}
