//! hyperlane-plugin-rpc
//!
//! JSON-RPC 2.0 server plugin for Hyperlane with async method routing,
//! single and batch dispatch, notification handling, and an opt-in [`ServerHook`].

mod r#const;
mod r#enum;
mod r#fn;
mod r#impl;
mod r#static;
mod r#struct;
#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;
mod r#type;

pub use {r#const::*, r#enum::*, r#struct::*, r#type::*};

pub(crate) use {r#fn::*, r#static::*};

use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display, Formatter},
    future::Future,
    pin::Pin,
    sync::{Arc, OnceLock},
};

use {
    hyperlane::{
        APPLICATION_JSON, CONTENT_TYPE, Context, HttpStatus, ResponseData, ResponseError,
        ServerHook, Status, Stream,
    },
    lombok_macros::*,
    serde::{Deserialize, Serialize},
    serde_json::{Value, from_slice, from_value, to_vec},
};
