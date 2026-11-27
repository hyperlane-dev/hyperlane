//! hyperlane-plugin-logger
//!
//! A high-concurrency logging plugin for the Hyperlane framework with
//! non-blocking buffered file output, pluggable sinks, plain/JSON formats,
//! and CRLF/ANSI log-forging sanitization.

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

pub use log::LevelFilter;

pub(crate) use {r#fn::*, r#static::*};

use std::{
    fmt::{self, Debug, Formatter},
    fs::{File, OpenOptions},
    io::{self, BufWriter, Write, stdout},
    sync::{
        Arc, OnceLock,
        atomic::{AtomicU64, Ordering},
        mpsc::{Receiver, RecvError, SyncSender, TrySendError, sync_channel},
    },
    thread::{Builder, JoinHandle},
    time::Instant,
};

use {
    hyperlane::{Context, RequestMethod, ServerHook, Status, Stream},
    log::{Level, Metadata, Record, SetLoggerError, log_enabled, set_logger, set_max_level},
};
