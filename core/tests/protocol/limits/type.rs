pub(super) type DefaultErrorReply = Result<(u16, http::HeaderMap, Vec<u8>), String>;
