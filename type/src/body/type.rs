use super::*;

/// An alias for `String`, representing the boundary declared by a `multipart/form-data` content type.
pub type MultipartFormDataBoundary = String;

/// An alias for `Vec<BodyPart>`, representing every part of a parsed `multipart/form-data` body.
pub type MultipartFormDataParts = Vec<BodyPart>;

/// An alias for `Vec<&[u8]>`, representing the raw segments carried between two boundary lines.
pub type MultipartFormDataSegments<'a> = Vec<&'a [u8]>;

/// An alias for `String`, representing the form field name declared by a body part.
pub type BodyPartName = String;

/// An alias for `String`, representing the file name declared by an uploaded body part.
pub type BodyPartFileName = String;

/// An alias for `String`, representing the media type declared by a body part.
pub type BodyPartContentType = String;

/// An alias for `String`, representing the key of a body part header.
pub type BodyPartHeaderKey = String;

/// An alias for `String`, representing the value of a body part header.
pub type BodyPartHeaderValue = String;

/// An alias for `HashMapXxHash3_64<BodyPartHeaderKey, BodyPartHeaderValue>`, representing the headers a single part carries.
pub type BodyPartHeaders = HashMapXxHash3_64<BodyPartHeaderKey, BodyPartHeaderValue>;

/// An alias for `Vec<u8>`, representing the raw content of a body part.
pub type BodyPartData = Vec<u8>;

/// An alias for `HashMapXxHash3_64<BodyFormKey, BodyFormValue>`, representing a decoded `application/x-www-form-urlencoded` body.
pub type BodyForm = HashMapXxHash3_64<BodyFormKey, BodyFormValue>;

/// An alias for `String`, representing the key of a form field.
pub type BodyFormKey = String;

/// An alias for `String`, representing the value of a form field.
pub type BodyFormValue = String;

/// An alias for `String`, representing the request media type without its parameters.
pub type RequestContentType = String;

/// An alias for `String`, representing one parameter of a request `Content-Type` header.
pub type RequestContentTypeParameter = String;
