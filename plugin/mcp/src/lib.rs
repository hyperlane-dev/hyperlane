//! hyperlane-plugin-mcp
//!
//! Model Context Protocol (MCP) Streamable HTTP server plugin for Hyperlane
//! with `XxHash3_64` tool/resource/prompt registries, sharded TTL session
//! management with high-entropy IDs, Origin validation, and an opt-in
//! [`ServerHook`].

mod r#const;
mod r#enum;
mod r#fn;
mod r#impl;
mod r#static;
mod r#struct;
mod r#type;

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

pub use {r#const::*, r#enum::*, r#struct::*, r#type::*};

pub(crate) use {r#fn::*, r#static::*};

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    future::Future,
    hash::Hasher,
    pin::Pin,
    process,
    sync::{
        Arc, OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard,
        atomic::{AtomicU64, AtomicUsize, Ordering},
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use {
    hyperlane::{
        APPLICATION_JSON, CONTENT_TYPE, Context, HashMapXxHash3_64, HttpStatus, ORIGIN,
        ResponseData, ResponseError, ServerHook, Status, Stream, XxHash3_64, hash_map_xx_hash3_64,
    },
    lombok_macros::*,
    serde::{Deserialize, Serialize},
    serde_json::{Map, Value, from_slice, json, to_value, to_vec},
};
