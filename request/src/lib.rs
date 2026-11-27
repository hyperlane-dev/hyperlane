//! http-request
//!
//! An HTTP request library providing client-side HTTP request construction and response parsing.

mod common;
mod pool;
mod protocol;
mod request;
mod response;
mod tls;
mod utils;

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

pub use {request::*, response::*};

pub use http_type::{
    ACCEPT, ACCEPT_ANY, BR_BYTES, COLON_U8, CONTENT_LENGTH, CONTENT_TYPE, Compress, ContentType,
    HOST, HTTP_BR_BYTES, HTTPS_LOWERCASE, HashMapXxHash3_64, HttpStatus, HttpUrlComponents,
    HttpVersion, LOCATION, Method, Protocol, QUERY, RequestError, RequestHeadersKey,
    RequestHeadersValue, ResponseStatusCode, SPACE_U8, TAB_U8, USER_AGENT, hash_map_xx_hash3_64,
};

pub use std::{
    cmp,
    collections::{HashMap, HashSet, VecDeque},
    fmt::{self, Debug, Display, Formatter},
    io::{self, Read, Write},
    mem,
    net::{Ipv4Addr, Ipv6Addr, TcpStream},
    pin::Pin,
    str::from_utf8,
    string::FromUtf8Error,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
    vec::IntoIter,
};

pub use {
    http_type::tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, ReadBuf},
    rustls::{
        ClientConfig, ClientConnection, Error, RootCertStore, StreamOwned,
        pki_types::{InvalidDnsNameError, ServerName},
    },
    serde::Serialize,
    serde_json::to_vec,
    tokio_rustls::{TlsConnector, client::TlsStream},
    webpki_roots::TLS_SERVER_ROOTS,
};

pub use common::*;
pub use lombok_macros::*;

pub(crate) use {pool::*, protocol::*, tls::*};

use std::{
    cell::{RefCell, RefMut},
    net::{SocketAddr, ToSocketAddrs},
    sync::{Mutex, OnceLock},
    thread,
    time::Instant,
};

use {
    bytes::Bytes,
    h2::{RecvStream, client::SendRequest},
    http::{HeaderName, HeaderValue, Request, Response, StatusCode, Uri, request::Builder},
    http_type::{
        BodyPart, CLOSE, CONNECTION, COOKIE, CookieBuilder, Cookies, KEEP_ALIVE, SET_COOKIE, tokio,
    },
    rustls::client::Resumption,
    rustls_pki_types::{CertificateDer, pem::PemObject},
    serde::de::DeserializeOwned,
    serde_json::from_slice,
};
