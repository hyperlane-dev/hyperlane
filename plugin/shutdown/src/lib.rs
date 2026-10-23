//! hyperlane-plugin-shutdown
//!
//! A shutdown plugin for the Hyperlane framework, turning the termination
//! signals of the operating system into a graceful shutdown of the server.

mod r#const;
mod r#impl;
mod r#static;
mod r#struct;

pub use r#struct::*;

pub(crate) use r#static::*;

use r#const::*;

use std::{io::Error, sync::OnceLock};

use hyperlane::{ServerControlHook, ServerControlHookHandler, tokio};
