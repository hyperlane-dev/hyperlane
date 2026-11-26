//! hyperlane-plugin-sse
//!
//! Server-Sent Events (SSE) framing, topic pub/sub fan-out, bounded
//! `Last-Event-ID` replay, and heartbeat streaming for Hyperlane.

mod r#const;
mod r#enum;
mod r#fn;
mod r#impl;
mod r#static;
mod r#struct;
#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

pub use {r#enum::*, r#struct::*};

pub(crate) use {r#const::*, r#fn::*, r#static::*};

use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    fmt::{self, Display, Formatter},
    sync::{
        Arc, Mutex, MutexGuard, OnceLock,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};

use {
    hyperlane::{
        CACHE_CONTROL, CONTENT_TYPE, Context, ResponseData, ResponseError, Stream,
        tokio::{
            select,
            sync::broadcast::{self, Receiver, Sender, error::RecvError},
            time::{Instant, Interval, MissedTickBehavior, interval},
        },
    },
    lombok_macros::*,
};
