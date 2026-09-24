//! http-request
//!
//! http-request is a lightweight, efficient library for building, sending,
//! and handling HTTP/HTTPS requests in Rust applications.
//! It provides a simple and intuitive API, allowing developers to easily
//! interact with web services, whether they use the "HTTP" or "HTTPS" protocol.
//! The library supports various HTTP methods, custom headers, request bodies,
//! timeout, automatic handling of redirects (including detecting redirect loops),
//! and enhanced response body decoding (both automatic and manual), enabling fast
//! and secure communication. Whether working with secure "HTTPS" connections
//! or standard "HTTP" requests, the library is optimized for performance,
//! minimal resource usage, and easy integration into Rust projects.

mod common;
mod request;
mod response;
mod utils;

pub use {request::*, response::*};

pub use {
    http_type::{HashMapXxHash3_64, RequestError, hash_map_xx_hash3_64},
    serde_json::{
        Deserializer, Error, Map, Number, StreamDeserializer, Value, from_reader, from_slice,
        from_str, from_value, to_string, to_string_pretty, to_value, to_vec, to_vec_pretty,
        to_writer, to_writer_pretty, value,
    },
};

use {common::*, utils::*};

use std::{
    borrow::Cow,
    collections::{HashSet, VecDeque},
    fmt::{self, Debug, Display, Formatter},
    io::{Read, Write},
    net::{Ipv4Addr, Ipv6Addr, TcpStream},
    pin::Pin,
    str::from_utf8,
    string::FromUtf8Error,
    sync::{
        Arc, RwLock, RwLockReadGuard,
        atomic::{AtomicBool, Ordering},
    },
    task::{Context, Poll},
    time::{Duration, SystemTime, UNIX_EPOCH},
    vec::IntoIter,
};

use {
    futures::{Future, Sink, SinkExt, Stream, StreamExt},
    http_type::{
        ACCEPT, ACCEPT_ANY, BR_BYTES, COLON_U8, CONNECTION, CONTENT_LENGTH, CONTENT_TYPE, Compress,
        ContentType, DEFAULT_BUFFER_SIZE, DEFAULT_HIGH_SECURITY_READ_TIMEOUT_MS, DEFAULT_HTTP_PATH,
        DEFAULT_MAX_REDIRECT_TIMES, EMPTY_STR, HOST, HTTP_BR_BYTES, HttpStatus, HttpUrlComponents,
        HttpVersion, LOCATION, Method, Protocol, QUERY, RequestBody, RequestBodyString,
        RequestHeaders, ResponseHeaders, ResponseStatusCode, SEC_WEBSOCKET_KEY,
        SEC_WEBSOCKET_VERSION, SPACE_U8, TAB_U8, UPGRADE, USER_AGENT,
        tokio::{
            io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, ReadBuf},
            runtime::Runtime,
            sync::Mutex,
            time::timeout,
        },
    },
    rustls::{
        ClientConfig, ClientConnection, RootCertStore, StreamOwned,
        pki_types::{InvalidDnsNameError, ServerName},
    },
    serde::{Serialize, Serializer},
    tokio_rustls::{TlsConnector, client::TlsStream},
    tokio_tungstenite::{
        MaybeTlsStream, WebSocketStream, client_async_with_config, connect_async_with_config,
        tungstenite::Message, tungstenite::handshake::client::Request,
    },
    webpki_roots::TLS_SERVER_ROOTS,
};
