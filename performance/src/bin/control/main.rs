//! hyperlane-performance-control
//!
//! The machine-ceiling reference of the hyperlane benchmark harness.
//!
//! It answers the same request shapes as the server under test, with one read
//! and one write per request and no framework in the way. The number it
//! reaches is the ceiling the machine allows, so a drop in it means the
//! machine moved rather than the framework.
mod config;
mod r#protocol;

pub(crate) use config::*;
pub(crate) use r#protocol::*;

use std::{env::var, io::Error, net::SocketAddr, process::exit, sync::Arc};

use bytes::{Buf, Bytes};
use h2::{RecvStream, SendStream, server::handshake};
use h3::{
    error::StreamError,
    server::{RequestResolver, RequestStream, builder},
};
use h3_quinn::BidiStream;
use http::{HeaderValue, Response, header::CONTENT_LENGTH};
use quinn::{Connection, Endpoint, crypto::rustls::QuicServerConfig};
use rustls::{
    ServerConfig, SupportedProtocolVersion,
    crypto::ring::default_provider,
    pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject},
    version::{TLS12, TLS13},
};
use rustls_pki_types::pem;
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    spawn,
};
use tokio_rustls::{TlsAcceptor, server::TlsStream};

/// The HTTP response construction error used by the protocol module.
pub(crate) type HttpError = http::Error;
/// The HTTP/2 error, distinct from the socket I/O error.
pub(crate) type Http2Error = h2::Error;
/// The HTTP/2 server connection, distinct from the QUIC connection.
pub(crate) type Http2Connection = h2::server::Connection<Box<dyn Transport>, Bytes>;
/// The HTTP/3 transport adapter, distinct from the QUIC connection.
pub(crate) type Http3Transport = h3_quinn::Connection;
/// The HTTP/3 server connection, distinct from the QUIC connection.
pub(crate) type Http3Connection = h3::server::Connection<Http3Transport, Bytes>;
/// The QUIC endpoint configuration, distinct from the TLS configuration.
pub(crate) type QuicEndpointConfig = quinn::ServerConfig;

/// Starts the control server and serves until it is shut down.
///
/// The address comes from the environment, so the entry point does not have to
/// know it. The protocol comes from the environment too, so one image serves
/// every row of the measurement matrix.
///
/// # Returns
///
/// - `Result<(), String>` - Nothing, or the reason the server could not start.
pub async fn serve() -> Result<(), String> {
    let address: String = match var(ENV_ADDRESS) {
        Ok(address) => address,
        Err(_) => DEFAULT_ADDRESS.to_owned(),
    };
    let protocol: Protocol = match var(ENV_PROTOCOL) {
        Ok(name) => match Protocol::from_name(&name) {
            Some(protocol) => protocol,
            None => return Err(format!("unknown protocol {name}")),
        },
        Err(_) => Protocol::Http1,
    };
    let certificate: Option<String> = var(ENV_TLS_CERTIFICATE).ok();
    let key: Option<String> = var(ENV_TLS_KEY).ok();
    match protocol {
        Protocol::Http3 => serve_http3(&address, certificate, key).await,
        _ => serve_stream(&address, protocol, certificate, key).await,
    }
}

/// Serves the socket-based protocols.
///
/// # Arguments
///
/// - `&str` - The address to bind.
/// - `Protocol` - The protocol to speak.
/// - `Option<String>` - The path of the certificate chain.
/// - `Option<String>` - The path of the private key.
///
/// # Returns
///
/// - `Result<(), String>` - Nothing, or the reason the server could not start.
async fn serve_stream(
    address: &str,
    protocol: Protocol,
    certificate: Option<String>,
    key: Option<String>,
) -> Result<(), String> {
    let listener: TcpListener = match TcpListener::bind(address).await {
        Ok(listener) => listener,
        Err(error) => return Err(error.to_string()),
    };
    let acceptor: Option<TlsAcceptor> = match (protocol.uses_tls(), certificate, key) {
        (true, Some(certificate), Some(key)) => {
            let config: ServerConfig = build_server_config(&certificate, &key, protocol.alpn())?;
            Some(TlsAcceptor::from(Arc::new(config)))
        }
        (true, _, _) => return Err("the TLS protocol needs a certificate and a key".to_owned()),
        _ => None,
    };
    println!("hyperlane-performance-control ready on {address} speaking {protocol:?}");
    loop {
        let (stream, _): (TcpStream, _) = match listener.accept().await {
            Ok(accepted) => accepted,
            Err(error) => return Err(error.to_string()),
        };
        let _: Result<(), Error> = stream.set_nodelay(true);
        let acceptor: Option<TlsAcceptor> = acceptor.clone();
        spawn(async move {
            match acceptor {
                Some(acceptor) => {
                    let session: TlsStream<TcpStream> = match acceptor.accept(stream).await {
                        Ok(session) => session,
                        Err(_) => return,
                    };
                    let transport: Box<dyn Transport> = Box::new(session);
                    let _: Result<(), String> = match protocol {
                        Protocol::H2c | Protocol::H2Tls => serve_http2(transport).await,
                        _ => serve_http1(transport).await,
                    };
                }
                None => {
                    let transport: Box<dyn Transport> = Box::new(stream);
                    let _: Result<(), String> = match protocol {
                        Protocol::H2c | Protocol::H2Tls => serve_http2(transport).await,
                        _ => serve_http1(transport).await,
                    };
                }
            }
        });
    }
}

/// Serves the HTTP/3 protocol over QUIC.
///
/// # Arguments
///
/// - `&str` - The address to bind.
/// - `Option<String>` - The path of the certificate chain.
/// - `Option<String>` - The path of the private key.
///
/// # Returns
///
/// - `Result<(), String>` - Nothing, or the reason the server could not start.
async fn serve_http3(
    address: &str,
    certificate: Option<String>,
    key: Option<String>,
) -> Result<(), String> {
    let (Some(certificate), Some(key)) = (certificate, key) else {
        return Err(ERROR_HTTP3_NEEDS_TLS.to_owned());
    };
    let config: ServerConfig = build_server_config(&certificate, &key, ALPN_HTTP3)?;
    let quic: QuicServerConfig = match QuicServerConfig::try_from(config) {
        Ok(quic) => quic,
        Err(error) => return Err(error.to_string()),
    };
    let bind: SocketAddr = match address.parse() {
        Ok(bind) => bind,
        Err(error) => return Err(error.to_string()),
    };
    let endpoint: Endpoint =
        match Endpoint::server(QuicEndpointConfig::with_crypto(Arc::new(quic)), bind) {
            Ok(endpoint) => endpoint,
            Err(error) => return Err(error.to_string()),
        };
    println!("hyperlane-performance-control ready on {address} speaking Http3");
    while let Some(incoming) = endpoint.accept().await {
        let connection: Connection = match incoming.await {
            Ok(connection) => connection,
            Err(_) => continue,
        };
        spawn(async move {
            let _: Result<(), String> = serve_http3_connection(connection).await;
        });
    }
    Ok(())
}

/// Serves until the process stops.
#[tokio::main]
async fn main() {
    if let Err(error) = serve().await {
        eprintln!("{error}");
        exit(1);
    }
}
