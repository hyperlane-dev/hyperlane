//! hyperlane-cli
//!
//! A command-line tool for Hyperlane framework.
mod command;
mod config;
mod fmt;
mod help;
mod logger;
mod new;
mod template;
#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;
mod version;
mod watch;

pub use {
    command::*, config::*, fmt::*, help::*, logger::*, new::*, template::*, version::*, watch::*,
};

pub(crate) use std::{
    env::args,
    ffi::OsStr,
    fmt::Arguments,
    fs::{self, DirEntry},
    io::{self, ErrorKind},
    path::{Component, Components, Path, PathBuf},
    process::{Output, Stdio},
    str::FromStr,
};

pub(crate) use {
    crate_cli::{UnifiedFmtReport, execute_unified_fmt, format_workspace_text_files},
    log::{
        Level, LevelFilter, Metadata, Record, SetLoggerError, max_level, set_logger, set_max_level,
    },
    lombok_macros::*,
    notify::{
        Error, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher, recommended_watcher,
    },
    tokio::{
        fs::{canonicalize, create_dir_all, write},
        process::{Child, Command},
        sync::watch::{Receiver, Sender, channel, error::SendError},
        time::{Duration, Interval, interval, sleep},
    },
};
