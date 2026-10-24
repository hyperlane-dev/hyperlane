//! bench-perf
//!
//! The Rust orchestrator, certificate generator and reporter for the hyperlane benchmark harness.
mod certificate;
mod config;
mod report;
mod runner;

pub use {certificate::generate_certificates, report::render_report, runner::run};

pub(crate) use config::*;

use std::{
    collections::BTreeMap,
    env, fs,
    fs::{File, OpenOptions},
    io::{self, ErrorKind, Write},
    mem::swap,
    path::{Path, PathBuf},
    process::{Child, Command, ExitStatus, Output, Stdio, id},
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use {
    serde::Deserialize,
    serde_json::{from_str, to_string},
};
