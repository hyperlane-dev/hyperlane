//! hyperlane-plugin-session
//!
//! High-concurrency sharded session plugin for Hyperlane with pluggable async
//! [`SessionStore`] traits, optional HMAC-SHA256 signed cookies, dirty-tracked
//! zero-write reads, and session-fixation ID regeneration.

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

pub use {r#const::*, r#fn::*, r#struct::*, r#trait::*, r#type::*};

pub(crate) use r#static::*;

use std::{
    array::from_fn,
    collections::HashMap,
    future::Future,
    hash::Hasher,
    pin::Pin,
    process,
    sync::{
        Arc, Mutex, MutexGuard, OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard,
        atomic::{AtomicU64, AtomicUsize, Ordering},
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use {
    hyperlane::{
        COOKIE, Context, Cookie, CookieBuilder, Cookies, HashMapXxHash3_64, SET_COOKIE, ServerHook,
        Status, Stream, XxHash3_64, hash_map_xx_hash3_64,
    },
    lombok_macros::*,
};
