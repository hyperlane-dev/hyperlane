//! hyperlane-plugin-static-files
//!
//! A static file plugin for the Hyperlane framework, serving the files of a
//! directory with the content type of the framework.

mod r#const;
mod r#impl;
mod r#static;
mod r#struct;

pub use r#struct::*;

pub(crate) use r#static::*;

use r#const::*;

use std::{
    path::{Path, PathBuf},
    sync::OnceLock,
};

use {
    hyperlane::{
        APPLICATION_OCTET_STREAM, CONTENT_TYPE, Context, FileExtension, HttpStatus, Response,
        ServerHook, Status, Stream, tokio::fs,
    },
    lombok_macros::*,
};
