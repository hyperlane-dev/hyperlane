//! hyperlane-plugin-sse
//!
//! High-concurrency Server-Sent Events (SSE) framing, 64-way sharded topic
//! pub/sub fan-out, bounded `Last-Event-ID` replay, and zero-allocation
//! streaming heartbeats for Hyperlane.

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

pub use {r#const::*, r#enum::*, r#struct::*};

pub(crate) use {r#fn::*, r#static::*, r#type::*};

use std::{
    array::from_fn,
    collections::VecDeque,
    error::Error,
    fmt::{self, Display, Formatter},
    hash::Hasher,
    sync::{
        Arc, Mutex, MutexGuard, OnceLock, RwLock, RwLockReadGuard,
        atomic::{AtomicU64, AtomicUsize, Ordering},
    },
    time::Duration,
};

use {
    hyperlane::{
        CACHE_CONTROL, CONTENT_TYPE, Context, HashMapXxHash3_64, ResponseData, ResponseError,
        Stream, XxHash3_64, hash_map_xx_hash3_64,
        tokio::{
            select,
            sync::broadcast::{self, Receiver, Sender, error::RecvError},
            time::{Instant, Interval, MissedTickBehavior, interval},
        },
    },
    lombok_macros::*,
};
