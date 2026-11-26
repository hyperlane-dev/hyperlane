//! hyperlane-plugin-grpc
//!
//! gRPC over HTTP/2 framing, bounded compression, canonical status codes,
//! timeout parsing, service routing, and an opt-in [`ServerHook`] for Hyperlane.

mod r#const;
mod r#enum;
mod r#fn;
mod r#impl;
mod r#static;
mod r#struct;
#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;
mod r#type;

pub use {r#const::*, r#enum::*, r#fn::*, r#struct::*, r#type::*};

pub(crate) use r#static::*;

use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display, Formatter},
    future::Future,
    pin::Pin,
    str::FromStr,
    sync::{Arc, OnceLock},
    time::Duration,
};

use {
    http_compress::Compress,
    hyperlane::{
        CONTENT_TYPE, Context, HttpStatus, Response, ResponseData, ResponseError, ServerHook,
        Status, Stream, tokio::time::timeout,
    },
    lombok_macros::*,
};
