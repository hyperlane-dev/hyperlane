use super::*;

/// Takes a recycled HTTP/3 byte buffer from the worker-local pool.
///
/// # Returns
///
/// - `Vec<u8>` - An empty byte vector retaining pooled capacity when available.
#[inline(always)]
pub(crate) fn take_h3_buffer() -> Vec<u8> {
    H3_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| {
        let mut buf: Vec<u8> = pool
            .borrow_mut()
            .pop()
            .unwrap_or_else(|| Vec::with_capacity(H3_READ_CHUNK_SIZE));
        buf.clear();
        buf
    })
}

/// Returns a reusable HTTP/3 byte buffer to the worker-local pool.
///
/// # Arguments
///
/// - `Vec<u8>` - The buffer to recycle.
#[inline(always)]
pub(crate) fn return_h3_buffer(mut buffer: Vec<u8>) {
    if buffer.capacity() == 0 || buffer.capacity() > MAX_POOLED_BUFFER_CAPACITY {
        return;
    }
    buffer.clear();
    H3_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| {
        let mut pool: RefMut<'_, Vec<Vec<u8>>> = pool.borrow_mut();
        if pool.len() < MAX_POOLED_H3_BUFFERS {
            pool.push(buffer);
        }
    });
}

/// Reports whether an HTTP/3 frame type is a reserved GREASE type (`0x1f * N + 0x21`).
///
/// # Arguments
///
/// - `u64` - The decoded frame type identifier.
///
/// # Returns
///
/// - `bool` - `true` when the frame must be ignored.
#[inline(always)]
fn is_grease_id(id: u64) -> bool {
    id >= 0x21 && (id - 0x21).is_multiple_of(0x1f)
}

/// Serves one QUIC connection speaking HTTP/3 until it closes.
///
/// Opens the server unidirectional control stream with a pre-encoded empty
/// `SETTINGS` frame and polls incoming bidirectional request streams,
/// unidirectional peer streams, and active request futures concurrently within
/// a single task without per-stream `tokio::spawn` overhead.
///
/// # Arguments
///
/// - `ConnectionEnv` - The shared server, compiled pipeline, limits and protocol.
/// - `StreamPeers` - The local and remote addresses of the QUIC connection.
/// - `QuicConnection` - The established QUIC connection.
pub(crate) async fn serve_h3_connection(
    env: ConnectionEnv,
    peers: StreamPeers,
    connection: QuicConnection,
) {
    let mut control_stream: Option<QuicSendStream> = match connection.open_uni().await {
        Ok(mut send) => {
            if send.write_all(H3_SERVER_CONTROL_SETTINGS).await.is_err() {
                return;
            }
            Some(send)
        }
        Err(_) => return,
    };
    let mut requests: FuturesUnordered<_> = FuturesUnordered::new();
    let mut uni_tasks: FuturesUnordered<_> = FuturesUnordered::new();
    let control_state: Arc<H3ControlState> = Arc::new(H3ControlState::default());
    loop {
        tokio::select! {
            biased;
            _ = requests.next(), if !requests.is_empty() => {}
            _ = uni_tasks.next(), if !uni_tasks.is_empty() => {}
            accepted = connection.accept_bi() => {
                match accepted {
                    Ok((send, recv)) => {
                        requests.push(
                            AssertUnwindSafe(h3_request_task(
                                H3RequestEnv {
                                    shared: Arc::clone(&env.shared),
                                    peers,
                                },
                                send,
                                recv,
                            ))
                            .catch_unwind(),
                        );
                    }
                    Err(_) => break,
                }
            }
            uni = connection.accept_uni() => {
                match uni {
                    Ok(recv) => {
                        uni_tasks.push(handle_h3_uni_stream(
                            connection.clone(),
                            recv,
                            Arc::clone(&control_state),
                        ));
                    }
                    Err(_) => break,
                }
            }
        }
    }
    let _: Option<QuicSendStream> = control_stream.take();
}

/// Validates an HTTP/3 `SETTINGS` frame payload per RFC 9114 §7.2.4.
///
/// # Arguments
///
/// - `&[u8]` - The `SETTINGS` frame payload bytes.
///
/// # Returns
///
/// - `Result<(), u32>` - `Ok(())` when valid, or the RFC 9114 HTTP/3 error code to close with.
fn validate_h3_settings_payload(payload: &[u8]) -> Result<(), u32> {
    let mut pos: usize = 0;
    let mut seen: [u64; 16] = [0; 16];
    let mut seen_len: usize = 0;
    let mut overflow: Option<HashSet<u64>> = None;
    while pos < payload.len() {
        let Some((id, id_len)) = decode_varint(&payload[pos..]) else {
            return Err(H3_FRAME_ERROR);
        };
        pos += id_len;
        let Some((_, val_len)) = decode_varint(&payload[pos..]) else {
            return Err(H3_FRAME_ERROR);
        };
        pos += val_len;
        if (0x02..=0x05).contains(&id) {
            return Err(H3_SETTINGS_ERROR);
        }
        if seen[..seen_len].contains(&id) {
            return Err(H3_SETTINGS_ERROR);
        }
        if seen_len < seen.len() {
            seen[seen_len] = id;
            seen_len += 1;
        } else {
            let set: &mut HashSet<u64> = overflow.get_or_insert_with(HashSet::new);
            if !set.insert(id) {
                return Err(H3_SETTINGS_ERROR);
            }
        }
    }
    Ok(())
}

/// Validates and drains the peer's HTTP/3 unidirectional control stream per RFC 9114 §6.2.1.
///
/// # Arguments
///
/// - `&QuicConnection` - The owning QUIC connection closed on any control-stream violation.
/// - `&mut QuicRecvStream` - The peer's control stream after its stream-type varint.
async fn drive_h3_control_stream(connection: &QuicConnection, recv: &mut QuicRecvStream) {
    let mut buf: Vec<u8> = take_h3_buffer();
    let mut pos: usize = 0;
    let mut settings_seen: bool = false;
    loop {
        if pos == buf.len() {
            buf.clear();
            pos = 0;
        }
        let header: Option<(u64, usize)> =
            match read_h3_frame_header(recv, &mut buf, &mut pos).await {
                Ok(Some(pair)) => Some(pair),
                Ok(None) => None,
                Err(_) => {
                    return_h3_buffer(buf);
                    connection.close(VarInt::from_u32(H3_FRAME_ERROR), b"malformed control frame");
                    return;
                }
            };
        let Some((frame_type, payload_len)) = header else {
            return_h3_buffer(buf);
            connection.close(
                VarInt::from_u32(H3_CLOSED_CRITICAL_STREAM),
                b"control stream closed",
            );
            return;
        };
        if !settings_seen {
            if frame_type != H3_FRAME_SETTINGS {
                return_h3_buffer(buf);
                connection.close(
                    VarInt::from_u32(H3_MISSING_SETTINGS),
                    b"first control frame must be settings",
                );
                return;
            }
            if payload_len > H3_MAX_SETTINGS_FRAME_LEN {
                return_h3_buffer(buf);
                connection.close(
                    VarInt::from_u32(H3_FRAME_ERROR),
                    b"settings frame too large",
                );
                return;
            }
            match ensure_h3_bytes(recv, &mut buf, pos, payload_len).await {
                Ok(true) => {}
                Ok(false) => {
                    return_h3_buffer(buf);
                    connection.close(
                        VarInt::from_u32(H3_CLOSED_CRITICAL_STREAM),
                        b"control stream closed during settings",
                    );
                    return;
                }
                Err(_) => {
                    return_h3_buffer(buf);
                    connection.close(VarInt::from_u32(H3_FRAME_ERROR), b"control read error");
                    return;
                }
            }
            let end: usize = pos + payload_len;
            if let Err(code) = validate_h3_settings_payload(&buf[pos..end]) {
                return_h3_buffer(buf);
                connection.close(VarInt::from_u32(code), b"invalid settings payload");
                return;
            }
            pos = end;
            settings_seen = true;
            continue;
        }
        match frame_type {
            H3_FRAME_SETTINGS | H3_FRAME_DATA | H3_FRAME_HEADERS | H3_FRAME_PUSH_PROMISE => {
                return_h3_buffer(buf);
                connection.close(
                    VarInt::from_u32(H3_FRAME_UNEXPECTED),
                    b"unexpected frame on control stream",
                );
                return;
            }
            _ => {
                if skip_h3_payload(recv, &mut buf, &mut pos, payload_len)
                    .await
                    .is_err()
                {
                    return_h3_buffer(buf);
                    connection.close(
                        VarInt::from_u32(H3_CLOSED_CRITICAL_STREAM),
                        b"control stream truncated",
                    );
                    return;
                }
            }
        }
    }
}

/// Identifies and enforces RFC 9114 §6.2 rules on an incoming unidirectional peer stream.
///
/// # Arguments
///
/// - `QuicConnection` - The owning QUIC connection for closing on fatal stream errors.
/// - `QuicRecvStream` - The incoming unidirectional stream.
/// - `Arc<H3ControlState>` - Shared atomic flags recording peer critical streams immediately.
async fn handle_h3_uni_stream(
    connection: QuicConnection,
    mut recv: QuicRecvStream,
    state: Arc<H3ControlState>,
) {
    let mut header_buf: [u8; 8] = [0; 8];
    let mut filled: usize = 0;
    let stream_type: u64 = loop {
        if filled > 0
            && let Some((val, _)) = decode_varint(&header_buf[..filled])
        {
            break val;
        }
        if filled == header_buf.len() {
            return;
        }
        match recv.read(&mut header_buf[filled..filled + 1]).await {
            Ok(Some(n)) if n > 0 => filled += n,
            _ => return,
        }
    };
    match stream_type {
        H3_STREAM_CONTROL => {
            if state.control_seen.swap(true, AtomicOrdering::AcqRel) {
                connection.close(
                    VarInt::from_u32(H3_STREAM_CREATION_ERROR),
                    b"duplicate control stream",
                );
                return;
            }
            drive_h3_control_stream(&connection, &mut recv).await;
        }
        H3_STREAM_QPACK_ENCODER => {
            if state.qpack_enc_seen.swap(true, AtomicOrdering::AcqRel) {
                connection.close(
                    VarInt::from_u32(H3_STREAM_CREATION_ERROR),
                    b"duplicate qpack encoder stream",
                );
                return;
            }
            let mut buf: [u8; 64] = [0; 64];
            loop {
                match recv.read(&mut buf).await {
                    Ok(Some(0)) | Ok(None) => {
                        connection.close(
                            VarInt::from_u32(H3_CLOSED_CRITICAL_STREAM),
                            b"qpack encoder stream closed",
                        );
                        return;
                    }
                    Ok(Some(_)) => {}
                    Err(_) => return,
                }
            }
        }
        H3_STREAM_QPACK_DECODER => {
            if state.qpack_dec_seen.swap(true, AtomicOrdering::AcqRel) {
                connection.close(
                    VarInt::from_u32(H3_STREAM_CREATION_ERROR),
                    b"duplicate qpack decoder stream",
                );
                return;
            }
            let mut buf: [u8; 64] = [0; 64];
            loop {
                match recv.read(&mut buf).await {
                    Ok(Some(0)) | Ok(None) => {
                        connection.close(
                            VarInt::from_u32(H3_CLOSED_CRITICAL_STREAM),
                            b"qpack decoder stream closed",
                        );
                        return;
                    }
                    Ok(Some(_)) => {}
                    Err(_) => return,
                }
            }
        }
        H3_STREAM_PUSH => {
            connection.close(
                VarInt::from_u32(H3_STREAM_CREATION_ERROR),
                b"client push stream forbidden",
            );
        }
        _ => {
            let _: Result<(), _> = recv.stop(VarInt::from_u32(H3_NO_ERROR));
        }
    }
}

/// Ensures at least `needed` unread bytes are buffered in `buf[pos..]`, returning `false` on EOF.
///
/// # Arguments
///
/// - `&mut QuicRecvStream` - The QUIC receive stream.
/// - `&mut Vec<u8>` - The pooled read buffer.
/// - `usize` - Current unread cursor inside `buf`.
/// - `usize` - Minimum unread bytes required.
///
/// # Returns
///
/// - `Result<bool, RequestError>` - `Ok(true)` when `needed` bytes are available, `Ok(false)` on clean EOF.
#[inline(always)]
async fn ensure_h3_bytes(
    recv: &mut QuicRecvStream,
    buf: &mut Vec<u8>,
    pos: usize,
    needed: usize,
) -> Result<bool, RequestError> {
    while buf.len() - pos < needed {
        let old_len: usize = buf.len();
        let grow: usize = H3_READ_CHUNK_SIZE.max(needed - (old_len - pos));
        buf.resize(old_len + grow, 0);
        match recv.read(&mut buf[old_len..]).await {
            Ok(Some(n)) if n > 0 => {
                buf.truncate(old_len + n);
            }
            Ok(Some(_)) | Ok(None) => {
                buf.truncate(old_len);
                return Ok(false);
            }
            Err(error) => {
                buf.truncate(old_len);
                return Err(RequestError::Request(error.to_string()));
            }
        }
    }
    Ok(true)
}

/// Reads the next HTTP/3 frame header `(frame_type, payload_len)` from `buf` / `recv`.
///
/// # Arguments
///
/// - `&mut QuicRecvStream` - The QUIC receive stream.
/// - `&mut Vec<u8>` - The pooled read buffer.
/// - `&mut usize` - Read cursor inside `buf`, advanced past the frame header.
///
/// # Returns
///
/// - `Result<Option<(u64, usize)>, RequestError>` - `None` on clean stream EOF before a frame.
async fn read_h3_frame_header(
    recv: &mut QuicRecvStream,
    buf: &mut Vec<u8>,
    pos: &mut usize,
) -> Result<Option<(u64, usize)>, RequestError> {
    let frame_type: u64 = loop {
        if *pos < buf.len() {
            let need: usize = 1usize << (buf[*pos] >> 6);
            if buf.len() - *pos >= need
                && let Some((val, consumed)) = decode_varint(&buf[*pos..])
            {
                *pos += consumed;
                break val;
            }
        }
        let need_more: usize = if *pos < buf.len() {
            1usize << (buf[*pos] >> 6)
        } else {
            1
        };
        if !ensure_h3_bytes(recv, buf, *pos, need_more).await? {
            if *pos == buf.len() {
                return Ok(None);
            }
            return Err(RequestError::HttpRead(HttpStatus::BadRequest));
        }
    };
    let payload_len_u64: u64 = loop {
        if *pos < buf.len() {
            let need: usize = 1usize << (buf[*pos] >> 6);
            if buf.len() - *pos >= need
                && let Some((val, consumed)) = decode_varint(&buf[*pos..])
            {
                *pos += consumed;
                break val;
            }
        }
        let need_more: usize = if *pos < buf.len() {
            1usize << (buf[*pos] >> 6)
        } else {
            1
        };
        if !ensure_h3_bytes(recv, buf, *pos, need_more).await? {
            return Err(RequestError::HttpRead(HttpStatus::BadRequest));
        }
    };
    let payload_len: usize = usize::try_from(payload_len_u64)
        .map_err(|_| RequestError::HttpRead(HttpStatus::BadRequest))?;
    Ok(Some((frame_type, payload_len)))
}

/// Skips `remaining` payload bytes belonging to an ignored GREASE or trailer frame.
///
/// # Arguments
///
/// - `&mut QuicRecvStream` - The QUIC receive stream.
/// - `&mut Vec<u8>` - The pooled read buffer.
/// - `&mut usize` - Read cursor inside `buf`.
/// - `usize` - Number of bytes to discard.
///
/// # Returns
///
/// - `Result<(), RequestError>` - `Ok(())` once discarded.
async fn skip_h3_payload(
    recv: &mut QuicRecvStream,
    buf: &mut Vec<u8>,
    pos: &mut usize,
    mut remaining: usize,
) -> Result<(), RequestError> {
    while remaining > 0 {
        let avail: usize = buf.len() - *pos;
        if avail > 0 {
            let take: usize = avail.min(remaining);
            *pos += take;
            remaining -= take;
            continue;
        }
        buf.clear();
        *pos = 0;
        if !ensure_h3_bytes(recv, buf, 0, 1).await? {
            return Err(RequestError::HttpRead(HttpStatus::BadRequest));
        }
    }
    Ok(())
}

/// Reads and decodes the initial HTTP/3 `HEADERS` frame directly into `request` and `target`.
///
/// # Arguments
///
/// - `&mut QuicRecvStream` - The QUIC receive stream.
/// - `&mut Vec<u8>` - The pooled read buffer.
/// - `&mut usize` - Read cursor inside `buf`.
/// - `&RequestConfig` - Security limits.
/// - `&mut Request` - The destination framework request.
/// - `&mut String` - The destination target buffer.
///
/// # Returns
///
/// - `Result<Option<H3DecodedRequestMeta>, RequestError>` - `None` if peer closed before sending headers.
async fn read_h3_request_headers(
    recv: &mut QuicRecvStream,
    buf: &mut Vec<u8>,
    pos: &mut usize,
    config: &RequestConfig,
    request: &mut Request,
    target: &mut String,
) -> Result<Option<H3DecodedRequestMeta>, RequestError> {
    loop {
        let Some((frame_type, payload_len)) = read_h3_frame_header(recv, buf, pos).await? else {
            return Ok(None);
        };
        if is_grease_id(frame_type) {
            skip_h3_payload(recv, buf, pos, payload_len).await?;
            continue;
        }
        if frame_type != H3_FRAME_HEADERS {
            return Err(RequestError::HttpRead(HttpStatus::BadRequest));
        }
        if payload_len > H3_MAX_HEADERS_FRAME_LEN {
            return Err(RequestError::HeaderValueTooLong(
                HttpStatus::RequestHeaderFieldsTooLarge,
            ));
        }
        if !ensure_h3_bytes(recv, buf, *pos, payload_len).await? {
            return Err(RequestError::HttpRead(HttpStatus::BadRequest));
        }
        let payload_end: usize = *pos + payload_len;
        let meta: H3DecodedRequestMeta =
            decode_qpack_request(&buf[*pos..payload_end], config, request, target)?;
        *pos = payload_end;
        return Ok(Some(meta));
    }
}

/// Collects HTTP/3 `DATA` frames into `body` while enforcing size limits and `Expect: 100-continue`.
///
/// # Arguments
///
/// - `&mut QuicSendStream` - The send stream for optional `100 Continue` emission.
/// - `&mut QuicRecvStream` - The receive stream carrying `DATA` frames.
/// - `&mut Vec<u8>` - The pooled read buffer containing any bytes already read after `HEADERS`.
/// - `&mut usize` - Read cursor inside `buf`.
/// - `usize` - Maximum permitted body length.
/// - `&mut Vec<u8>` - Destination request body buffer.
/// - `bool` - Whether to send `100 Continue` before reading body bytes.
///
/// # Returns
///
/// - `Result<(), RequestError>` - `Ok(())` when the request stream ends cleanly.
async fn collect_h3_data_frames(
    send: &mut QuicSendStream,
    recv: &mut QuicRecvStream,
    buf: &mut Vec<u8>,
    pos: &mut usize,
    max_body_size: usize,
    body: &mut Vec<u8>,
    send_continue: bool,
) -> Result<(), RequestError> {
    if send_continue {
        send.write_all(H3_CONTINUE_FRAME)
            .await
            .map_err(|error: QuicWriteError| RequestError::Request(error.to_string()))?;
    }
    loop {
        if *pos == buf.len() {
            buf.clear();
            *pos = 0;
        }
        let Some((frame_type, payload_len)) = read_h3_frame_header(recv, buf, pos).await? else {
            return Ok(());
        };
        match frame_type {
            H3_FRAME_DATA => {
                if max_body_size != DEFAULT_LOW_SECURITY_MAX_BODY_SIZE
                    && body.len().saturating_add(payload_len) > max_body_size
                {
                    let _: Result<(), _> = recv.stop(VarInt::from_u32(H3_NO_ERROR));
                    return Err(RequestError::ContentLengthTooLarge(
                        HttpStatus::PayloadTooLarge,
                    ));
                }
                let mut remaining: usize = payload_len;
                while remaining > 0 {
                    let avail: usize = buf.len() - *pos;
                    if avail > 0 {
                        let take: usize = avail.min(remaining);
                        body.extend_from_slice(&buf[*pos..*pos + take]);
                        *pos += take;
                        remaining -= take;
                        continue;
                    }
                    let old_len: usize = body.len();
                    let chunk_len: usize = remaining.min(MAX_POOLED_BUFFER_CAPACITY);
                    body.resize(old_len + chunk_len, 0);
                    match recv.read(&mut body[old_len..]).await {
                        Ok(Some(n)) if n > 0 => {
                            body.truncate(old_len + n);
                            remaining -= n;
                        }
                        Ok(Some(_)) | Ok(None) => {
                            body.truncate(old_len);
                            return Err(RequestError::HttpRead(HttpStatus::BadRequest));
                        }
                        Err(error) => {
                            body.truncate(old_len);
                            return Err(RequestError::Request(error.to_string()));
                        }
                    }
                }
            }
            H3_FRAME_HEADERS => {
                skip_h3_payload(recv, buf, pos, payload_len).await?;
            }
            other if is_grease_id(other) => {
                skip_h3_payload(recv, buf, pos, payload_len).await?;
            }
            H3_FRAME_SETTINGS
            | H3_FRAME_GOAWAY
            | H3_FRAME_CANCEL_PUSH
            | H3_FRAME_PUSH_PROMISE
            | H3_FRAME_MAX_PUSH_ID => {
                let _: Result<(), _> = recv.stop(VarInt::from_u32(H3_FRAME_UNEXPECTED));
                return Err(RequestError::HttpRead(HttpStatus::BadRequest));
            }
            _ => {
                skip_h3_payload(recv, buf, pos, payload_len).await?;
            }
        }
    }
}

/// Runs the framework lifecycle directly over one QUIC bidirectional stream speaking HTTP/3.
///
/// # Arguments
///
/// - `H3RequestEnv` - Shared server state, compiled pipeline and connection addresses.
/// - `QuicSendStream` - The QUIC send stream for the response.
/// - `QuicRecvStream` - The QUIC receive stream for the request.
pub(crate) async fn h3_request_task(
    env: H3RequestEnv,
    mut send_stream: QuicSendStream,
    mut recv_stream: QuicRecvStream,
) {
    let H3RequestEnv { shared, peers } = env;
    let server: &Server = &shared.server;
    let request_config: RequestConfig = shared.request_config;
    let mut read_buf: Vec<u8> = take_h3_buffer();
    let mut read_pos: usize = 0;
    let mut ctx: Context = take_pooled_context();
    let mut target: String = take_target_buffer();
    let read_timeout_ms: u64 = request_config.get_read_timeout_ms();
    let head_result: Result<Option<H3DecodedRequestMeta>, RequestError> =
        if read_timeout_ms == DEFAULT_LOW_SECURITY_READ_TIMEOUT_MS {
            read_h3_request_headers(
                &mut recv_stream,
                &mut read_buf,
                &mut read_pos,
                &request_config,
                ctx.get_mut_request(),
                &mut target,
            )
            .await
        } else {
            let mut reading: Pin<&mut _> = pin!(read_h3_request_headers(
                &mut recv_stream,
                &mut read_buf,
                &mut read_pos,
                &request_config,
                ctx.get_mut_request(),
                &mut target,
            ));
            if let Some(done) = poll_once_protocol(reading.as_mut()).await {
                done
            } else {
                match timeout(Duration::from_millis(read_timeout_ms), reading).await {
                    Ok(done) => done,
                    Err(_) => Err(RequestError::ReadTimeout(HttpStatus::RequestTimeout)),
                }
            }
        };
    let meta: H3DecodedRequestMeta = match head_result {
        Ok(Some(meta)) => meta,
        Ok(None) => {
            return_h3_buffer(read_buf);
            return_target_buffer(target);
            return_pooled_context(ctx);
            return;
        }
        Err(error) => {
            let _: Result<(), _> = recv_stream.stop(VarInt::from_u32(H3_NO_ERROR));
            return_h3_buffer(read_buf);
            return_target_buffer(target);
            h3_fail_request(
                server,
                request_config,
                peers,
                send_stream,
                &mut ctx,
                HttpVersion::Http3,
                error,
            )
            .await;
            return_pooled_context(ctx);
            return;
        }
    };
    let max_body_size: usize = request_config.get_max_body_size();
    let body: &mut Vec<u8> = ctx.get_mut_request().get_mut_body();
    body.clear();
    if let Some(expected) = meta.declared_content_length
        && expected > 0
    {
        body.reserve(expected.min(MAX_POOLED_BUFFER_CAPACITY));
    }
    let body_result: Result<(), RequestError> =
        if read_timeout_ms == DEFAULT_LOW_SECURITY_READ_TIMEOUT_MS {
            collect_h3_data_frames(
                &mut send_stream,
                &mut recv_stream,
                &mut read_buf,
                &mut read_pos,
                max_body_size,
                body,
                meta.expect_continue,
            )
            .await
        } else {
            let mut reading: Pin<&mut _> = pin!(collect_h3_data_frames(
                &mut send_stream,
                &mut recv_stream,
                &mut read_buf,
                &mut read_pos,
                max_body_size,
                body,
                meta.expect_continue,
            ));
            if let Some(done) = poll_once_protocol(reading.as_mut()).await {
                done
            } else {
                match timeout(Duration::from_millis(read_timeout_ms), reading).await {
                    Ok(done) => done,
                    Err(_) => Err(RequestError::ReadTimeout(HttpStatus::RequestTimeout)),
                }
            }
        };
    return_h3_buffer(read_buf);
    let body_check: Result<(), RequestError> =
        body_result.and_then(|()| match meta.declared_content_length {
            None => Ok(()),
            Some(declared) => match ctx.get_request().get_body().len().cmp(&declared) {
                Ordering::Equal => Ok(()),
                _ => Err(RequestError::Request(CONTENT_LENGTH_MISMATCH.to_owned())),
            },
        });
    if let Err(error) = body_check {
        let _: Result<(), _> = recv_stream.stop(VarInt::from_u32(H3_NO_ERROR));
        return_target_buffer(target);
        h3_fail_request(
            server,
            request_config,
            peers,
            send_stream,
            &mut ctx,
            HttpVersion::Http3,
            error,
        )
        .await;
        return_pooled_context(ctx);
        return;
    }
    let mut stream: Stream = Stream::from_transport(
        H3DirectTransport::new(send_stream, peers, meta.is_head),
        request_config,
        false,
    );
    server
        .run_protocol_lifecycle_in_place(&shared.pipeline, &mut stream, &mut ctx, target)
        .await;
    let _: Result<(), io::Error> = stream.get_mut_stream().shutdown().await;
    return_pooled_context(ctx);
}

/// Answers an HTTP/3 request that failed before a handler could run.
///
/// # Arguments
///
/// - `&Server` - The server owning the request error hooks.
/// - `RequestConfig` - The parsing limits and read timeout.
/// - `StreamPeers` - The connection's socket addresses.
/// - `QuicSendStream` - The QUIC send stream for the error response.
/// - `&mut Context` - The context passed to error hooks.
/// - `HttpVersion` - The real protocol version to stamp on the error context.
/// - `RequestError` - The failure that stopped the request.
#[cold]
pub(crate) async fn h3_fail_request(
    server: &Server,
    request_config: RequestConfig,
    peers: StreamPeers,
    send_stream: QuicSendStream,
    ctx: &mut Context,
    version: HttpVersion,
    error: RequestError,
) {
    ctx.get_mut_request().reset().set_version(version.clone());
    ctx.get_mut_response().reset().set_version(version.clone());
    ctx.clear_route_params();
    ctx.reset_request_attributes();
    let mut stream: Stream = Stream::from_transport(
        H3DirectTransport::new(send_stream, peers, false),
        request_config,
        false,
    );
    reply_to_request_error(server, &mut stream, ctx, &error, Some(version)).await;
    let _: Result<(), io::Error> = stream.get_mut_stream().shutdown().await;
}
