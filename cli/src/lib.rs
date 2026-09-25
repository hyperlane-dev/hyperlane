mod command;
mod config;
mod help;
mod logger;
mod new;
mod template;
mod version;
mod watch;

pub use {command::*, config::*, help::*, logger::*, new::*, template::*, version::*, watch::*};

pub(crate) use std::{
    env::args,
    io,
    path::{Path, PathBuf},
    process::Stdio,
    str::FromStr,
};

pub(crate) use {
    notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher, recommended_watcher},
    std::ffi::OsStr,
    tokio::{
        fs::{create_dir_all, write},
        process::Command,
        sync::watch::{Receiver, Sender, channel},
        time::{Duration, Interval, interval, sleep},
    },
};
