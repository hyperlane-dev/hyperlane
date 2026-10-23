//! hyperlane-plugin-logger
//!
//! A logging plugin for the Hyperlane framework, writing the log of a server
//! to the console, to a file, or to both.

mod r#const;
mod r#impl;
mod r#static;
mod r#struct;

pub use {log::LevelFilter, r#struct::*};

pub(crate) use r#static::*;

use r#const::*;

use std::{
    fs::{File, OpenOptions},
    io::{Error, Write},
    sync::{Mutex, OnceLock},
    time::Instant,
};

use hyperlane::{Context, ServerHook, Status, Stream};
