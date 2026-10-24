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
    cors_policy::CorsPolicy,
    hyperlane::{ALL_METHODS, Context, ORIGIN, Response, ServerHook, Status, Stream, WILDCARD_ANY},
    lombok_macros::*,
};
