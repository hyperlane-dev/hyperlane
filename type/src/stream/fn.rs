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
            b'\\' if quoted => escaped = true,
            b'"' => quoted = !quoted,
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

/// The number of header entries the pool keeps for reuse.
pub(crate) const MAX_POOLED_HEADER_ENTRIES: usize = B_128;

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
    HEADER_ENTRY_POOL.with(|pool: &RefCell<Vec<(String, VecDeque<String>)>>| {
        let mut pool: RefMut<'_, Vec<(String, VecDeque<String>)>> = pool.borrow_mut();
        if pool.len() < MAX_POOLED_HEADER_ENTRIES {
            pool.push(entry);
        }
    });
}

/// Polls a future exactly once.
///
/// # Arguments
///
/// - `Pin<&mut F>` - The future to poll.
///
/// # Returns
///
/// - `Option<F::Output>` - `Some` with the value when the future completed on
///   that first poll, `None` when it is still pending. A pending future stays
///   registered with its waker, so it can be awaited normally afterwards.
pub(crate) async fn poll_once<F>(mut future: Pin<&mut F>) -> Option<F::Output>
where
    F: Future,
{
    future::poll_fn(
        |context: &mut Context<'_>| match future.as_mut().poll(context) {
            Poll::Ready(value) => Poll::Ready(Some(value)),
            Poll::Pending => Poll::Ready(None),
        },
    )
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
