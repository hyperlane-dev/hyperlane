//! hyperlane-plugin-rate-limit
//!
//! 64-way sharded lock-free atomic GCRA and fixed-window rate limiter plugin
//! for Hyperlane with extensible [`KeyExtractor`] and [`RateLimitStore`] traits
//! and RFC `RateLimit-*` response headers.

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

pub(crate) use {r#fn::*, r#static::*};

use std::{
    array::from_fn,
    hash::Hasher,
    net::SocketAddr,
    sync::{
        Arc, OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard,
        atomic::{AtomicU64, AtomicUsize, Ordering},
    },
    time::Instant,
};

use {
    hyperlane::{
        Context, HashMapXxHash3_64, HttpStatus, RETRY_AFTER, Response, ResponseData, ResponseError,
        ServerHook, Status, Stream, X_FORWARDED_FOR, X_REAL_IP, XxHash3_64, hash_map_xx_hash3_64,
    },
    lombok_macros::*,
};
