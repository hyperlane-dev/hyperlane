use super::*;

/// Decodes chunked request content on the same reader that owns pipelined bytes.
///
/// # Arguments
///
/// - `&mut PooledReader<'_>` - The reader positioned after the initial fields.
/// - `&mut Request` - The request to populate with decoded content.
/// - `&RequestConfig` - The request size limits.
/// # Returns
///
/// - `Result<(), RequestError>` - Whether the complete chunked request was read.
#[cold]
pub(super) async fn fill_chunked_request(
    reader: &mut PooledReader<'_>,
    request: &mut Request,
    config: &RequestConfig,
) -> Result<(), RequestError> {
    if expects_http_continue(request) {
        let transport: &mut Box<dyn Transport> = reader.get_mut_stream();
        transport.write_all(HTTP_CONTINUE_RESPONSE).await?;
        transport.flush().await?;
    }
    fill_chunked_body(reader, request.get_mut_body(), config).await?;
    request.normalize_chunked_headers();
    Ok(())
}

/// Checks for the continue expectation on a request that has unread content.
///
/// # Arguments
///
/// - `&Request` - The request with a validated header section.
///
/// # Returns
///
/// - `bool` - Whether to send an HTTP/1.1 interim response.
#[cold]
pub(super) fn expects_http_continue(request: &Request) -> bool {
    request.get_version().is_http1_1()
        && request
            .get_headers()
            .get(EXPECT)
            .is_some_and(|values: &RequestHeadersValue| {
                values
                    .iter()
                    .any(|value: &String| contains_continue_member(value))
            })
}

/// Finds the continue token without treating commas inside quoted values as separators.
///
/// # Arguments
///
/// - `&str` - One Expect field value.
/// # Returns
///
/// - `bool` - Whether the field contains the bare continue expectation.
fn contains_continue_member(value: &str) -> bool {
    let mut start: usize = 0;
    let mut quoted: bool = false;
    let mut escaped: bool = false;
    for (offset, byte) in value.bytes().enumerate() {
        match byte {
            _ if escaped => escaped = false,
            HTTP_QUOTED_FIELD_ESCAPE if quoted => escaped = true,
            HTTP_QUOTED_FIELD_QUOTE => quoted = !quoted,
            COMMA_U8 if !quoted => {
                if value[start..offset]
                    .trim_matches([' ', '\t'])
                    .eq_ignore_ascii_case(CONTINUE_EXPECT)
                {
                    return true;
                }
                start = offset + 1;
            }
            _ => {}
        }
    }
    value[start..]
        .trim_matches([' ', '\t'])
        .eq_ignore_ascii_case(CONTINUE_EXPECT)
}

/// Takes a recycled header entry from the pool.
///
/// # Returns
///
/// - `Option<(String, VecDeque<String>)>` - The recycled entry, or `None` when
///   the pool is empty.
pub(crate) fn take_header_entry() -> Option<(String, VecDeque<String>)> {
    HEADER_ENTRY_POOL
        .with(|pool: &RefCell<Vec<(String, VecDeque<String>)>>| pool.borrow_mut().pop())
}

/// Returns a header entry to the pool for reuse.
///
/// # Arguments
///
/// - `(String, VecDeque<String>)` - The entry to recycle.
pub(crate) fn return_header_entry(entry: (String, VecDeque<String>)) {
    if entry.0.capacity() > MAX_POOLED_HEADER_STRING_CAPACITY
        || entry.1.capacity() > MAX_POOLED_HEADER_VALUE_CAPACITY
    {
        return;
    }
    if entry
        .1
        .iter()
        .any(|value: &String| value.capacity() > MAX_POOLED_HEADER_STRING_CAPACITY)
    {
        return;
    }
    HEADER_ENTRY_POOL.with(|pool: &RefCell<Vec<(String, VecDeque<String>)>>| {
        let mut pool: RefMut<'_, Vec<(String, VecDeque<String>)>> = pool.borrow_mut();
        if pool.len() < MAX_POOLED_HEADER_ENTRIES {
            pool.push(entry);
        }
    });
}

/// Returns multiple header or query entries to the worker-local pool in a single lookup.
///
/// # Arguments
///
/// - `I` - An iterator yielding header or query entries to recycle.
pub(crate) fn return_header_entries<I>(entries: I)
where
    I: IntoIterator<Item = (String, VecDeque<String>)>,
{
    HEADER_ENTRY_POOL.with(|pool: &RefCell<Vec<(String, VecDeque<String>)>>| {
        let mut pool: RefMut<'_, Vec<(String, VecDeque<String>)>> = pool.borrow_mut();
        for entry in entries {
            if pool.len() >= MAX_POOLED_HEADER_ENTRIES {
                break;
            }
            if entry.0.capacity() > MAX_POOLED_HEADER_STRING_CAPACITY
                || entry.1.capacity() > MAX_POOLED_HEADER_VALUE_CAPACITY
            {
                continue;
            }
            if entry
                .1
                .iter()
                .any(|value: &String| value.capacity() > MAX_POOLED_HEADER_STRING_CAPACITY)
            {
                continue;
            }
            pool.push(entry);
        }
    });
}

/// Polls a future once before lazily arming or reusing a connection-scoped sleep timer.
///
/// The reusable timer stays armed in Tokio's timer wheel across consecutive
/// keep-alive requests so high-concurrency loops avoid per-request timer wheel
/// insertion and removal while still firing at the exact request deadline.
///
/// # Arguments
///
/// - `Pin<&mut F>` - The future to drive.
/// - `&mut Option<Pin<Box<Sleep>>>` - Reusable sleep slot owned by the connection.
/// - `Duration` - Maximum time to wait once the first poll returns `Poll::Pending`.
///
/// # Returns
///
/// - `Option<F::Output>` - `Some` with the completed value, or `None` on timeout.
pub(crate) async fn poll_with_timeout<F>(
    mut future: Pin<&mut F>,
    timer_slot: &mut Option<Pin<Box<Sleep>>>,
    duration: Duration,
) -> Option<F::Output>
where
    F: Future,
{
    let mut exact_deadline: Option<Instant> = None;
    future::poll_fn(|context: &mut Context<'_>| {
        if let Poll::Ready(value) = future.as_mut().poll(context) {
            return Poll::Ready(Some(value));
        }
        let deadline: Instant = *exact_deadline.get_or_insert_with(|| Instant::now() + duration);
        let timer: &mut Pin<Box<Sleep>> = match timer_slot {
            Some(existing) => {
                if existing.deadline() > deadline {
                    existing.as_mut().reset(deadline);
                }
                existing
            }
            None => timer_slot.insert(Box::pin(sleep_until(deadline))),
        };
        loop {
            match timer.as_mut().poll(context) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(()) => {
                    if Instant::now() < deadline {
                        timer.as_mut().reset(deadline);
                        continue;
                    }
                    return Poll::Ready(None);
                }
            }
        }
    })
    .await
}

/// Takes a read buffer from the thread-local pool or allocates a new one.
///
/// # Arguments
///
/// - `usize` - The required buffer length in bytes.
///
/// # Returns
///
/// - `Vec<u8>` - A buffer with at least the requested length.
pub(crate) fn take_read_buffer(capacity: usize) -> Vec<u8> {
    READ_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| {
        let mut pool: RefMut<'_, Vec<Vec<u8>>> = pool.borrow_mut();
        match pool.pop() {
            Some(mut buffer) => {
                if buffer.len() < capacity {
                    buffer.resize(capacity, 0);
                }
                buffer
            }
            None => vec![0; capacity],
        }
    })
}

/// Returns a read buffer to the thread-local pool for reuse.
///
/// Buffers larger than `MAX_POOLED_READ_BUFFER_SIZE` are dropped instead
/// of being retained, and the pool is capped at `MAX_POOLED_READ_BUFFERS`.
///
/// # Arguments
///
/// - `Vec<u8>` - The buffer to recycle.
pub(crate) fn return_read_buffer(buffer: Vec<u8>) {
    if buffer.is_empty() || buffer.capacity() > MAX_POOLED_READ_BUFFER_SIZE {
        return;
    }
    READ_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| {
        let mut pool: RefMut<'_, Vec<Vec<u8>>> = pool.borrow_mut();
        if pool.len() < MAX_POOLED_READ_BUFFERS {
            pool.push(buffer);
        }
    });
}
