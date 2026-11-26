//! hyperlane-plugin-negotiate
//!
//! Response negotiation for the Hyperlane framework: a middleware that answers
//! with the content coding the client accepts, and a middleware that answers a
//! conditional request with 304 Not Modified while a validator still matches.

mod r#const;
mod r#fn;
mod r#impl;
mod r#static;
mod r#struct;
#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

pub use r#struct::*;

pub(crate) use {r#const::*, r#fn::*, r#static::*};

use std::{hash::Hasher, sync::OnceLock};

use {
    http_compress::{Compress, XxHash3_64},
    hyperlane::{
        ACCEPT_ENCODING, CACHE_CONTROL, COMMA, CONTENT_ENCODING, CONTENT_TYPE, Context, EMPTY_STR,
        ETAG, HttpStatus, Request, RequestMethod, Response, ResponseHeadersValue,
        ResponseStatusCode, ServerHook, Status, Stream, TEXT_EVENT_STREAM, VARY,
        VARY_ACCEPT_ENCODING,
    },
};
