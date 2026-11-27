//! hyperlane-plugin-tracing
//!
//! Lock-free W3C Trace Context (`traceparent`/`tracestate`), `X-Request-Id`,
//! `Server-Timing`, and pluggable [`SpanExporter`] distributed tracing plugin
//! for Hyperlane.

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
    process,
    str::Split,
    sync::{
        Arc, OnceLock,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use {
    hyperlane::{Context, SERVER_TIMING, ServerHook, Status, Stream, X_REQUEST_ID},
    lombok_macros::*,
};
