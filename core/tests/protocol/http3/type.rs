pub(crate) type H3Reply = (u16, http::Version, Vec<u8>, Option<Vec<u8>>);

pub(crate) type H3HeadReply = (u16, http::HeaderMap, Vec<u8>, Option<Vec<u8>>);

pub(crate) struct H3Exchange<'a> {
    pub(crate) connection: quinn::Connection,
    pub(crate) alpn: Option<Vec<u8>>,
    pub(crate) method: &'a str,
    pub(crate) target: &'a str,
    pub(crate) body: &'a [u8],
}
