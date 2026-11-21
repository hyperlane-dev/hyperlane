use super::*;

/// Represents a parsed HTTP response.
///
/// The fields are crate-internal (§17.18): a consumer reaches them through
/// the generated `get_*` and `set_*` accessors.
#[derive(Clone, Data, Debug, Deserialize, DisplayDebug, Eq, PartialEq, Serialize)]
pub struct Response {
    /// The HTTP version used in the response.
    pub(crate) version: ResponseVersion,
    /// The HTTP status code.
    #[get(type(copy))]
    pub(crate) status_code: ResponseStatusCode,
    /// The reason phrase associated with the status code.
    #[set(skip)]
    pub(crate) reason_phrase: ResponseReasonPhrase,
    /// The response headers as key-value pairs.
    pub(crate) headers: ResponseHeaders,
    /// The binary body content of the response.
    #[set(skip)]
    pub(crate) body: ResponseBody,
}
