//! hyperlane-plugin-static-files
//!
//! High-performance static file serving plugin for Hyperlane with path-traversal
//! defense, Hyperlane [`FileExtension`] MIME detection, 64-way sharded hot-file
//! caching, `.br`/`.gz` precompressed sidecars, `ETag`/`304`, RFC 9110 `Range`
//! (`206`/`416`), and SPA fallback.

mod r#const;
mod r#fn;
mod r#impl;
mod r#static;
mod r#struct;
mod r#type;

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

pub use {r#const::*, r#fn::*, r#struct::*};

pub(crate) use {r#static::*, r#type::*};

use std::{
    array::from_fn,
    ffi::OsString,
    fs::{Metadata, canonicalize},
    hash::Hasher,
    io,
    path::{Component, Path, PathBuf},
    str::Split,
    sync::{
        Arc, OnceLock, RwLock,
        atomic::{AtomicUsize, Ordering},
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use {
    hyperlane::{
        ACCEPT_ENCODING, ACCEPT_RANGES, APPLICATION_OCTET_STREAM, CACHE_CONTROL, CONTENT_ENCODING,
        CONTENT_RANGE, CONTENT_TYPE, Context, ETAG, FileExtension, HashMapXxHash3_64, HttpStatus,
        IF_NONE_MATCH, RANGE, Response, ResponseData, ResponseError, ServerHook, Status, Stream,
        VARY, XxHash3_64, hash_map_xx_hash3_64, tokio::fs,
    },
    lombok_macros::*,
};
