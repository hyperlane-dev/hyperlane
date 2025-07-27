//! # Hyperlane
//!
//! Hyperlane is a lightweight and high-performance Rust HTTP server library
//! designed to simplify network service development.
//! It supports HTTP request parsing, response building, and TCP communication,
//! making it ideal for building modern web services. Additionally,
//! it provides support for request and response middleware, WebSocket, and Server-Sent Events (SSE),
//! enabling flexible and efficient real-time communication. Built with pure Rust and standard library,
//! Hyperlane offers true cross-platform compatibility across Windows, Linux and macOS,
//! with the same API experience on all platforms,
//! powered by Tokio's async runtime for seamless networking without platform-specific dependencies.

// The `attribute` module provides custom attributes for route handling.
mod attribute;
// The `config` module contains configuration structures and utilities.
mod config;
// The `context` module defines the application context and request handling.
mod context;
// The `error` module defines custom error types for the framework.
mod error;
// The `hook` module provides mechanisms for hooking into the application lifecycle.
mod hook;
// The `lifecycle` module manages the application's lifecycle events.
mod lifecycle;
// The `panic` module handles panic recovery and custom panic hooks.
mod panic;
// The `route` module is responsible for routing requests to handlers.
mod route;
// The `server` module contains the core HTTP server implementation.
mod server;
// The `tests` module contains integration and unit tests.
mod tests;

// Re-exports the `attribute` module for public use.
pub use attribute::*;
// Re-exports the `context` module for public use.
pub use context::*;
// Re-exports the `error` module for public use.
pub use error::*;
// Re-exports the `hook` module for public use.
pub use hook::*;
// Re-exports the `panic` module for public use.
pub use panic::*;
// Re-exports the `route` module for public use.
pub use route::*;
// Re-exports the `server` module for public use.
pub use server::*;

// Re-exports the `http_type` crate for convenient access to HTTP types.
pub use http_type::*;

// Makes the `config` module available within the crate.
pub(crate) use config::*;
// Makes the `lifecycle` module available within the crate.
pub(crate) use lifecycle::*;

// Internal imports from the standard library.
pub(crate) use std::{
    any::Any,
    collections::HashMap,
    future::Future,
    io::{self, Write},
    net::SocketAddr,
    panic::Location,
    panic::{PanicHookInfo, set_hook},
    pin::Pin,
    sync::Arc,
    time::Duration,
};

// Internal imports for macros and external crates.
pub(crate) use lombok_macros::*;
pub(crate) use regex::Regex;
pub(crate) use serde::de::DeserializeOwned;
pub(crate) use tokio::{
    net::{TcpListener, TcpStream},
    sync::{RwLockReadGuard, RwLockWriteGuard},
    task::JoinError,
};

// Test-specific internal imports.
#[cfg(test)]
pub(crate) use tokio::task::JoinHandle;
