mod config;
mod context;
mod error;
mod framing;
mod protocol;
mod route;
mod server;
#[cfg(target_os = "linux")]
mod uring;

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

#[cfg(target_os = "linux")]
use std::{
    future::pending,
    panic::{catch_unwind, resume_unwind},
};

use bytes::{Buf, Bytes};
use h2::{Error, client::ResponseFuture, client::handshake};
use h3::{
    client::RequestStream,
    client::new,
    error::{Code, StreamError},
};
use h3_quinn::{BidiStream, OpenStreams};
use http::Version;
use quinn::{Connecting, crypto::rustls::QuicClientConfig};
use rustls::{
    ClientConfig, RootCertStore,
    pki_types::{CertificateDer, InvalidDnsNameError, ServerName, pem::PemObject},
};
use socket2::SockRef;
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::TcpStream,
    spawn,
    sync::Notify,
    task::JoinHandle,
    time::{sleep, timeout},
};
use tokio_rustls::{Connect, TlsConnector, client::TlsStream};

pub(crate) type ClientTlsStream = TlsStream<TcpStream>;
