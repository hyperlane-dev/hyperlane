mod common;
mod request;
mod response;
mod utils;

pub use {request::*, response::*};

pub use http_type::{
    ACCEPT, ACCEPT_ANY, BR_BYTES, COLON_U8, CONTENT_LENGTH, CONTENT_TYPE, Compress, ContentType,
    HOST, HTTP_BR_BYTES, HTTPS_LOWERCASE, HashMapXxHash3_64, HttpStatus, HttpUrlComponents,
    HttpVersion, LOCATION, Method, Protocol, QUERY, RequestError, RequestHeadersKey,
    RequestHeadersValue, ResponseStatusCode, SPACE_U8, TAB_U8, USER_AGENT, hash_map_xx_hash3_64,
};

pub use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::{self, Display, Formatter},
    io::{Read, Write},
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
        ClientConfig, ClientConnection, RootCertStore, StreamOwned,
        pki_types::{InvalidDnsNameError, ServerName},
    },
    serde::Serialize,
    tokio_rustls::TlsConnector,
    webpki_roots::TLS_SERVER_ROOTS,
};

use common::*;
use lombok_macros::*;
