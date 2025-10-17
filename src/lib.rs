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
mod handler;
mod hook;
mod lifecycle;
mod panic;
mod route;
mod server;
mod tests;

pub use attribute::*;
pub use config::*;
pub use context::*;
pub use error::*;
pub use handler::*;
pub use hook::*;
pub use panic::*;
pub use route::*;
pub use server::*;

pub use http_type::*;

pub(crate) use lifecycle::*;

pub(crate) use std::{
    any::Any,
    borrow::Borrow,
    cmp::Ordering,
    collections::{HashMap, HashSet},
    future::Future,
    net::SocketAddr,
    panic::Location,
    pin::Pin,
    sync::Arc,
    time::Duration,
};

pub(crate) use inventory::collect;
pub(crate) use lombok_macros::*;
pub(crate) use regex::Regex;
pub(crate) use serde::{Deserialize, Serialize, de::DeserializeOwned};
pub(crate) use tokio::{
    net::{TcpListener, TcpStream},
    spawn,
    sync::{
        RwLockReadGuard, RwLockWriteGuard,
        watch::{Receiver, Sender, channel},
    },
    task::{JoinError, JoinHandle},
};
