//! hyperlane
//!
//! hyperlane is a lightweight and high-performance Rust HTTP server library
//! designed to simplify network service development. It supports HTTP request parsing,
//! response building, and TCP communication, making it ideal for building modern
//! web services. Additionally, it provides support for request and response middleware,
//! WebSocket, and Server-Sent Events (SSE), enabling flexible and efficient
//! real-time communication. Built with pure Rust and standard library,
//! Hyperlane offers true cross-platform compatibility across Windows, Linux and macOS,
//! with the same API experience on all platforms, powered by Tokio's async runtime for
//! seamless networking without platform-specific dependencies.

mod attribute;
mod config;
mod context;
mod error;
mod hook;
mod lifecycle;
mod panic;
mod route;
mod server;
mod tests;

pub use attribute::*;
pub use context::*;
pub use error::*;
pub use hook::*;
pub use panic::*;
pub use route::*;
pub use server::*;

pub use http_type::*;

pub(crate) use config::*;
pub(crate) use lifecycle::*;

pub(crate) use std::{
    any::Any,
    collections::HashMap,
    future::Future,
    net::SocketAddr,
    panic::Location,
    panic::{PanicHookInfo, set_hook},
    pin::Pin,
    sync::Arc,
    time::Duration,
};

pub(crate) use lombok_macros::*;
pub(crate) use regex::Regex;
pub(crate) use serde::de::DeserializeOwned;
pub(crate) use tokio::{
    net::{TcpListener, TcpStream},
    sync::{RwLockReadGuard, RwLockWriteGuard, mpsc},
    task::{JoinError, JoinHandle},
};
