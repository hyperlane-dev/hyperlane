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
    collections::{HashMap, HashSet, VecDeque},
    fmt,
    future::{Future, poll_fn},
    hash::{Hash, Hasher},
    io::{self, IoSlice, IoSliceMut, Write, stderr, stdout},
    marker::PhantomPinned,
    mem::{self, MaybeUninit},
    net::{self, AddrParseError, SocketAddr},
    num::ParseIntError,
    panic::AssertUnwindSafe,
    pin::{Pin, pin},
    process::abort,
    ptr, slice,
    str::{Split, SplitWhitespace, Utf8Error, from_utf8},
    sync::{
        self, Arc, Mutex, MutexGuard, OnceLock,
        atomic::{AtomicBool, AtomicUsize},
    },
    task::{self, Poll, Waker},
    thread,
    time::Duration,
};

use {
    bytes::Bytes,
    futures::{FutureExt, StreamExt, stream::FuturesUnordered},
    h2::server::{Builder, SendResponse},
    http::{
        HeaderName, HeaderValue, StatusCode, Uri,
        status::InvalidStatusCode,
        uri::{Authority, Scheme},
    },
    inventory::collect,
    lombok_macros::*,
    quinn::{
        self, AsyncUdpSocket, Connection, ConnectionId, ConnectionIdGenerator, Endpoint,
        EndpointConfig, Incoming, RecvStream, Runtime, SendStream, TokioRuntime, UdpPoller, VarInt,
        WriteError,
        crypto::rustls::QuicServerConfig,
        udp::{BATCH_SIZE, RecvMeta, Transmit},
    },
    regex::Regex,
    rustls::{
        SupportedCipherSuite,
        crypto::{
            CryptoProvider, SecureRandom,
            ring::{
                cipher_suite::{
                    TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256,
                    TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384,
                    TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256,
                    TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256, TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384,
                    TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256, TLS13_AES_128_GCM_SHA256,
                    TLS13_AES_256_GCM_SHA384, TLS13_CHACHA20_POLY1305_SHA256,
                },
                default_provider,
            },
        },
        pki_types::{
            CertificateDer, PrivateKeyDer,
            pem::{self, PemObject},
        },
        server::UnbufferedServerConnection,
        unbuffered::{ConnectionState, EncodeError, EncryptError, UnbufferedStatus},
        version::{TLS12, TLS13},
    },
    serde::{Deserialize, Serialize},
    socket2::{Domain, SockRef, Socket, Type},
    tokio::{
        io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, ReadBuf},
        net::{TcpListener, TcpStream},
        runtime, spawn,
        sync::watch::{Receiver, Sender, channel, error::SendError},
        task::{JoinError, JoinHandle, spawn_blocking},
        time::{error::Elapsed, timeout},
    },
};

#[cfg(target_os = "linux")]
use {
    io_uring::{IoUring, opcode, squeue, types},
    libc::{
        AF_INET, AF_INET6, ECANCELED, EFD_CLOEXEC, EFD_NONBLOCK, EPIPE, IP_TTL, IPPROTO_IP,
        IPPROTO_IPV6, IPV6_UNICAST_HOPS, MSG_MORE, MSG_WAITALL, POLLIN, SHUT_WR, SOCK_CLOEXEC,
        SOCK_NONBLOCK, c_int, c_void, close, eventfd, read, setsockopt, shutdown, sockaddr,
        sockaddr_in, sockaddr_in6, sockaddr_storage, socklen_t, write,
    },
    std::{
        net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6},
        os::fd::{AsRawFd, RawFd},
        task::Wake,
    },
    tokio::runtime::{EnterGuard, Handle},
};
