//! hyperlane-plugin-metrics
//!
//! Lock-free atomic Prometheus/OpenMetrics observability plugin for Hyperlane
//! with built-in HTTP request counters, in-flight gauges, fixed-bucket latency
//! histograms, custom metric handles, and a scrapeable `/metrics` endpoint.

mod r#const;
mod r#fn;
mod r#impl;
mod r#static;
mod r#struct;

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

pub use {r#const::*, r#struct::*};

pub(crate) use {r#fn::*, r#static::*};

use std::{
    fmt::{self, Write},
    sync::{
        Arc, OnceLock, RwLock,
        atomic::{AtomicI64, AtomicU64, Ordering},
    },
    time::Instant,
};

use {
    hyperlane::{
        AUTHORIZATION, CACHE_CONTROL, CONTENT_TYPE, Context, HashMapXxHash3_64, HttpStatus,
        ResponseData, ResponseError, ServerHook, Status, Stream, hash_map_xx_hash3_64,
    },
    lombok_macros::*,
};
