//! hyperlane-plugin-security
//!
//! Zero-allocation security hardening plugin for Hyperlane providing prebuilt
//! security headers, stateless HMAC-SHA256 CSRF protection, host/URI/body/
//! path-traversal guards, and extensible [`SecurityRule`] hooks.

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
    error::Error,
    fmt::{self, Display, Formatter},
    sync::{Arc, OnceLock},
};

use {
    hyperlane::{
        CONTENT_SECURITY_POLICY, COOKIE, CROSS_ORIGIN_EMBEDDER_POLICY, CROSS_ORIGIN_OPENER_POLICY,
        CROSS_ORIGIN_RESOURCE_POLICY, Context, Cookie, Cookies, HOST, HttpStatus, Method,
        PERMISSIONS_POLICY, REFERRER_POLICY, Response, ResponseData, ResponseError,
        STRICT_TRANSPORT_SECURITY, ServerHook, Status, Stream, X_CONTENT_TYPE_OPTIONS,
        X_FRAME_OPTIONS, X_POWERED_BY,
    },
    lombok_macros::*,
};
