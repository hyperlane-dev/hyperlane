pub(crate) type H3Reply = (u16, http::Version, Vec<u8>, Option<Vec<u8>>);

pub(crate) type H3HeadReply = (u16, http::HeaderMap, Vec<u8>, Option<Vec<u8>>);
