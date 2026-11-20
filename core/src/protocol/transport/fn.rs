use super::*;

/// Takes a pooled TLS record buffer sized to `size` bytes.
///
/// # Arguments
///
/// - `usize` - Minimum initialized length required.
///
/// # Returns
///
/// - `Vec<u8>` - A reusable buffer of at least `size` bytes.
#[inline(always)]
pub(crate) fn take_tls_buffer(size: usize) -> Vec<u8> {
    TLS_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| {
        let mut buf: Vec<u8> = pool.borrow_mut().pop().unwrap_or_else(|| vec![0; size]);
        if buf.len() < size {
            buf.resize(size, 0);
        }
        buf
    })
}

/// Returns a TLS record buffer to the worker-local pool if within size limits.
///
/// # Arguments
///
/// - `Vec<u8>` - The buffer to recycle.
#[inline(always)]
pub(crate) fn return_tls_buffer(buf: Vec<u8>) {
    if buf.capacity() == 0 || buf.capacity() > TLS_MAX_INCOMING_BUFFER {
        return;
    }
    TLS_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| {
        let mut pool: RefMut<'_, Vec<Vec<u8>>> = pool.borrow_mut();
        if pool.len() < MAX_POOLED_TLS_BUFFERS {
            pool.push(buf);
        }
    });
}

/// Runs the TLS server handshake over `tcp` using [`UnbufferedServerConnection`] and pooled buffers.
///
/// # Arguments
///
/// - `Arc<TlsServerConfig>` - The validated TLS server configuration.
/// - `T` - The accepted stream transport.
///
/// # Returns
///
/// - `Result<TlsTransport<T>, io::Error>` - The negotiated zero-allocation-write TLS transport.
pub(crate) async fn accept_unbuffered_tls<T>(
    config: Arc<TlsServerConfig>,
    mut tcp: T,
) -> Result<TlsTransport<T>, io::Error>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    let mut conn: UnbufferedServerConnection =
        UnbufferedServerConnection::new(config).map_err(io::Error::other)?;
    let mut in_buf: Vec<u8> = take_tls_buffer(TLS_RECORD_BUFFER_SIZE);
    let mut in_len: usize = 0;
    let mut out_buf: Vec<u8> = take_tls_buffer(TLS_RECORD_BUFFER_SIZE);
    let mut out_len: usize = 0;
    let mut plain_buf: Vec<u8> = Vec::new();
    let result: Result<(), io::Error> = loop {
        let UnbufferedStatus { discard, state } = conn.process_tls_records(&mut in_buf[..in_len]);
        let outcome: Result<bool, io::Error> = match state {
            Ok(ConnectionState::EncodeTlsData(mut enc)) => loop {
                match enc.encode(&mut out_buf[out_len..]) {
                    Ok(written) => {
                        out_len += written;
                        break Ok(false);
                    }
                    Err(EncodeError::InsufficientSize(need)) => {
                        let required: usize = out_len + need.required_size;
                        if required > TLS_MAX_INCOMING_BUFFER * 2 {
                            break Err(io::Error::other("tls handshake output too large"));
                        }
                        out_buf.resize(required, 0);
                    }
                    Err(error) => break Err(io::Error::other(format!("{error:?}"))),
                }
            },
            Ok(ConnectionState::TransmitTlsData(transmit)) => {
                let write_res: Result<(), io::Error> = tcp.write_all(&out_buf[..out_len]).await;
                out_len = 0;
                transmit.done();
                write_res.map(|()| false)
            }
            Ok(ConnectionState::BlockedHandshake) => {
                if discard > 0 {
                    in_buf.copy_within(discard..in_len, 0);
                    in_len -= discard;
                }
                if in_len == in_buf.len() {
                    if in_buf.len() >= TLS_MAX_INCOMING_BUFFER {
                        break Err(io::Error::other("tls handshake record too large"));
                    }
                    in_buf.resize((in_buf.len() * 2).min(TLS_MAX_INCOMING_BUFFER), 0);
                }
                match tcp.read(&mut in_buf[in_len..]).await {
                    Ok(0) => {
                        break Err(io::Error::new(
                            io::ErrorKind::UnexpectedEof,
                            "tls handshake eof",
                        ));
                    }
                    Ok(n) => {
                        in_len += n;
                        continue;
                    }
                    Err(error) => break Err(error),
                }
            }
            Ok(ConnectionState::WriteTraffic(_)) => Ok(true),
            Ok(ConnectionState::ReadTraffic(mut rt)) => {
                let mut err: Option<io::Error> = None;
                while let Some(rec) = rt.next_record() {
                    match rec {
                        Ok(app) => plain_buf.extend_from_slice(app.payload),
                        Err(error) => {
                            err = Some(io::Error::other(error));
                            break;
                        }
                    }
                }
                match err {
                    Some(error) => Err(error),
                    None => Ok(true),
                }
            }
            Ok(ConnectionState::ReadEarlyData(mut red)) => {
                while red.next_record().is_some() {}
                Ok(false)
            }
            Ok(ConnectionState::PeerClosed | ConnectionState::Closed) => Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "tls closed during handshake",
            )),
            Err(error) => {
                if discard > 0 {
                    in_buf.copy_within(discard..in_len, 0);
                    in_len -= discard;
                }
                // Best-effort flush any fatal TLS alert so peers see the alert reason cleanly.
                let UnbufferedStatus {
                    state: alert_state, ..
                } = conn.process_tls_records(&mut []);
                if let Ok(ConnectionState::EncodeTlsData(mut enc)) = alert_state
                    && let Ok(n) = enc.encode(&mut out_buf[out_len..])
                {
                    out_len += n;
                    let _: Result<(), io::Error> = tcp.write_all(&out_buf[..out_len]).await;
                }
                break Err(io::Error::other(error));
            }
            Ok(_) => Err(io::Error::other("unsupported tls handshake state")),
        };
        if discard > 0 {
            in_buf.copy_within(discard..in_len, 0);
            in_len -= discard;
        }
        match outcome {
            Ok(true) => break Ok(()),
            Ok(false) => {}
            Err(error) => break Err(error),
        }
    };
    if let Err(error) = result {
        return_tls_buffer(in_buf);
        return_tls_buffer(out_buf);
        return Err(error);
    }
    Ok(TlsTransport {
        tcp,
        conn,
        in_buf,
        in_len,
        plain_buf,
        plain_pos: 0,
        out_buf,
        out_pos: 0,
        out_len,
        peer_closed: false,
        write_closed: false,
    })
}
