//! hyperlane-plugin-health
//!
//! High-concurrency `/healthz`, `/livez`, and `/readyz` health probe plugin for
//! Hyperlane with concurrent async [`HealthCheck`] evaluation, per-probe timeout,
//! TTL snapshot caching, and lock-free graceful drain signaling.

mod r#const;
mod r#enum;
mod r#impl;
mod r#static;
mod r#struct;
mod r#trait;
mod r#type;

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

pub use {r#const::*, r#enum::*, r#struct::*, r#trait::*, r#type::*};

pub(crate) use r#static::*;

use std::{
    future::Future,
    pin::Pin,
    sync::{
        Arc, OnceLock, RwLock,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use {
    hyperlane::{
        APPLICATION_JSON, CACHE_CONTROL, CONTENT_TYPE, Context, HttpStatus, ResponseData,
        ResponseError, ServerHook, Status, Stream,
        tokio::{spawn, task::JoinHandle, time::timeout},
    },
    lombok_macros::*,
    serde::{Deserialize, Serialize},
    serde_json::to_vec,
};
