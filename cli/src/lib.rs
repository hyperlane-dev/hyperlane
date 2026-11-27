//! hyperlane-cli
//!
//! A command-line tool for Hyperlane framework.
mod command;
mod config;
mod help;
mod logger;
mod new;
mod template;
mod version;
mod watch;

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

pub use {command::*, config::*, help::*, logger::*, new::*, template::*, version::*, watch::*};

pub(crate) use std::{
    env::args,
    fmt::Arguments,
    io::{self, ErrorKind},
    path::{Component, Components, Path, PathBuf},
    process::{Output, Stdio},
    str::FromStr,
};

pub(crate) use {
    log::{
        Level, LevelFilter, Metadata, Record, SetLoggerError, max_level, set_logger, set_max_level,
    },
    lombok_macros::*,
    notify::{
        Error, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher, recommended_watcher,
    },
    std::ffi::OsStr,
    tokio::{
        fs::{canonicalize, create_dir_all, write},
        process::{Child, Command},
        sync::watch::{Receiver, Sender, channel, error::SendError},
        time::{Duration, Interval, interval, sleep},
    },
};
