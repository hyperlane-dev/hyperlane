use super::*;

pub(crate) type H3Reply = (u16, Version, Vec<u8>, Option<Vec<u8>>);

pub(crate) type H3HeadReply = (u16, HeaderMap, Vec<u8>, Option<Vec<u8>>);

pub(crate) struct H3Exchange<'a> {
    pub(crate) connection: quinn::Connection,
    pub(crate) alpn: Option<Vec<u8>>,
    pub(crate) method: &'a str,
    pub(crate) target: &'a str,
    pub(crate) body: &'a [u8],
}
