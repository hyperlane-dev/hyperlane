//! hyperlane-plugin-rpc
//!
//! High-concurrency JSON-RPC 2.0 server plugin for Hyperlane with `XxHash3_64`
//! async method routing, concurrent batch dispatch, extensible interceptors,
//! notification handling, and an opt-in [`ServerHook`].

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

pub use {r#const::*, r#enum::*, r#struct::*, r#trait::*, r#type::*};

pub(crate) use {r#fn::*, r#static::*, r#type::PreparedRpcCall};

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    future::Future,
    pin::Pin,
    sync::{Arc, OnceLock},
};

use {
    hyperlane::{
        APPLICATION_JSON, CONTENT_TYPE, Context, HashMapXxHash3_64, HttpStatus, ResponseData,
        ResponseError, ServerHook, Status, Stream, hash_map_xx_hash3_64,
        tokio::{spawn, task::JoinHandle},
    },
    lombok_macros::*,
    serde::{Deserialize, Serialize},
    serde_json::{Value, from_slice, from_value, to_vec},
};
