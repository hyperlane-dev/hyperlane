mod config;
mod context;
mod error;
mod framing;
mod protocol;
mod route;
mod server;

use super::*;

use std::{
    any::Any,
    collections::VecDeque,
    io,
    net::{AddrParseError, SocketAddr, TcpListener, UdpSocket},
    path::PathBuf,
    sync::{Arc, Mutex, OnceLock},
    time::{Duration, Instant},
};

use bytes::{Buf, Bytes};
use h3::error::Code;
use rustls::{
    ClientConfig, RootCertStore,
    pki_types::{CertificateDer, ServerName, pem::PemObject},
};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::TcpStream,
    spawn,
    sync::Notify,
    task::JoinHandle,
    time::{sleep, timeout},
};

pub(crate) type ClientTlsStream = tokio_rustls::client::TlsStream<TcpStream>;
