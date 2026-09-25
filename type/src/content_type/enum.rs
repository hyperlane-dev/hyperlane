use super::*;

/// Supported HTTP content types.
///
/// Defines common content types for HTTP communication.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum ContentType {
    /// `application/json` content type.
    ///
    /// For JSON data format.
    ApplicationJson,
    /// `application/xml` content type.
    ///
    /// For XML data format.
    ApplicationXml,
    /// `text/plain` content type.
    ///
    /// For plain text data.
    TextPlain,
    /// `text/html` content type.
    ///
    /// For HTML documents.
    TextHtml,
    /// `application/x-www-form-urlencoded` content type.
    ///
    /// For form data submission.
    FormUrlEncoded,
    /// Unknown content type.
    ///
    /// For unrecognized content types.
    #[default]
    Unknown,
}
