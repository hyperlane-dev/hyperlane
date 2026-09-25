mod config;
mod context;
mod error;
mod route;
mod server;

use hyperlane_core::*;

use std::{
    net::TcpListener,
    sync::{Arc, OnceLock},
    time::{Duration, Instant},
};

use tokio::{io::AsyncWriteExt, net::TcpStream, spawn, task::JoinHandle, time::sleep};
