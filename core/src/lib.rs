//! hyperlane-core
//!
//! A lightweight, high-performance, and cross-platform Rust
//! HTTP server library built on Tokio. It simplifies modern
//! web service development by providing built-in support
//! for middleware, WebSocket, Server-Sent Events (SSE),
//! and raw TCP communication. With a unified and ergonomic
//! API across Windows, Linux, and MacOS, it enables developers
//! to build robust, scalable, and event-driven network
//! applications with minimal overhead and maximum flexibility.

mod config;
mod context;
mod error;
mod hook;
mod listener;
mod protocol;
mod route;
mod server;
#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;
mod tls;

pub use {config::*, context::*, error::*, hook::*, route::*, server::*};

pub use {http_type::*, inventory};

pub(crate) use {listener::*, protocol::*, tls::*};

use std::{
    any::Any,
    cell::{RefCell, RefMut},
    cmp::Ordering,
    collections::HashSet,
    future::{Future, poll_fn},
    hash::{Hash, Hasher},
    io::{self, Write, stderr, stdout},
    mem,
    net::{AddrParseError, SocketAddr},
    num::ParseIntError,
    panic::AssertUnwindSafe,
    pin::Pin,
    ptr,
    str::{Split, SplitWhitespace, Utf8Error, from_utf8},
    sync::Arc,
    task::{self, Poll},
    time::Duration,
};

use {
    bytes::Bytes,
    futures::FutureExt,
    h3::error::Code,
    inventory::collect,
    lombok_macros::*,
    quinn::{self, Endpoint, Incoming, VarInt},
    regex::Regex,
    rustls::pki_types::{
        CertificateDer, PrivateKeyDer,
        pem::{self, PemObject},
    },
    serde::{Deserialize, Serialize},
    tokio::{
        io::{AsyncRead, AsyncWrite, AsyncWriteExt, ReadBuf},
        join,
        net::{TcpListener, TcpStream},
        spawn,
        sync::watch::{Receiver, Sender, channel},
        task::JoinHandle,
        time::timeout,
    },
    tokio_rustls::TlsAcceptor,
};
