//! hyperlane-plugin-mq
//!
//! High-concurrency pluggable message queue abstraction and sharded in-memory
//! broker for Hyperlane with topics, consumer groups, bounded backpressure,
//! TTL expiration, `O(log N)` min-heap visibility timeouts, retries, and
//! dead-letter queues.

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
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    error::Error,
    fmt::{self, Display, Formatter},
    future::Future,
    pin::Pin,
    sync::{
        Arc, Mutex, MutexGuard, OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use lombok_macros::*;
