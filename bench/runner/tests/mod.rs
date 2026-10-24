mod certificate;
mod report;

use bench_perf::{generate_certificates, render_report};

use std::{
    env, fs,
    path::PathBuf,
    process::{Command, Output, id},
    time::{SystemTime, UNIX_EPOCH},
};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
