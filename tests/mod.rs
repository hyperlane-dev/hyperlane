#[path = "protocol/head/mod.rs"]
mod head;

use hyperlane::*;
use std::{io, net::TcpListener, str::Utf8Error, str::from_utf8, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::{error::Elapsed, timeout},
};
