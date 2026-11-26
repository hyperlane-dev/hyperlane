//! hyperlane-plugin-mcp
//!
//! Model Context Protocol (MCP) Streamable HTTP server plugin for Hyperlane
//! with async tool, resource, and prompt registries, session management,
//! Origin validation, and an opt-in [`ServerHook`].

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
    collections::{HashMap, HashSet},
    error::Error,
    fmt::{self, Display, Formatter},
    future::Future,
    pin::Pin,
    sync::{
        Arc, Mutex, MutexGuard, OnceLock,
        atomic::{AtomicU64, Ordering},
    },
};

use {
    hyperlane::{
        APPLICATION_JSON, CONTENT_TYPE, Context, HttpStatus, ORIGIN, ResponseData, ResponseError,
        ServerHook, Status, Stream,
    },
    lombok_macros::*,
    serde::{Deserialize, Serialize},
    serde_json::{Map, Value, from_slice, json, to_value, to_vec},
};
