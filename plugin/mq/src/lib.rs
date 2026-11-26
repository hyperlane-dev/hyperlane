//! hyperlane-plugin-mq
//!
//! Pluggable message queue abstraction and in-memory broker for Hyperlane
//! with topics, consumer groups, bounded backpressure, TTL expiration,
//! visibility timeouts, retries, and dead-letter queues.

mod r#const;
mod r#enum;
mod r#fn;
mod r#impl;
mod r#static;
mod r#struct;
#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;
mod r#trait;
mod r#type;

pub use {r#enum::*, r#struct::*, r#trait::*, r#type::*};

pub(crate) use {r#const::*, r#fn::*, r#static::*};

use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fmt::{self, Display, Formatter},
    future::Future,
    pin::Pin,
    sync::{
        Arc, Mutex, OnceLock,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use lombok_macros::*;
