//! hyperlane-plugin-shutdown
//!
//! Graceful shutdown coordinator plugin for Hyperlane providing OS signal
//! arming (`SIGINT`/`SIGTERM`), lock-free in-flight draining via RAII
//! [`ShutdownGuard`], LIFO async [`ShutdownCallback`] execution, and
//! [`ServerControlHook`] integration.

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
    future::{Future, pending},
    io,
    pin::Pin,
    sync::{
        Arc, OnceLock,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    time::{Duration, Instant},
};

#[cfg(unix)]
use hyperlane::tokio::signal::unix::{SignalKind, signal};

use {
    hyperlane::{
        ServerControlHook,
        tokio::{
            select,
            signal::ctrl_c,
            spawn,
            sync::{Notify, futures::Notified},
            task::JoinHandle,
            time::timeout,
        },
    },
    lombok_macros::*,
};
