mod config;
mod context;
mod error;
mod route;
mod server;

use hyperlane::*;

use std::{
    sync::{Arc, OnceLock},
    time::{Duration, Instant},
};

use tokio::{spawn, task::JoinHandle, time::sleep};
