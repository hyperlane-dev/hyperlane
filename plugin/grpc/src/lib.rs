//! hyperlane-plugin-grpc
//!
//! High-concurrency gRPC over HTTP/2 framing, bounded compression, canonical
//! status codes, timeout parsing, `XxHash3_64` service routing, extensible
//! interceptors, and an opt-in [`ServerHook`] for Hyperlane.

mod r#const;
mod r#enum;
mod r#fn;
mod r#impl;
mod r#static;
mod r#struct;
mod r#trait;
mod r#type;

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

pub use {r#const::*, r#enum::*, r#fn::*, r#struct::*, r#trait::*, r#type::*};

pub(crate) use r#static::*;

use std::{
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
        CONTENT_TYPE, Context, HashMapXxHash3_64, HttpStatus, Response, ResponseData,
        ResponseError, ServerHook, Status, Stream, hash_map_xx_hash3_64, tokio::time::timeout,
    },
    lombok_macros::*,
};
