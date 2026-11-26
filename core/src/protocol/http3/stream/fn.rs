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

/// Reports whether a frame type is reserved from HTTP/2 and forbidden in HTTP/3 (RFC 9114 §7.2.8).
///
/// # Arguments
///
/// - `u64` - The decoded frame type identifier.
///
/// # Returns
///
/// - `bool` - `true` when receiving the frame must close the connection with `H3_FRAME_UNEXPECTED`.
#[inline(always)]
fn is_reserved_h2_frame(id: u64) -> bool {
    let low_reserved: bool = id == 0x02 || id == 0x06;
    let high_reserved: bool = id == 0x08 || id == 0x09;
    low_reserved || high_reserved
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
    let shared: &ConnectionShared = &env.shared;
    let control_state: H3ControlState = H3ControlState::default();
    let mut first_req: Pin<&mut Option<_>> = pin!(None);
    let mut extra_reqs: Option<FuturesUnordered<_>> = None;
    let mut first_uni: Pin<&mut Option<_>> = pin!(None);
    let mut extra_unis: Option<FuturesUnordered<_>> = None;
    let mut accept_bi: Pin<&mut Option<_>> = pin!(None);
    let mut accept_uni: Pin<&mut Option<_>> = pin!(None);
    poll_fn(|cx: &mut task::Context<'_>| -> Poll<()> {
        loop {
            let mut progressed: bool = false;
            if let Some(fut) = first_req.as_mut().as_pin_mut()
                && Future::poll(fut, cx).is_ready()
            {
                first_req.set(None);
                progressed = true;
            }
            let mut completed_reqs: usize = 0;
            if let Some(queue) = extra_reqs.as_mut() {
                while !queue.is_empty() && completed_reqs < 64 {
                    match queue.poll_next_unpin(cx) {
                        Poll::Ready(Some(_)) => {
                            completed_reqs += 1;
                            progressed = true;
                        }
                        Poll::Ready(None) | Poll::Pending => break,
                    }
                }
            }
            if let Some(fut) = first_uni.as_mut().as_pin_mut()
                && Future::poll(fut, cx).is_ready()
            {
                first_uni.set(None);
                progressed = true;
            }
            if let Some(queue) = extra_unis.as_mut() {
                while !queue.is_empty() {
                    match queue.poll_next_unpin(cx) {
                        Poll::Ready(Some(())) => {
                            progressed = true;
                        }
                        Poll::Ready(None) | Poll::Pending => break,
                    }
                }
            }
            let mut accepted_bi: usize = 0;
            while accepted_bi < 64 {
                if accept_bi.is_none() {
                    accept_bi.set(Some(connection.accept_bi()));
                }
                let Some(bi_fut) = accept_bi.as_mut().as_pin_mut() else {
                    break;
                };
                match Future::poll(bi_fut, cx) {
                    Poll::Ready(Ok((send, recv))) => {
                        accept_bi.set(None);
                        if first_req.is_none() {
                            first_req.set(Some(
                                AssertUnwindSafe(h3_request_task(
                                    shared,
                                    peers,
                                    &connection,
                                    send,
                                    recv,
                                ))
                                .catch_unwind(),
                            ));
                            if let Some(fut) = first_req.as_mut().as_pin_mut()
                                && Future::poll(fut, cx).is_ready()
                            {
                                first_req.set(None);
                            }
                        } else {
                            extra_reqs.get_or_insert_with(FuturesUnordered::new).push(
                                AssertUnwindSafe(h3_request_task(
                                    shared,
                                    peers,
                                    &connection,
                                    send,
                                    recv,
                                ))
                                .catch_unwind(),
                            );
                        }
                        accepted_bi += 1;
                        progressed = true;
                    }
                    Poll::Ready(Err(_)) => return Poll::Ready(()),
                    Poll::Pending => break,
                }
            }
            let mut accepted_uni: usize = 0;
            while accepted_uni < 8 {
                if accept_uni.is_none() {
                    accept_uni.set(Some(connection.accept_uni()));
                }
                let Some(uni_fut) = accept_uni.as_mut().as_pin_mut() else {
                    break;
                };
                match Future::poll(uni_fut, cx) {
                    Poll::Ready(Ok(recv)) => {
                        accept_uni.set(None);
                        if first_uni.is_none() {
                            first_uni.set(Some(handle_h3_uni_stream(
                                &connection,
                                recv,
                                &control_state,
                            )));
                            if let Some(fut) = first_uni.as_mut().as_pin_mut()
                                && Future::poll(fut, cx).is_ready()
                            {
                                first_uni.set(None);
                            }
                        } else {
                            extra_unis
                                .get_or_insert_with(FuturesUnordered::new)
                                .push(handle_h3_uni_stream(&connection, recv, &control_state));
                        }
                        accepted_uni += 1;
                        progressed = true;
                    }
                    Poll::Ready(Err(_)) => return Poll::Ready(()),
                    Poll::Pending => break,
                }
            }
            if !progressed {
                return Poll::Pending;
            }
            if accepted_bi == 64 || completed_reqs == 64 {
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
        }
    })
    .await;
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

/// Compacts consumed leading bytes from an HTTP/3 read buffer so `buf` stays bounded.
///
/// # Arguments
///
/// - `&mut Vec<u8>` - The pooled HTTP/3 read buffer.
/// - `&mut usize` - Read cursor inside `buf`.
#[inline(always)]
fn compact_h3_buffer(buf: &mut Vec<u8>, pos: &mut usize) {
    if *pos == 0 {
        return;
    }
    if *pos >= buf.len() {
        buf.clear();
        *pos = 0;
    } else if *pos >= H3_READ_CHUNK_SIZE {
        buf.drain(..*pos);
        *pos = 0;
    }
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
    let mut skipped_total: usize = 0;
    loop {
        compact_h3_buffer(&mut buf, &mut pos);
        let header: Option<(u64, usize)> =
            match read_h3_frame_header(recv, &mut buf, &mut pos).await {
                Ok(Some(pair)) => Some(pair),
                Ok(None) => None,
                Err(_) => {
                    return_h3_buffer(buf);
                    connection.close(
                        VarInt::from_u32(H3_FRAME_ERROR),
                        H3_REASON_MALFORMED_CONTROL,
                    );
                    return;
                }
            };
        let Some((frame_type, payload_len)) = header else {
            return_h3_buffer(buf);
            connection.close(
                VarInt::from_u32(H3_CLOSED_CRITICAL_STREAM),
                H3_REASON_CONTROL_CLOSED,
            );
            return;
        };
        if !settings_seen {
            if frame_type != H3_FRAME_SETTINGS {
                return_h3_buffer(buf);
                connection.close(
                    VarInt::from_u32(H3_MISSING_SETTINGS),
                    H3_REASON_MISSING_SETTINGS,
                );
                return;
            }
            if payload_len > H3_MAX_SETTINGS_FRAME_LEN {
                return_h3_buffer(buf);
                connection.close(
                    VarInt::from_u32(H3_FRAME_ERROR),
                    H3_REASON_SETTINGS_TOO_LARGE,
                );
                return;
            }
            match ensure_h3_bytes(recv, &mut buf, pos, payload_len).await {
                Ok(true) => {}
                Ok(false) => {
                    return_h3_buffer(buf);
                    connection.close(
                        VarInt::from_u32(H3_CLOSED_CRITICAL_STREAM),
                        H3_REASON_SETTINGS_TRUNCATED,
                    );
                    return;
                }
                Err(_) => {
                    return_h3_buffer(buf);
                    connection.close(VarInt::from_u32(H3_FRAME_ERROR), H3_REASON_CONTROL_READ);
                    return;
                }
            }
            let end: usize = pos + payload_len;
            if let Err(code) = validate_h3_settings_payload(&buf[pos..end]) {
                return_h3_buffer(buf);
                connection.close(VarInt::from_u32(code), H3_REASON_INVALID_SETTINGS);
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
                    H3_REASON_UNEXPECTED_CONTROL,
                );
                return;
            }
            other if is_reserved_h2_frame(other) => {
                return_h3_buffer(buf);
                connection.close(
                    VarInt::from_u32(H3_FRAME_UNEXPECTED),
                    H3_REASON_RESERVED_FRAME,
                );
                return;
            }
            _ => {
                if skip_h3_payload(recv, &mut buf, &mut pos, payload_len, &mut skipped_total)
                    .await
                    .is_err()
                {
                    return_h3_buffer(buf);
                    connection.close(
                        VarInt::from_u32(H3_CLOSED_CRITICAL_STREAM),
                        H3_REASON_CONTROL_TRUNCATED,
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
/// - `&QuicConnection` - The owning QUIC connection for closing on fatal stream errors.
/// - `QuicRecvStream` - The incoming unidirectional stream.
/// - `&H3ControlState` - Atomic flags recording peer critical streams immediately.
async fn handle_h3_uni_stream(
    connection: &QuicConnection,
    mut recv: QuicRecvStream,
    state: &H3ControlState,
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
                    H3_REASON_DUPLICATE_CONTROL,
                );
                return;
            }
            drive_h3_control_stream(connection, &mut recv).await;
        }
        H3_STREAM_QPACK_ENCODER => {
            if state.qpack_enc_seen.swap(true, AtomicOrdering::AcqRel) {
                connection.close(
                    VarInt::from_u32(H3_STREAM_CREATION_ERROR),
                    H3_REASON_DUPLICATE_ENCODER,
                );
                return;
            }
            let mut buf: [u8; 64] = [0; 64];
            loop {
                match recv.read(&mut buf).await {
                    Ok(Some(0)) | Ok(None) => {
                        connection.close(
                            VarInt::from_u32(H3_CLOSED_CRITICAL_STREAM),
                            H3_REASON_ENCODER_CLOSED,
                        );
                        return;
                    }
                    Ok(Some(n)) => {
                        if buf[..n].iter().any(|b: &u8| *b != QPACK_SET_CAPACITY_ZERO) {
                            connection.close(
                                VarInt::from_u32(H3_QPACK_ENCODER_STREAM_ERROR),
                                H3_REASON_QPACK_DYNAMIC_UNSUPPORTED,
                            );
                            return;
                        }
                    }
                    Err(_) => return,
                }
            }
        }
        H3_STREAM_QPACK_DECODER => {
            if state.qpack_dec_seen.swap(true, AtomicOrdering::AcqRel) {
                connection.close(
                    VarInt::from_u32(H3_STREAM_CREATION_ERROR),
                    H3_REASON_DUPLICATE_DECODER,
                );
                return;
            }
            let mut buf: [u8; 64] = [0; 64];
            let mut in_cancel_continuation: bool = false;
            let mut cancel_cont_bytes: u8 = 0;
            loop {
                match recv.read(&mut buf).await {
                    Ok(Some(0)) | Ok(None) => {
                        connection.close(
                            VarInt::from_u32(H3_CLOSED_CRITICAL_STREAM),
                            H3_REASON_DECODER_CLOSED,
                        );
                        return;
                    }
                    Ok(Some(n)) => {
                        for &byte in &buf[..n] {
                            if in_cancel_continuation {
                                cancel_cont_bytes += 1;
                                if cancel_cont_bytes > H3_MAX_QPACK_INT_CONTINUATIONS {
                                    connection.close(
                                        VarInt::from_u32(H3_QPACK_DECODER_STREAM_ERROR),
                                        H3_REASON_QPACK_DECODER_INVALID,
                                    );
                                    return;
                                }
                                if (byte & 0x80) == 0 {
                                    in_cancel_continuation = false;
                                    cancel_cont_bytes = 0;
                                }
                            } else if (byte & 0xc0) == 0x40 {
                                if (byte & 0x3f) == 0x3f {
                                    in_cancel_continuation = true;
                                    cancel_cont_bytes = 0;
                                }
                            } else {
                                connection.close(
                                    VarInt::from_u32(H3_QPACK_DECODER_STREAM_ERROR),
                                    H3_REASON_QPACK_DECODER_INVALID,
                                );
                                return;
                            }
                        }
                    }
                    Err(_) => return,
                }
            }
        }
        H3_STREAM_PUSH => {
            connection.close(
                VarInt::from_u32(H3_STREAM_CREATION_ERROR),
                H3_REASON_CLIENT_PUSH,
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
        let grow: usize = H3_READ_CHUNK_SIZE.max(needed - (buf.len() - pos));
        let chunk_res: Result<Option<Chunk>, ReadError> = recv.read_chunk(grow, true).await;
        match chunk_res {
            Ok(Some(chunk)) if !chunk.bytes.is_empty() => {
                buf.extend_from_slice(&chunk.bytes);
            }
            Ok(Some(_)) | Ok(None) => {
                return Ok(false);
            }
            Err(error) => {
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
/// - `&mut usize` - Running total of skipped bytes on this stream.
///
/// # Returns
///
/// - `Result<(), RequestError>` - `Ok(())` once discarded, or bad request if the skip budget is exceeded.
async fn skip_h3_payload(
    recv: &mut QuicRecvStream,
    buf: &mut Vec<u8>,
    pos: &mut usize,
    mut remaining: usize,
    skipped_total: &mut usize,
) -> Result<(), RequestError> {
    let next_total: usize = skipped_total.saturating_add(remaining);
    if next_total > H3_MAX_SKIP_PAYLOAD_BYTES {
        return Err(RequestError::HttpRead(HttpStatus::BadRequest));
    }
    *skipped_total = next_total;
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
/// - `&QuicConnection` - The owning QUIC connection closed on connection-level frame violations.
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
    connection: &QuicConnection,
    recv: &mut QuicRecvStream,
    buf: &mut Vec<u8>,
    pos: &mut usize,
    config: &RequestConfig,
    request: &mut Request,
    target: &mut String,
) -> Result<Option<H3DecodedRequestMeta>, RequestError> {
    let mut skipped_total: usize = 0;
    loop {
        compact_h3_buffer(buf, pos);
        let Some((frame_type, payload_len)) = read_h3_frame_header(recv, buf, pos).await? else {
            return Ok(None);
        };
        if frame_type != H3_FRAME_HEADERS {
            let forbidden: bool =
                frame_type == H3_FRAME_DATA || H3_FORBIDDEN_REQUEST_FRAMES.contains(&frame_type);
            if forbidden || is_reserved_h2_frame(frame_type) {
                let reason: &[u8] = if is_reserved_h2_frame(frame_type) {
                    H3_REASON_RESERVED_FRAME
                } else {
                    H3_REASON_UNEXPECTED_REQUEST_FRAME
                };
                connection.close(VarInt::from_u32(H3_FRAME_UNEXPECTED), reason);
                return Err(RequestError::HttpRead(HttpStatus::BadRequest));
            }
            skip_h3_payload(recv, buf, pos, payload_len, &mut skipped_total).await?;
            continue;
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

/// Collects HTTP/3 `DATA` frames into `body` while enforcing size limits.
///
/// # Arguments
///
/// - `&QuicConnection` - The owning QUIC connection closed on connection-level frame violations.
/// - `&mut QuicRecvStream` - The receive stream carrying `DATA` frames.
/// - `&mut Vec<u8>` - The pooled read buffer containing any bytes already read after `HEADERS`.
/// - `&mut usize` - Read cursor inside `buf`.
/// - `usize` - Maximum permitted body length.
/// - `Option<usize>` - Declared `content-length`, if present.
/// - `&mut Vec<u8>` - Destination request body buffer.
///
/// # Returns
///
/// - `Result<(), RequestError>` - `Ok(())` when the request stream ends cleanly.
async fn collect_h3_data_frames(
    connection: &QuicConnection,
    recv: &mut QuicRecvStream,
    buf: &mut Vec<u8>,
    pos: &mut usize,
    max_body_size: usize,
    declared: Option<usize>,
    body: &mut Vec<u8>,
) -> Result<(), RequestError> {
    let mut trailers_seen: bool = false;
    let mut skipped_total: usize = 0;
    loop {
        compact_h3_buffer(buf, pos);
        let Some((frame_type, payload_len)) = read_h3_frame_header(recv, buf, pos).await? else {
            return Ok(());
        };
        match frame_type {
            H3_FRAME_DATA => {
                if trailers_seen {
                    connection.close(
                        VarInt::from_u32(H3_FRAME_UNEXPECTED),
                        H3_REASON_FRAME_AFTER_TRAILERS,
                    );
                    return Err(RequestError::HttpRead(HttpStatus::BadRequest));
                }
                if let Some(expected) = declared
                    && payload_len > expected.saturating_sub(body.len())
                {
                    let _: Result<(), _> = recv.stop(VarInt::from_u32(H3_NO_ERROR));
                    return Err(RequestError::Request(CONTENT_LENGTH_MISMATCH.to_owned()));
                }
                if max_body_size != DEFAULT_LOW_SECURITY_MAX_BODY_SIZE
                    && payload_len > max_body_size.saturating_sub(body.len())
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
                    let chunk_len: usize = remaining.min(MAX_POOLED_BUFFER_CAPACITY);
                    let chunk_res: Result<Option<Chunk>, ReadError> =
                        recv.read_chunk(chunk_len, true).await;
                    match chunk_res {
                        Ok(Some(chunk)) if !chunk.bytes.is_empty() => {
                            body.extend_from_slice(&chunk.bytes);
                            remaining -= chunk.bytes.len();
                        }
                        Ok(Some(_)) | Ok(None) => {
                            return Err(RequestError::HttpRead(HttpStatus::BadRequest));
                        }
                        Err(error) => {
                            return Err(RequestError::Request(error.to_string()));
                        }
                    }
                }
            }
            H3_FRAME_HEADERS => {
                if trailers_seen {
                    connection.close(
                        VarInt::from_u32(H3_FRAME_UNEXPECTED),
                        H3_REASON_FRAME_AFTER_TRAILERS,
                    );
                    return Err(RequestError::HttpRead(HttpStatus::BadRequest));
                }
                trailers_seen = true;
                skip_h3_payload(recv, buf, pos, payload_len, &mut skipped_total).await?;
            }
            other if is_grease_id(other) => {
                skip_h3_payload(recv, buf, pos, payload_len, &mut skipped_total).await?;
            }
            H3_FRAME_SETTINGS
            | H3_FRAME_GOAWAY
            | H3_FRAME_CANCEL_PUSH
            | H3_FRAME_PUSH_PROMISE
            | H3_FRAME_MAX_PUSH_ID => {
                connection.close(
                    VarInt::from_u32(H3_FRAME_UNEXPECTED),
                    H3_REASON_UNEXPECTED_REQUEST_FRAME,
                );
                return Err(RequestError::HttpRead(HttpStatus::BadRequest));
            }
            other if is_reserved_h2_frame(other) => {
                connection.close(
                    VarInt::from_u32(H3_FRAME_UNEXPECTED),
                    H3_REASON_RESERVED_FRAME,
                );
                return Err(RequestError::HttpRead(HttpStatus::BadRequest));
            }
            _ => {
                skip_h3_payload(recv, buf, pos, payload_len, &mut skipped_total).await?;
            }
        }
    }
}

/// Attempts to decode an HTTP/3 request head directly from the first QUIC chunk without a scratch buffer.
///
/// # Arguments
///
/// - `&QuicConnection` - The owning QUIC connection closed on connection-level frame violations.
/// - `&mut QuicRecvStream` - The QUIC receive stream.
/// - `&mut Option<Vec<u8>>` - Lazily-allocated pooled buffer populated only when trailing bytes or multi-chunk reads occur.
/// - `&mut usize` - Read cursor inside the optional buffer.
/// - `&RequestConfig` - Security limits.
/// - `&mut Request` - The destination framework request.
/// - `&mut String` - The destination target buffer.
///
/// # Returns
///
/// - `Result<Option<H3DecodedRequestMeta>, RequestError>` - Decoded head metadata, `None` on clean idle EOF, or error.
async fn read_h3_request_headers_fast(
    connection: &QuicConnection,
    recv: &mut QuicRecvStream,
    read_buf: &mut Option<Vec<u8>>,
    read_pos: &mut usize,
    config: &RequestConfig,
    request: &mut Request,
    target: &mut String,
) -> Result<Option<H3DecodedRequestMeta>, RequestError> {
    let chunk_res: Result<Option<Chunk>, ReadError> =
        recv.read_chunk(H3_READ_CHUNK_SIZE, true).await;
    let first_chunk: Chunk = match chunk_res {
        Ok(Some(chunk)) if !chunk.bytes.is_empty() => chunk,
        Ok(Some(_)) | Ok(None) => return Ok(None),
        Err(error) => return Err(RequestError::Request(error.to_string())),
    };
    let bytes: &[u8] = &first_chunk.bytes;
    if let Some((frame_type, type_len)) = decode_varint(bytes)
        && frame_type == H3_FRAME_HEADERS
        && let Some((payload_len_u64, len_bytes)) = decode_varint(&bytes[type_len..])
        && let Ok(payload_len) = usize::try_from(payload_len_u64)
    {
        if payload_len > H3_MAX_HEADERS_FRAME_LEN {
            return Err(RequestError::HeaderValueTooLong(
                HttpStatus::RequestHeaderFieldsTooLarge,
            ));
        }
        let header_start: usize = type_len + len_bytes;
        if let Some(frame_end) = header_start.checked_add(payload_len)
            && frame_end <= bytes.len()
        {
            let meta: H3DecodedRequestMeta =
                decode_qpack_request(&bytes[header_start..frame_end], config, request, target)?;
            if frame_end < bytes.len() {
                let buf: &mut Vec<u8> = read_buf.get_or_insert_with(take_h3_buffer);
                buf.extend_from_slice(&bytes[frame_end..]);
                *read_pos = 0;
            }
            return Ok(Some(meta));
        }
    }
    let buf: &mut Vec<u8> = read_buf.get_or_insert_with(take_h3_buffer);
    buf.extend_from_slice(bytes);
    *read_pos = 0;
    read_h3_request_headers(connection, recv, buf, read_pos, config, request, target).await
}

/// Runs the framework lifecycle directly over one QUIC bidirectional stream speaking HTTP/3.
///
/// # Arguments
///
/// - `&ConnectionShared` - Shared server state, compiled pipeline and request limits.
/// - `StreamPeers` - Local and remote addresses of the QUIC connection.
/// - `&QuicConnection` - The owning QUIC connection closed on connection-level frame violations.
/// - `QuicSendStream` - The QUIC send stream for the response.
/// - `QuicRecvStream` - The QUIC receive stream for the request.
pub(crate) async fn h3_request_task(
    shared: &ConnectionShared,
    peers: StreamPeers,
    connection: &QuicConnection,
    mut send_stream: QuicSendStream,
    mut recv_stream: QuicRecvStream,
) {
    let server: &Server = &shared.server;
    let request_config: RequestConfig = shared.request_config;
    let mut read_buf: Option<Vec<u8>> = None;
    let mut read_pos: usize = 0;
    let mut ctx: Context = take_pooled_context();
    let mut target: String = take_target_buffer();
    let read_timeout_ms: u64 = request_config.get_read_timeout_ms();
    let head_result: Result<Option<H3DecodedRequestMeta>, RequestError> = if read_timeout_ms
        == DEFAULT_LOW_SECURITY_READ_TIMEOUT_MS
    {
        read_h3_request_headers_fast(
            connection,
            &mut recv_stream,
            &mut read_buf,
            &mut read_pos,
            &request_config,
            ctx.get_mut_request(),
            &mut target,
        )
        .await
    } else {
        let mut reading: Pin<&mut _> = pin!(read_h3_request_headers_fast(
            connection,
            &mut recv_stream,
            &mut read_buf,
            &mut read_pos,
            &request_config,
            ctx.get_mut_request(),
            &mut target,
        ));
        match poll_with_timeout_protocol(reading.as_mut(), Duration::from_millis(read_timeout_ms))
            .await
        {
            Some(done) => done,
            None => Err(RequestError::ReadTimeout(HttpStatus::RequestTimeout)),
        }
    };
    let meta: H3DecodedRequestMeta = match head_result {
        Ok(Some(meta)) => meta,
        Ok(None) => {
            if let Some(buf) = read_buf {
                return_h3_buffer(buf);
            }
            return_target_buffer(target);
            return_pooled_context(ctx);
            return;
        }
        Err(error) => {
            let _: Result<(), _> = recv_stream.stop(VarInt::from_u32(H3_NO_ERROR));
            if let Some(buf) = read_buf {
                return_h3_buffer(buf);
            }
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
    let body_result: Result<(), RequestError> = {
        let mut reading: Pin<&mut _> = pin!(async {
            if meta.expect_continue {
                send_stream
                    .write_all(H3_CONTINUE_FRAME)
                    .await
                    .map_err(|error: QuicWriteError| RequestError::Request(error.to_string()))?;
            }
            if read_buf.is_none()
                && !meta.expect_continue
                && matches!(meta.declared_content_length, None | Some(0))
            {
                match recv_stream.read_chunk(H3_READ_CHUNK_SIZE, true).await {
                    Ok(None) => return Ok(()),
                    Ok(Some(chunk)) => {
                        let buf: &mut Vec<u8> = read_buf.get_or_insert_with(take_h3_buffer);
                        buf.extend_from_slice(&chunk.bytes);
                        read_pos = 0;
                    }
                    Err(error) => return Err(RequestError::Request(error.to_string())),
                }
            }
            let buf: &mut Vec<u8> = read_buf.get_or_insert_with(take_h3_buffer);
            collect_h3_data_frames(
                connection,
                &mut recv_stream,
                buf,
                &mut read_pos,
                max_body_size,
                meta.declared_content_length,
                body,
            )
            .await
        });
        if read_timeout_ms == DEFAULT_LOW_SECURITY_READ_TIMEOUT_MS {
            reading.await
        } else {
            match poll_with_timeout_protocol(
                reading.as_mut(),
                Duration::from_millis(read_timeout_ms),
            )
            .await
            {
                Some(done) => done,
                None => Err(RequestError::ReadTimeout(HttpStatus::RequestTimeout)),
            }
        }
    };
    if let Some(buf) = read_buf {
        return_h3_buffer(buf);
    }
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
    let mut stream_guard: H3StreamGuard =
        H3StreamGuard::take(send_stream, peers, request_config, meta.is_head);
    if let Some(stream) = stream_guard.stream_mut() {
        server
            .run_protocol_lifecycle_in_place(&shared.pipeline, stream, &mut ctx, target)
            .await;
    }
    stream_guard.finish().await;
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
    let mut stream_guard: H3StreamGuard =
        H3StreamGuard::take(send_stream, peers, request_config, false);
    if let Some(stream) = stream_guard.stream_mut() {
        reply_to_request_error(server, stream, ctx, &error, Some(version)).await;
        let _: Result<(), io::Error> = stream.get_mut_stream().shutdown().await;
    }
}
