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

mod attribute;
mod config;
mod context;
mod error;
mod hook;
mod lifetime;
mod panic;
mod route;
mod server;
mod task;

pub use {
    attribute::*, config::*, context::*, error::*, hook::*, lifetime::*, panic::*, route::*,
    server::*, task::*,
};

pub use {http_type::*, inventory};

#[cfg(test)]
use std::time::{Duration, Instant};
use std::{
    any::Any,
    cmp::Ordering,
    collections::{HashMap, HashSet},
    future::Future,
    hash::{Hash, Hasher},
    io::{self, Write, stderr, stdout},
    pin::Pin,
    sync::{
        Arc, OnceLock,
        atomic::{self, AtomicBool, AtomicUsize},
    },
};

#[cfg(test)]
use tokio::time::sleep;
use {
    inventory::collect,
    lombok_macros::*,
    regex::Regex,
    serde::{Deserialize, Serialize},
    tokio::{
        net::{TcpListener, TcpStream},
        runtime::Handle,
        spawn,
        sync::{
            Notify,
            mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
            watch::{Receiver, Sender, channel},
        },
        task::{JoinError, JoinHandle, LocalSet, spawn_blocking, spawn_local},
    },
};
