//! hyperlane-plugin-cache
//!
//! 64-way sharded bounded cache and HTTP response caching plugin for Hyperlane
//! with TTL/byte-budget eviction, single-flight stampede coalescing, pluggable
//! [`CacheStore`] traits, and RFC `Cache-Control`/`Age`/`X-Cache` support.

mod r#const;
mod r#fn;
mod r#impl;
mod r#static;
mod r#struct;
mod r#trait;
mod r#type;

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

pub use {r#const::*, r#struct::*, r#trait::*, r#type::*};

pub(crate) use {r#fn::*, r#static::*};

use std::{
    array::from_fn,
    collections::VecDeque,
    future::Future,
    hash::Hasher,
    pin::Pin,
    sync::{
        Arc, Mutex, OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard,
        atomic::{AtomicUsize, Ordering},
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use {
    hyperlane::{
        AGE, AUTHORIZATION, CACHE_CONTROL, COOKIE, Context, HashMapXxHash3_64, HttpStatus,
        RequestQuerys, Response, ResponseData, ResponseError, ServerHook, Status, Stream, VARY,
        XxHash3_64, hash_map_xx_hash3_64,
        tokio::sync::{Notify, futures::Notified},
    },
    lombok_macros::*,
};
