use super::*;

/// A single part of a `multipart/form-data` body.
///
/// A part carries the form field name declared by its `Content-Disposition`
/// header, the file name of an uploaded file when the part is a file, the
/// headers that belong to the part alone, and its raw content.
///
/// Storage is `pub(super)` (§17.14): a caller never touches the fields
/// directly. The public read surface is the `try_get_*` family below,
/// which follows the same `Option`-returning convention as
/// [`crate::Request`] and [`crate::Response`] and therefore never fails
/// loudly on a part that lacks an optional attribute.
#[derive(Clone, Data, Debug, Default, Deserialize, DisplayDebug, Eq, New, PartialEq, Serialize)]
pub struct BodyPart {
    /// Form field name declared by the part `Content-Disposition` header.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) name: BodyPartName,
    /// File name of an uploaded file, empty when the part is a plain field.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) file_name: BodyPartFileName,
    /// Media type declared by the part `Content-Type` header.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) content_type: BodyPartContentType,
    /// Headers carried by this part alone.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) headers: BodyPartHeaders,
    /// Raw content of this part.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) data: BodyPartData,
}

/// Parser for `multipart/form-data` bodies.
///
/// Decodes a body into its parts using the boundary declared by the request
/// `Content-Type` header, without depending on any external multipart crate.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct MultipartFormData;
