//! hyperlane-plugin-compression
//!
//! A compression plugin for the Hyperlane framework, answering a client with
//! the encoding it asked for.

mod r#const;
mod r#impl;
mod r#static;
mod r#struct;

pub use {hyperlane::Compress, r#struct::*};

pub(crate) use r#static::*;

use r#const::*;

use std::sync::OnceLock;

use {
    encoding_negotiator::CompressionPolicy,
    hyperlane::{
        ACCEPT_ENCODING, CONTENT_ENCODING, CONTENT_TYPE, Context, Response, ServerHook, Status,
        Stream, VARY, VARY_ACCEPT_ENCODING,
    },
    lombok_macros::*,
};
