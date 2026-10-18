use super::*;

pub(crate) type H2TlsSession = (
    h2::client::SendRequest<Bytes>,
    h2::client::Connection<ClientTlsStream, Bytes>,
);

pub(crate) type H2Reply = (u16, http::Version, Vec<u8>);

pub(crate) type H2HeadReply = (u16, http::HeaderMap, Vec<u8>);
