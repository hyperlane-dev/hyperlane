use super::*;

pub(super) type DefaultErrorReply = Result<(u16, HeaderMap, Vec<u8>), String>;
