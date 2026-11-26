use super::*;

/// Computes the target QUIC shard index from a QUIC packet's Destination Connection ID.
///
/// Both RFC 9000 long-header packets (`dcid` starting at byte 6) and short-header 1-RTT
/// packets (`dcid` starting at byte 1) route by `dcid[0] % shard_count`, so client IP/port
/// migration always reaches the exact shard owning the connection.
///
/// # Arguments
///
/// - `&[u8]` - Raw UDP datagram bytes containing one QUIC packet (or coalesced chain).
/// - `usize` - Total number of active QUIC shards (`>= 1`).
///
/// # Returns
///
/// - `usize` - Zero-based shard index in `0..shard_count`.
#[inline(always)]
pub(crate) fn route_quic_datagram(packet: &[u8], shard_count: usize) -> usize {
    if shard_count <= 1 || packet.is_empty() {
        return 0;
    }
    let first_cid_byte: u8 = if (packet[0] & 0x80) != 0 {
        let dcid_len: usize = packet.get(5).copied().unwrap_or(0) as usize;
        if dcid_len > 0 {
            packet.get(6).copied().unwrap_or(0)
        } else {
            0
        }
    } else {
        packet.get(1).copied().unwrap_or(0)
    };
    usize::from(first_cid_byte) % shard_count
}

/// Polls one batch of incoming datagrams from the exclusively bound OS UDP socket.
///
/// # Arguments
///
/// - `&Arc<dyn AsyncUdpSocket>` - The real OS UDP socket.
/// - `&mut [Vec<u8>]` - Preallocated receive storage buffers.
/// - `&mut [RecvMeta]` - Output metadata slice.
///
/// # Returns
///
/// - `io::Result<usize>` - Number of received buffer entries.
async fn recv_udp_batch(
    real_socket: &Arc<dyn AsyncUdpSocket>,
    buffers: &mut [Vec<u8>],
    metas: &mut [RecvMeta],
) -> io::Result<usize> {
    poll_fn(|cx: &mut task::Context<'_>| {
        let mut slices: [MaybeUninit<IoSliceMut<'_>>; BATCH_SIZE] =
            [const { MaybeUninit::uninit() }; BATCH_SIZE];
        let count: usize = buffers.len().min(BATCH_SIZE).min(metas.len());
        for (idx, buf) in buffers[..count].iter_mut().enumerate() {
            slices[idx].write(IoSliceMut::new(buf.as_mut_slice()));
        }
        // SAFETY: Exactly `0..count` elements of `slices` were initialized above, and
        // `IoSliceMut` has no `Drop` implementation.
        let slice_refs: &mut [IoSliceMut<'_>] = unsafe {
            slice::from_raw_parts_mut(slices.as_mut_ptr().cast::<IoSliceMut<'_>>(), count)
        };
        real_socket.poll_recv(cx, slice_refs, &mut metas[..count])
    })
    .await
}

/// Runs the dedicated single-socket UDP receive demultiplexer feeding all CID-steered QUIC shards.
///
/// # Arguments
///
/// - `Arc<dyn AsyncUdpSocket>` - The exclusively bound OS UDP socket registered on this thread's reactor.
/// - `Vec<Arc<Mutex<ShardQueueState>>>` - Per-shard inbound datagram queues and buffer pools.
/// - `Receiver<()>` - Shutdown signal stopping the dispatcher once all endpoints drain.
async fn run_quic_shard_dispatcher(
    real_socket: Arc<dyn AsyncUdpSocket>,
    queues: Vec<Arc<Mutex<ShardQueueState>>>,
    mut shutdown_rx: Receiver<()>,
) {
    let seg_count: usize = real_socket.max_receive_segments().max(1);
    let buf_size: usize = (seg_count * 1500).max(64 * 1024);
    let mut buffers: Vec<Vec<u8>> = (0..BATCH_SIZE).map(|_: usize| vec![0; buf_size]).collect();
    let mut metas: Vec<RecvMeta> = vec![RecvMeta::default(); BATCH_SIZE];
    let shard_count: usize = queues.len();
    let mut wakers_to_wake: [Option<Waker>; 64] = [const { None }; 64];
    let mut local_pool: Vec<Vec<u8>> = Vec::with_capacity(QUIC_SHARD_POOL_CAP);
    let mut staged: Vec<Vec<ShardDatagram>> = (0..shard_count)
        .map(|_: usize| Vec::with_capacity(BATCH_SIZE))
        .collect();
    loop {
        let batch_res: Option<io::Result<usize>> = tokio::select! {
            biased;
            _ = shutdown_rx.changed() => None,
            received = recv_udp_batch(&real_socket, &mut buffers, &mut metas) => Some(received),
        };
        let Some(received) = batch_res else {
            break;
        };
        let Ok(count) = received else {
            continue;
        };
        let mut touched_mask: u64 = 0;
        for idx in 0..count {
            let meta: RecvMeta = metas[idx];
            let total_len: usize = meta.len.min(buffers[idx].len());
            if total_len == 0 {
                continue;
            }
            let stride: usize = meta.stride.max(1);
            let mut offset: usize = 0;
            while offset < total_len {
                let end: usize = (offset + stride).min(total_len);
                let slice: &[u8] = &buffers[idx][offset..end];
                let shard_idx: usize = route_quic_datagram(slice, shard_count);
                let mut seg_meta: RecvMeta = meta;
                seg_meta.len = slice.len();
                seg_meta.stride = slice.len();
                let mut payload: Vec<u8> = local_pool
                    .pop()
                    .unwrap_or_else(|| Vec::with_capacity(slice.len().max(QUIC_DATAGRAM_INIT_CAP)));
                payload.clear();
                payload.extend_from_slice(slice);
                staged[shard_idx].push(ShardDatagram {
                    data: payload,
                    meta: seg_meta,
                });
                if shard_idx < 64 {
                    touched_mask |= 1u64 << shard_idx;
                }
                offset = end;
            }
        }
        let mut wake_mask: u64 = 0;
        let mut iter_mask: u64 = touched_mask;
        let max_local_pool: usize = QUIC_SHARD_POOL_CAP.saturating_mul(2);
        while iter_mask != 0 {
            let shard_idx: usize = iter_mask.trailing_zeros() as usize;
            iter_mask &= iter_mask - 1;
            let batch: &mut Vec<ShardDatagram> = &mut staged[shard_idx];
            if batch.is_empty() {
                continue;
            }
            if let Ok(mut q) = queues[shard_idx].lock() {
                let room: usize = QUIC_SHARD_QUEUE_CAP.saturating_sub(q.queue.len());
                if room > 0 {
                    let take: usize = room.min(batch.len());
                    q.queue.extend(batch.drain(..take));
                }
                if local_pool.len() < max_local_pool && !q.pool.is_empty() {
                    let pull: usize = (max_local_pool - local_pool.len()).min(q.pool.len());
                    let split: usize = q.pool.len() - pull;
                    local_pool.extend(q.pool.drain(split..));
                }
                if let Some(waker) = q.waker.take() {
                    wakers_to_wake[shard_idx] = Some(waker);
                    wake_mask |= 1u64 << shard_idx;
                }
            }
            for dropped in batch.drain(..) {
                if local_pool.len() < max_local_pool
                    && dropped.data.capacity() <= QUIC_DATAGRAM_INIT_CAP * 4
                {
                    let mut buf: Vec<u8> = dropped.data;
                    buf.clear();
                    local_pool.push(buf);
                }
            }
        }
        let mut mask: u64 = wake_mask;
        while mask != 0 {
            let bit: usize = mask.trailing_zeros() as usize;
            mask &= mask - 1;
            if let Some(waker) = wakers_to_wake[bit].take() {
                waker.wake();
            }
        }
    }
    for queue in queues {
        let waker: Option<Waker> =
            queue
                .lock()
                .ok()
                .and_then(|mut q: MutexGuard<'_, ShardQueueState>| {
                    q.closed = true;
                    q.waker.take()
                });
        if let Some(waker) = waker {
            waker.wake();
        }
    }
}

/// Builds a tuned Quinn [`TransportConfig`] sized for high-concurrency HTTP/3 workloads.
///
/// # Returns
///
/// - `Arc<TransportConfig>` - Shared transport configuration for every QUIC endpoint shard.
fn build_quic_transport_config() -> Arc<TransportConfig> {
    let mut transport: TransportConfig = TransportConfig::default();
    transport.max_concurrent_bidi_streams(VarInt::from_u32(512));
    transport.stream_receive_window(VarInt::from_u32(256 * 1024));
    transport.receive_window(VarInt::from_u32(2 * 1024 * 1024));
    transport.send_window(2 * 1024 * 1024);
    transport.datagram_receive_buffer_size(None);
    Arc::new(transport)
}

/// Binds one exclusive OS UDP socket and creates `num_workers` CID-steered QUIC endpoints.
///
/// Single-worker runtimes bind one direct `Endpoint::server` with zero dispatcher overhead.
/// Multi-worker runtimes bind one exclusive OS `UdpSocket` (never `SO_REUSEPORT`, preserving
/// exclusive port ownership) and drive inbound `recvmmsg` demultiplexing on a dedicated I/O
/// thread so worker-thread handshake cryptography never starves UDP packet reception.
///
/// # Arguments
///
/// - `Arc<QuicServerConfig>` - Validated QUIC TLS configuration.
/// - `SocketAddr` - Exclusive UDP bind address.
///
/// # Returns
///
/// - `Result<QuicEndpointSet, Box<ServerError>>` - Ready QUIC endpoints and optional dispatcher.
pub(crate) fn create_quic_endpoint_set(
    quic_config: Arc<QuicServerConfig>,
    bind_addr: SocketAddr,
) -> Result<QuicEndpointSet, Box<ServerError>> {
    let transport_config: Arc<TransportConfig> = build_quic_transport_config();
    let workers: usize = Handle::current().metrics().num_workers();
    if workers <= 1 {
        let domain: Domain = if bind_addr.is_ipv6() {
            Domain::IPV6
        } else {
            Domain::IPV4
        };
        let socket: Socket = Socket::new(domain, Type::DGRAM, None)
            .map_err(|error: io::Error| Box::new(ServerError::from(error)))?;
        let _: io::Result<()> = socket.set_recv_buffer_size(QUIC_SHARD_SOCKET_BUF_SIZE);
        let _: io::Result<()> = socket.set_send_buffer_size(QUIC_SHARD_SOCKET_BUF_SIZE);
        socket
            .bind(&bind_addr.into())
            .map_err(|error: io::Error| Box::new(ServerError::from(error)))?;
        socket
            .set_nonblocking(true)
            .map_err(|error: io::Error| Box::new(ServerError::from(error)))?;
        let std_udp: StdUdpSocket = StdUdpSocket::from(socket);
        let endpoint: Endpoint = Endpoint::new(
            EndpointConfig::default(),
            Some(
                quinn::ServerConfig::with_crypto(quic_config)
                    .transport_config(transport_config)
                    .clone(),
            ),
            std_udp,
            Arc::new(TokioRuntime),
        )
        .map_err(|error: io::Error| Box::new(ServerError::from(error)))?;
        return Ok(QuicEndpointSet {
            endpoints: vec![endpoint],
            dispatcher: None,
            dispatcher_stop: None,
        });
    }
    let shard_count: usize = workers.min(64);
    let std_udp: StdUdpSocket = StdUdpSocket::bind(bind_addr)
        .map_err(|error: io::Error| Box::new(ServerError::from(error)))?;
    let sock_ref: SockRef<'_> = SockRef::from(&std_udp);
    let _: Result<(), io::Error> = sock_ref.set_recv_buffer_size(QUIC_SHARD_SOCKET_BUF_SIZE);
    let _: Result<(), io::Error> = sock_ref.set_send_buffer_size(QUIC_SHARD_SOCKET_BUF_SIZE);
    let rx_udp: StdUdpSocket = std_udp
        .try_clone()
        .map_err(|error: io::Error| Box::new(ServerError::from(error)))?;
    let runtime: Arc<dyn Runtime> = Arc::new(TokioRuntime);
    let tx_socket: Arc<dyn AsyncUdpSocket> = runtime
        .wrap_udp_socket(std_udp)
        .map_err(|error: io::Error| Box::new(ServerError::from(error)))?;
    let mut queues: Vec<Arc<Mutex<ShardQueueState>>> = Vec::with_capacity(shard_count);
    let mut endpoints: Vec<Endpoint> = Vec::with_capacity(shard_count);
    for shard_idx in 0..shard_count {
        let queue: Arc<Mutex<ShardQueueState>> = Arc::new(Mutex::new(ShardQueueState {
            queue: VecDeque::new(),
            pool: Vec::new(),
            waker: None,
            closed: false,
        }));
        let shard_socket: Arc<ShardUdpSocket> = Arc::new(ShardUdpSocket {
            real_socket: Arc::clone(&tx_socket),
            queue: Arc::clone(&queue),
            recycle: Mutex::new(Vec::with_capacity(BATCH_SIZE)),
        });
        let mut endpoint_config: EndpointConfig = EndpointConfig::default();
        let shard_id: u8 = shard_idx as u8;
        let total_shards: u8 = shard_count as u8;
        endpoint_config.cid_generator(move || {
            Box::new(ShardCidGenerator {
                shard_id,
                shard_count: total_shards,
            })
        });
        let crypto: Arc<QuicServerConfig> = Arc::clone(&quic_config);
        let endpoint: Endpoint = Endpoint::new_with_abstract_socket(
            endpoint_config,
            Some(
                quinn::ServerConfig::with_crypto(crypto)
                    .transport_config(Arc::clone(&transport_config))
                    .clone(),
            ),
            shard_socket,
            Arc::clone(&runtime),
        )
        .map_err(|error: io::Error| Box::new(ServerError::from(error)))?;
        queues.push(queue);
        endpoints.push(endpoint);
    }
    let (stop_tx, stop_rx): (Sender<()>, Receiver<()>) = channel(());
    let dispatcher: ThreadJoinHandle<()> = ThreadBuilder::new()
        .name(QUIC_DISPATCHER_THREAD_NAME.to_owned())
        .spawn(move || {
            let Ok(rx_runtime) = RuntimeBuilder::new_current_thread().enable_io().build() else {
                return;
            };
            rx_runtime.block_on(async move {
                let rx_quinn_rt: TokioRuntime = TokioRuntime;
                let Ok(rx_socket) = rx_quinn_rt.wrap_udp_socket(rx_udp) else {
                    return;
                };
                run_quic_shard_dispatcher(rx_socket, queues, stop_rx).await;
            });
        })
        .map_err(|error: io::Error| Box::new(ServerError::from(error)))?;
    Ok(QuicEndpointSet {
        endpoints,
        dispatcher: Some(dispatcher),
        dispatcher_stop: Some(stop_tx),
    })
}
