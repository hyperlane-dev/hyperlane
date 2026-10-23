//! hyperlane-plugin-cors
//!
//! A cross origin plugin for the Hyperlane framework, answering the browsers
//! that ask a server for another origin.

mod r#const;
mod r#impl;
mod r#static;
mod r#struct;

pub use r#struct::*;

pub(crate) use r#static::*;

use r#const::*;

use std::sync::OnceLock;

use {
    hyperlane::{
        ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS,
        ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_MAX_AGE,
        ALL_METHODS, Context, ORIGIN, Response, ServerHook, Status, Stream, VARY, WILDCARD_ANY,
    },
    lombok_macros::*,
};
