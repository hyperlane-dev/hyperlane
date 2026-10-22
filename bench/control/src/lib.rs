//! bench-control
//!
//! A minimal HTTP/1.1 server used as the environment reference of the
//! hyperlane benchmark harness.
//!
//! It answers the same request shapes as the server under test, with one read
//! and one write per request and no allocation on the hot path. The number it
//! reaches is the ceiling the machine allows, so a drop in it means the
//! machine moved rather than the framework.
mod r#const;

pub use r#const::*;

use std::{env::var, io::Error};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

/// Picks the answer for a request.
///
/// # Arguments
///
/// - `&[u8]` - The request bytes.
///
/// # Returns
///
/// - `&'static [u8]` - The answer.
fn respond_to(request: &[u8]) -> &'static [u8] {
    if request.starts_with(PREFIX_ROOT) {
        return RESPONSE_ROOT;
    }
    if request.starts_with(PREFIX_USER) {
        return RESPONSE_USER;
    }
    if request.starts_with(PREFIX_SEARCH) {
        return RESPONSE_SEARCH;
    }
    if request.starts_with(PREFIX_HEADERS) {
        return RESPONSE_HEADERS;
    }
    RESPONSE_ECHO
}

/// Serves one connection until the peer closes it.
///
/// # Arguments
///
/// - `TcpStream` - The accepted connection.
///
/// # Returns
///
/// - `Result<(), String>` - Nothing, or the reason the connection failed.
async fn serve_connection(mut stream: TcpStream) -> Result<(), String> {
    let _: Result<(), Error> = stream.set_nodelay(true);
    let mut buffer: Vec<u8> = vec![0; READ_BUFFER_SIZE];
    loop {
        let read: usize = match stream.read(&mut buffer).await {
            Ok(read) => read,
            Err(error) => return Err(error.to_string()),
        };
        if read == 0 {
            return Ok(());
        }
        let response: &'static [u8] = respond_to(&buffer[..read]);
        if stream.write_all(response).await.is_err() {
            return Ok(());
        }
    }
}

/// Starts the control server and serves until it is shut down.
///
/// The address comes from the environment, so the entry point does not have to
/// know it.
///
/// # Returns
///
/// - `Result<(), String>` - Nothing, or the reason the server could not start.
pub async fn serve() -> Result<(), String> {
    let address: String = match var(ENV_ADDRESS) {
        Ok(address) => address,
        Err(_) => DEFAULT_ADDRESS.to_owned(),
    };
    println!("bench-control listening on {address}");
    let listener: TcpListener = match TcpListener::bind(&address).await {
        Ok(listener) => listener,
        Err(error) => return Err(error.to_string()),
    };
    loop {
        let (stream, _): (TcpStream, _) = match listener.accept().await {
            Ok(accepted) => accepted,
            Err(error) => return Err(error.to_string()),
        };
        tokio::spawn(async move {
            let _: Result<(), String> = serve_connection(stream).await;
        });
    }
}
