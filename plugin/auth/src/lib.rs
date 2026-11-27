//! hyperlane-plugin-auth
//!
//! Pure-Rust high-performance authentication and RBAC plugin for Hyperlane
//! with HS256 JWT (`kid` rotation, `exp`/`nbf`/`iss`/`aud`), API key and
//! HTTP Basic authenticators, constant-time verification, and extensible
//! async [`Authenticator`] traits.

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
    collections::HashMap,
    error::Error,
    fmt::{self, Display, Formatter},
    future::Future,
    pin::Pin,
    str::Split,
    sync::{Arc, OnceLock},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use {
    hyperlane::{
        AUTHORIZATION, Context, HashMapXxHash3_64, HttpStatus, ResponseData, ResponseError,
        ServerHook, Status, Stream, WWW_AUTHENTICATE, X_API_KEY, hash_map_xx_hash3_64,
    },
    lombok_macros::*,
    serde::{Deserialize, Serialize},
    serde_json::{Value, from_slice, to_vec},
};
