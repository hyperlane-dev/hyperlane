//! hyperlane
//!
//! A lightweight, high-performance, and cross-platform
//! Rust HTTP server library built on Tokio. It simplifies
//! modern web service development by providing built-in
//! support for middleware, WebSocket, Server-Sent Events (SSE),
//! and raw TCP communication. With a unified and ergonomic API
//! across Windows, Linux, and MacOS, it enables developers to
//! build robust, scalable, and event-driven network
//! applications with minimal overhead and maximum flexibility.

mod config;
mod context;
mod error;
mod hook;
mod route;
mod server;

pub use {config::*, context::*, error::*, hook::*, route::*, server::*};

pub use {http_type::*, inventory};

use std::{
    cmp::Ordering,
    collections::HashSet,
    future::Future,
    hash::{Hash, Hasher},
    io::{self, Write, stderr, stdout},
    net::{AddrParseError, SocketAddr},
    pin::Pin,
    sync::Arc,
};

use {
    inventory::collect,
    lombok_macros::*,
    regex::Regex,
    serde::{Deserialize, Serialize},
    socket2::{Domain, Socket, Type},
    tokio::{
        net::{TcpListener, TcpStream},
        spawn,
        sync::watch::{Receiver, Sender, channel},
        task::JoinHandle,
    },
};
