use super::*;

/// Computes a deterministic 64-bit fingerprint for custom TLS trust stores (`0` for default roots).
///
/// # Arguments
///
/// - `&RootCertStore` - Configured TLS trust anchor store.
///
/// # Returns
///
/// - `u64` - `0` when using the default Mozilla root set, or a non-zero FNV-1a digest of custom anchors.
fn root_cert_fingerprint(roots: &RootCertStore) -> u64 {
    if roots.roots.len() == TLS_SERVER_ROOTS.len() {
        return 0;
    }
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    let extra_start: usize = roots.roots.len().min(TLS_SERVER_ROOTS.len());
    for anchor in &roots.roots[extra_start..] {
        for &byte in anchor.subject.as_ref() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        for &byte in anchor.subject_public_key_info.as_ref() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    hash ^ (roots.roots.len() as u64)
}

/// Builds a normalized [`PoolKey`] for connection pooling.
///
/// # Arguments
///
/// - `bool` - Whether the request targets HTTPS.
/// - `&str` - Target hostname or IP literal.
/// - `u16` - Effective target TCP port.
/// - `Option<&Proxy>` - Optional proxy configuration.
/// - `&RootCertStore` - Configured root CA certificate store.
/// - `u64` - Synchronous socket timeout in milliseconds (`0` for async pools).
///
/// # Returns
///
/// - `PoolKey` - Canonical lookup key for the connection pool.
pub(crate) fn make_pool_key(
    is_https: bool,
    host: &str,
    port: u16,
    proxy: Option<&Proxy>,
    roots: &RootCertStore,
    sync_timeout_ms: u64,
) -> PoolKey {
    let proxy_key: Option<ProxyKey> = proxy.map(|p: &Proxy| ProxyKey {
        proxy_type_tag: match *p.get_proxy_type() {
            ProxyType::Http => 0,
            ProxyType::Https => 1,
            ProxyType::Socks5 => 2,
        },
        host: p.get_host().to_ascii_lowercase(),
        port: p.get_port(),
        username: p.get_username(),
        password: p.get_password(),
    });
    PoolKey {
        is_https,
        host: host.to_ascii_lowercase(),
        port,
        proxy: proxy_key,
        root_cert_fingerprint: root_cert_fingerprint(roots),
        sync_timeout_ms,
    }
}

/// Takes one non-expired asynchronous HTTP/1.1 connection from the pool if available.
///
/// # Arguments
///
/// - `&PoolKey` - Origin and proxy pool key.
///
/// # Returns
///
/// - `Option<BoxAsyncReadWrite>` - A reusable keep-alive connection, or `None`.
pub(crate) fn take_async_h1_conn(key: &PoolKey) -> Option<BoxAsyncReadWrite> {
    let map_lock: &Mutex<AsyncH1PoolMap> = ASYNC_H1_POOL.get_or_init(|| Mutex::new(HashMap::new()));
    let Ok(mut map) = map_lock.lock() else {
        return None;
    };
    let queue: &mut VecDeque<PooledAsyncConn> = map.get_mut(key)?;
    let ttl: Duration = Duration::from_secs(POOL_IDLE_TTL_SECS);
    while let Some(conn) = queue.pop_back() {
        if conn.idle_since.elapsed() < ttl {
            if queue.is_empty() {
                map.remove(key);
            }
            return Some(conn.stream);
        }
    }
    map.remove(key);
    None
}

/// Returns a clean asynchronous HTTP/1.1 connection to the idle pool within capacity caps.
///
/// # Arguments
///
/// - `PoolKey` - Origin and proxy pool key.
/// - `BoxAsyncReadWrite` - Idle keep-alive stream ready for the next request.
pub(crate) fn put_async_h1_conn(key: PoolKey, stream: BoxAsyncReadWrite) {
    let map_lock: &Mutex<AsyncH1PoolMap> = ASYNC_H1_POOL.get_or_init(|| Mutex::new(HashMap::new()));
    let Ok(mut map) = map_lock.lock() else {
        return;
    };
    let ttl: Duration = Duration::from_secs(POOL_IDLE_TTL_SECS);
    let mut total: usize = 0;
    map.retain(|_: &PoolKey, q: &mut VecDeque<PooledAsyncConn>| {
        q.retain(|item: &PooledAsyncConn| item.idle_since.elapsed() < ttl);
        total += q.len();
        !q.is_empty()
    });
    if total >= POOL_MAX_IDLE_TOTAL {
        return;
    }
    let queue: &mut VecDeque<PooledAsyncConn> = map.entry(key).or_default();
    if queue.len() < POOL_MAX_IDLE_PER_HOST {
        queue.push_back(PooledAsyncConn {
            stream,
            idle_since: Instant::now(),
        });
    }
}

/// Takes one non-expired synchronous HTTP/1.1 connection from the pool if available.
///
/// # Arguments
///
/// - `&PoolKey` - Origin and proxy pool key.
///
/// # Returns
///
/// - `Option<BoxReadWrite>` - A reusable keep-alive connection, or `None`.
pub(crate) fn take_sync_h1_conn(key: &PoolKey) -> Option<BoxReadWrite> {
    let map_lock: &Mutex<SyncH1PoolMap> = SYNC_H1_POOL.get_or_init(|| Mutex::new(HashMap::new()));
    let Ok(mut map) = map_lock.lock() else {
        return None;
    };
    let queue: &mut VecDeque<PooledSyncConn> = map.get_mut(key)?;
    let ttl: Duration = Duration::from_secs(POOL_IDLE_TTL_SECS);
    while let Some(conn) = queue.pop_back() {
        if conn.idle_since.elapsed() < ttl {
            if queue.is_empty() {
                map.remove(key);
            }
            return Some(conn.stream);
        }
    }
    map.remove(key);
    None
}

/// Returns a clean synchronous HTTP/1.1 connection to the idle pool within capacity caps.
///
/// # Arguments
///
/// - `PoolKey` - Origin and proxy pool key.
/// - `BoxReadWrite` - Idle keep-alive stream ready for the next request.
pub(crate) fn put_sync_h1_conn(key: PoolKey, stream: BoxReadWrite) {
    let map_lock: &Mutex<SyncH1PoolMap> = SYNC_H1_POOL.get_or_init(|| Mutex::new(HashMap::new()));
    let Ok(mut map) = map_lock.lock() else {
        return;
    };
    let ttl: Duration = Duration::from_secs(POOL_IDLE_TTL_SECS);
    let mut total: usize = 0;
    map.retain(|_: &PoolKey, q: &mut VecDeque<PooledSyncConn>| {
        q.retain(|item: &PooledSyncConn| item.idle_since.elapsed() < ttl);
        total += q.len();
        !q.is_empty()
    });
    if total >= POOL_MAX_IDLE_TOTAL {
        return;
    }
    let queue: &mut VecDeque<PooledSyncConn> = map.entry(key).or_default();
    if queue.len() < POOL_MAX_IDLE_PER_HOST {
        queue.push_back(PooledSyncConn {
            stream,
            idle_since: Instant::now(),
        });
    }
}

/// Retrieves a cloned multiplexed HTTP/2 sender for `key` when an active connection exists.
///
/// # Arguments
///
/// - `&PoolKey` - Origin and proxy pool key.
///
/// # Returns
///
/// - `Option<H2SendRequest>` - Cloned HTTP/2 request sender, or `None`.
pub(crate) fn get_h2_sender(key: &PoolKey) -> Option<H2SendRequest> {
    let map_lock: &Mutex<AsyncH2PoolMap> = ASYNC_H2_POOL.get_or_init(|| Mutex::new(HashMap::new()));
    let Ok(map) = map_lock.lock() else {
        return None;
    };
    map.get(key).cloned()
}

/// Stores an active multiplexed HTTP/2 sender in the pool.
///
/// # Arguments
///
/// - `PoolKey` - Origin and proxy pool key.
/// - `H2SendRequest` - Active HTTP/2 sender handle.
pub(crate) fn put_h2_sender(key: PoolKey, sender: H2SendRequest) {
    let map_lock: &Mutex<AsyncH2PoolMap> = ASYNC_H2_POOL.get_or_init(|| Mutex::new(HashMap::new()));
    let Ok(mut map) = map_lock.lock() else {
        return;
    };
    if map.len() >= POOL_MAX_IDLE_TOTAL {
        map.clear();
    }
    map.insert(key, sender);
}

/// Removes a closed or broken HTTP/2 sender from the pool.
///
/// # Arguments
///
/// - `&PoolKey` - Origin and proxy pool key to evict.
pub(crate) fn remove_h2_sender(key: &PoolKey) {
    let map_lock: &Mutex<AsyncH2PoolMap> = ASYNC_H2_POOL.get_or_init(|| Mutex::new(HashMap::new()));
    if let Ok(mut map) = map_lock.lock() {
        map.remove(key);
    }
}

/// Takes a recycled empty byte buffer with at least `min_cap` capacity from the worker-local pool.
///
/// # Arguments
///
/// - `usize` - Minimum required capacity in bytes.
///
/// # Returns
///
/// - `Vec<u8>` - Empty buffer retaining pooled capacity when available.
#[inline(always)]
pub(crate) fn take_client_buffer(min_cap: usize) -> Vec<u8> {
    CLIENT_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| {
        let mut buf: Vec<u8> = pool
            .borrow_mut()
            .pop()
            .unwrap_or_else(|| Vec::with_capacity(min_cap));
        buf.clear();
        if buf.capacity() < min_cap {
            buf.reserve(min_cap - buf.capacity());
        }
        buf
    })
}

/// Returns a reusable byte buffer to the worker-local pool when within size caps.
///
/// # Arguments
///
/// - `Vec<u8>` - Buffer to recycle.
#[inline(always)]
pub(crate) fn return_client_buffer(mut buf: Vec<u8>) {
    if buf.capacity() == 0 || buf.capacity() > CLIENT_MAX_POOLED_BUFFER_CAP {
        return;
    }
    buf.clear();
    CLIENT_BUFFER_POOL.with(|pool: &RefCell<Vec<Vec<u8>>>| {
        let mut guard: RefMut<'_, Vec<Vec<u8>>> = pool.borrow_mut();
        if guard.len() < CLIENT_MAX_POOLED_BUFFERS {
            guard.push(buf);
        }
    });
}
