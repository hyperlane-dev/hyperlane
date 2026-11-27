//! hyperlane-plugin-cors
//!
//! Zero-allocation precompiled Cross-Origin Resource Sharing (CORS) plugin for
//! Hyperlane with `XxHash3_64` origin matching, safe credential reflection,
//! preflight handling, and `Vary` merging.

mod r#const;
mod r#enum;
mod r#fn;
mod r#impl;
mod r#static;
mod r#struct;
mod r#type;

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

pub use {r#const::*, r#enum::*, r#struct::*, r#type::*};

pub(crate) use {r#fn::*, r#static::*};

use std::{
    fmt::{self, Debug, Formatter},
    sync::{Arc, OnceLock},
};

use {
    hyperlane::{
        ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS,
        ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_EXPOSE_HEADERS,
        ACCESS_CONTROL_MAX_AGE, ACCESS_CONTROL_REQUEST_HEADERS, ACCESS_CONTROL_REQUEST_METHOD,
        Context, HashSetXxHash3_64, HttpStatus, ORIGIN, Response, ResponseData, ResponseError,
        ServerHook, Status, Stream, VARY, hash_set_xx_hash3_64,
    },
    lombok_macros::*,
};
