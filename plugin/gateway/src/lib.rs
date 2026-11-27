//! hyperlane-plugin-gateway
//!
//! High-concurrency reverse proxy and API gateway plugin for Hyperlane
//! providing prefix/host routing, allocation-free load balancing
//! (round-robin, weighted, least-connections, power-of-two-choices),
//! circuit breaking, lock-free GCRA rate limiting, extensible interceptors,
//! RFC 9110 hop-by-hop header stripping, and pooled HTTP/1.1 + HTTP/2 forwarding.

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

pub use {r#const::*, r#enum::*, r#fn::*, r#struct::*, r#trait::*, r#type::*};

pub(crate) use r#static::*;

use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    sync::{
        Arc, OnceLock,
        atomic::{AtomicU32, AtomicU64, AtomicUsize, Ordering},
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use {
    http_request::{HttpRequest, HttpResponse, RequestBuilder, RequestError},
    hyperlane::{
        CONNECTION, Context, HOST, HttpStatus, KEEP_ALIVE, PROXY_AUTHENTICATE, PROXY_AUTHORIZATION,
        RequestHeadersValue, Response, ResponseData, ResponseError, ServerHook, Status, Stream, TE,
        TRANSFER_ENCODING, UPGRADE,
    },
    lombok_macros::*,
};
