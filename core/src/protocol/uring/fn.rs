use super::*;

/// Encodes an 8-bit operation tag and 56-bit slot index into an `io_uring` `user_data` value.
///
/// # Arguments
///
/// - `u64` - Operation tag (`URING_TAG_*`).
/// - `usize` - Slot or accept index.
///
/// # Returns
///
/// - `u64` - Packed `user_data` identifier.
#[inline(always)]
pub(crate) fn encode_uring_user_data(tag: u64, index: usize) -> u64 {
    (tag << URING_TAG_SHIFT) | ((index as u64) & URING_INDEX_MASK)
}

/// Decodes a packed `io_uring` `user_data` identifier into `(tag, index)`.
///
/// # Arguments
///
/// - `u64` - Packed `user_data` identifier.
///
/// # Returns
///
/// - `(u64, usize)` - Operation tag and slot index.
#[inline(always)]
pub(crate) fn decode_uring_user_data(user_data: u64) -> (u64, usize) {
    (
        user_data >> URING_TAG_SHIFT,
        (user_data & URING_INDEX_MASK) as usize,
    )
}

/// Attempts to create one Linux `io_uring` ring with the lowest-overhead kernel flags supported.
///
/// # Returns
///
/// - `Option<IoUring>` - A ready ring, or `None` when blocked by seccomp/kernel (`EPERM`/`ENOSYS`).
pub(crate) fn try_create_uring() -> Option<IoUring> {
    if let Ok(ring) = IoUring::builder()
        .setup_single_issuer()
        .setup_defer_taskrun()
        .setup_coop_taskrun()
        .build(URING_RING_ENTRIES)
    {
        return Some(ring);
    }
    if let Ok(ring) = IoUring::builder()
        .setup_coop_taskrun()
        .build(URING_RING_ENTRIES)
    {
        return Some(ring);
    }
    IoUring::new(URING_RING_ENTRIES).ok()
}

/// Probes whether Linux `io_uring` is permitted in the current container/kernel environment.
///
/// # Returns
///
/// - `bool` - `true` when `io_uring_setup` succeeds.
pub(crate) fn uring_is_available() -> bool {
    IoUring::new(8).is_ok()
}

/// Converts a kernel-filled `sockaddr_storage` from `opcode::Accept` into a Rust [`SocketAddr`].
///
/// # Arguments
///
/// - `&sockaddr_storage` - Kernel-written peer address buffer.
/// - `SocketAddr` - Fallback address when family is unexpected.
///
/// # Returns
///
/// - `SocketAddr` - Decoded IPv4 or IPv6 remote peer address.
#[inline(always)]
fn decode_sockaddr(storage: &sockaddr_storage, fallback: SocketAddr) -> SocketAddr {
    let family: c_int = c_int::from(storage.ss_family);
    if family == AF_INET {
        // SAFETY: `ss_family == AF_INET` guarantees `storage` contains a valid `sockaddr_in`.
        let addr_in: sockaddr_in =
            unsafe { ptr::read((storage as *const sockaddr_storage).cast::<sockaddr_in>()) };
        let ip: Ipv4Addr = Ipv4Addr::from(u32::from_be(addr_in.sin_addr.s_addr));
        let port: u16 = u16::from_be(addr_in.sin_port);
        return SocketAddr::V4(SocketAddrV4::new(ip, port));
    }
    if family == AF_INET6 {
        // SAFETY: `ss_family == AF_INET6` guarantees `storage` contains a valid `sockaddr_in6`.
        let addr_in6: sockaddr_in6 =
            unsafe { ptr::read((storage as *const sockaddr_storage).cast::<sockaddr_in6>()) };
        let ip: Ipv6Addr = Ipv6Addr::from(addr_in6.sin6_addr.s6_addr);
        let port: u16 = u16::from_be(addr_in6.sin6_port);
        return SocketAddr::V6(SocketAddrV6::new(
            ip,
            port,
            addr_in6.sin6_flowinfo,
            addr_in6.sin6_scope_id,
        ));
    }
    fallback
}

/// Applies optional IP TTL configured on `ServerConfig` to an accepted socket.
///
/// # Arguments
///
/// - `RawFd` - Accepted TCP socket descriptor.
/// - `u32` - Configured IP time-to-live value.
/// - `bool` - Whether the listener socket is IPv6.
#[inline(always)]
fn apply_socket_ttl(fd: RawFd, ttl: u32, is_v6: bool) {
    let val: c_int = ttl as c_int;
    let (level, opt): (c_int, c_int) = if is_v6 {
        (IPPROTO_IPV6, IPV6_UNICAST_HOPS)
    } else {
        (IPPROTO_IP, IP_TTL)
    };
    // SAFETY: `fd` is a valid TCP socket and `val` is a valid `c_int`.
    let _: c_int = unsafe {
        setsockopt(
            fd,
            level,
            opt,
            (&val as *const c_int).cast::<c_void>(),
            mem::size_of::<c_int>() as socklen_t,
        )
    };
}

/// Arms one `opcode::Accept` SQE on `listener_fd` using `slot`'s stable heap sockaddr storage.
///
/// # Arguments
///
/// - `&mut UringReactor` - The worker's `io_uring` reactor.
/// - `RawFd` - Exclusively bound listening TCP socket descriptor.
/// - `&mut UringAcceptSlot` - Fixed-address accept state slot.
/// - `usize` - Accept slot index.
#[inline(always)]
fn arm_accept_slot(
    reactor: &mut UringReactor,
    listener_fd: RawFd,
    slot: &mut UringAcceptSlot,
    idx: usize,
) {
    *slot.addr_len = mem::size_of::<sockaddr_storage>() as socklen_t;
    slot.inflight = true;
    let entry: squeue::Entry = opcode::Accept::new(
        types::Fd(listener_fd),
        (&mut *slot.addr as *mut sockaddr_storage).cast::<sockaddr>(),
        &mut *slot.addr_len as *mut socklen_t,
    )
    .flags(SOCK_NONBLOCK | SOCK_CLOEXEC)
    .build()
    .user_data(encode_uring_user_data(URING_TAG_ACCEPT, idx));
    reactor.push_sqe(entry);
}

/// Arms `opcode::PollAdd(POLLIN)` on the worker's `eventfd` wakeup descriptor.
///
/// # Arguments
///
/// - `&mut UringReactor` - The worker's `io_uring` reactor.
#[inline(always)]
fn arm_eventfd_poll(reactor: &mut UringReactor) {
    let entry: squeue::Entry =
        opcode::PollAdd::new(types::Fd(reactor.wake_state.eventfd), POLLIN as u32)
            .build()
            .user_data(encode_uring_user_data(URING_TAG_EVENTFD, 0));
    reactor.push_sqe(entry);
}

/// Runs one plaintext HTTP/1.1 connection over [`UringTcpTransport`].
///
/// # Arguments
///
/// - `ConnectionEnv` - Shared server, compiled pipeline, and request configuration.
/// - `UringTcpTransport` - Zero-syscall `io_uring` transport handle.
async fn run_uring_h1(env: ConnectionEnv, transport: UringTcpTransport) {
    let mut stream: Stream = Stream::from_transport(transport, env.shared.request_config, false);
    env.shared
        .server
        .serve_h1_connection_stream(&env.shared.pipeline, &mut stream)
        .await;
}

/// Runs one HTTP/1.1 TLS connection over `TlsTransport<UringTcpTransport>`.
///
/// # Arguments
///
/// - `ConnectionEnv` - Shared server, compiled pipeline, TLS config, and request configuration.
/// - `UringTcpTransport` - Zero-syscall `io_uring` transport handle.
async fn run_uring_h1_tls(env: ConnectionEnv, transport: UringTcpTransport) {
    let Some(tls_config) = env.shared.tls_config.as_ref() else {
        return;
    };
    let Ok(Ok(tls_stream)) = bounded_handshake(
        env.shared.request_config,
        accept_unbuffered_tls(Arc::clone(tls_config), transport),
    )
    .await
    else {
        return;
    };
    if !negotiated_protocol_matches(&tls_stream, &env.shared.protocol) {
        return;
    }
    let mut stream: Stream = Stream::from_transport(tls_stream, env.shared.request_config, false);
    env.shared
        .server
        .serve_h1_connection_stream(&env.shared.pipeline, &mut stream)
        .await;
    let _: Result<(), io::Error> = stream.stream.shutdown().await;
}

/// Polls every connection slot currently queued in `reactor.local_ready`.
///
/// Takes each slot's future out of thread-local storage before calling `poll` so that
/// [`UringTcpTransport::poll_read`] and [`UringTcpTransport::poll_write`] can mutably borrow
/// `URING_REACTOR` without re-entrancy conflicts.
fn poll_local_ready_slots() {
    loop {
        let next: Option<(usize, UringTaskFuture, Waker)> =
            URING_REACTOR.with(|cell: &RefCell<Option<UringReactor>>| {
                let mut guard: RefMut<'_, Option<UringReactor>> = cell.borrow_mut();
                let reactor: &mut UringReactor = guard.as_mut()?;
                while let Some(slot_id) = reactor.local_ready.pop() {
                    if let Some(Some(slot)) = reactor.slots.get_mut(slot_id) {
                        slot.in_local_ready = false;
                        if !slot.closing
                            && let Some(task) = slot.task.take()
                        {
                            let waker: Waker = slot.waker.clone();
                            return Some((slot_id, task, waker));
                        }
                    }
                }
                None
            });
        let Some((slot_id, mut task, waker)) = next else {
            break;
        };
        let mut cx: task::Context<'_> = task::Context::from_waker(&waker);
        let poll_res: Poll<()> = task.as_mut().poll(&mut cx);
        URING_REACTOR.with(|cell: &RefCell<Option<UringReactor>>| {
            let mut guard: RefMut<'_, Option<UringReactor>> = cell.borrow_mut();
            let Some(reactor) = guard.as_mut() else {
                return;
            };
            match poll_res {
                Poll::Pending => {
                    let is_closing: bool = reactor
                        .slots
                        .get_mut(slot_id)
                        .and_then(|opt: &mut Option<UringConnSlot>| opt.as_mut())
                        .is_some_and(|slot: &mut UringConnSlot| {
                            if slot.closing {
                                true
                            } else {
                                slot.task = Some(task);
                                false
                            }
                        });
                    reactor.flush_staged_send(slot_id, is_closing);
                    if is_closing {
                        reactor.advance_close(slot_id);
                    }
                }
                Poll::Ready(()) => {
                    drop(task);
                    if let Some(Some(slot)) = reactor.slots.get_mut(slot_id) {
                        slot.closing = true;
                    }
                    reactor.flush_staged_send(slot_id, true);
                    reactor.advance_close(slot_id);
                }
            }
        });
    }
}

/// Runs one dedicated Linux `io_uring` worker thread serving HTTP/1.1 or HTTP/1.1-TLS connections.
///
/// # Arguments
///
/// - `ConnectionEnv` - Shared server state, compiled pipeline, and TLS config.
/// - `RawFd` - Shared exclusively bound TCP listener file descriptor.
/// - `SocketAddr` - Local listener address.
/// - `Arc<UringWakeState>` - Shared wakeup channel for Tokio futures and graceful shutdown.
/// - `Handle` - Parent Tokio runtime handle entered so user hooks can call Tokio APIs.
/// - `usize` - Number of concurrent `Accept` SQEs armed on this worker.
fn run_uring_worker_thread(
    env: ConnectionEnv,
    listener_fd: RawFd,
    local_addr: SocketAddr,
    wake_state: Arc<UringWakeState>,
    tokio_handle: Handle,
    accept_slot_count: usize,
) {
    let _tokio_guard: EnterGuard<'_> = tokio_handle.enter();
    let Some(ring) = try_create_uring() else {
        return;
    };
    let configured_ttl: Option<u32> = env.shared.server.get_server_config().get_ttl();
    let is_v6: bool = local_addr.is_ipv6();
    let is_tls: bool = env.shared.tls_config.is_some();
    let mut accept_slots: Vec<UringAcceptSlot> = (0..accept_slot_count)
        .map(|_: usize| UringAcceptSlot {
            // SAFETY: `sockaddr_storage` is a plain C POD struct valid when zero-initialized.
            addr: Box::new(unsafe { mem::zeroed() }),
            addr_len: Box::new(mem::size_of::<sockaddr_storage>() as socklen_t),
            inflight: false,
        })
        .collect();
    URING_REACTOR.with(|cell: &RefCell<Option<UringReactor>>| {
        let mut reactor: UringReactor = UringReactor {
            ring,
            slots: Vec::with_capacity(256),
            free_slots: Vec::with_capacity(256),
            buf_pool: Vec::with_capacity(URING_MAX_POOLED_BUFS),
            waker_cache: Vec::with_capacity(256),
            local_ready: Vec::with_capacity(256),
            wake_state: Arc::clone(&wake_state),
            active_slots: 0,
        };
        arm_eventfd_poll(&mut reactor);
        for (idx, slot) in accept_slots.iter_mut().enumerate() {
            arm_accept_slot(&mut reactor, listener_fd, slot, idx);
        }
        *cell.borrow_mut() = Some(reactor);
    });
    let mut cqe_batch: Vec<(u64, i32)> = Vec::with_capacity(512);
    let mut shutdown_initiated: bool = false;
    loop {
        let should_exit: bool = URING_REACTOR.with(|cell: &RefCell<Option<UringReactor>>| {
            let mut guard: RefMut<'_, Option<UringReactor>> = cell.borrow_mut();
            let Some(reactor) = guard.as_mut() else {
                return true;
            };
            if shutdown_initiated
                && reactor.active_slots == 0
                && accept_slots.iter().all(|s: &UringAcceptSlot| !s.inflight)
            {
                return true;
            }
            let _: io::Result<usize> = reactor.ring.submit_and_wait(1);
            cqe_batch.clear();
            for cqe in reactor.ring.completion() {
                cqe_batch.push((cqe.user_data(), cqe.result()));
            }
            false
        });
        if should_exit {
            break;
        }
        let mut shutdown_now: bool = false;
        URING_REACTOR.with(|cell: &RefCell<Option<UringReactor>>| {
            let mut guard: RefMut<'_, Option<UringReactor>> = cell.borrow_mut();
            let Some(reactor) = guard.as_mut() else {
                return;
            };
            for &(user_data, res) in &cqe_batch {
                let (tag, idx): (u64, usize) = decode_uring_user_data(user_data);
                match tag {
                    URING_TAG_ACCEPT => {
                        if let Some(acc) = accept_slots.get_mut(idx) {
                            acc.inflight = false;
                            if res >= 0 && !shutdown_initiated {
                                let fd: RawFd = res;
                                if let Some(ttl) = configured_ttl {
                                    apply_socket_ttl(fd, ttl, is_v6);
                                }
                                let remote: SocketAddr = decode_sockaddr(&acc.addr, local_addr);
                                let peers: StreamPeers = StreamPeers {
                                    local: local_addr,
                                    remote,
                                };
                                let slot_id: usize = match reactor.free_slots.pop() {
                                    Some(id) => id,
                                    None => {
                                        let id: usize = reactor.slots.len();
                                        reactor.slots.push(None);
                                        reactor.waker_cache.push(None);
                                        id
                                    }
                                };
                                let read_buf: Box<[u8]> = reactor.take_buf();
                                let write_buf: Box<[u8]> = reactor.take_buf();
                                let waker: Waker = reactor.waker_cache[slot_id]
                                    .get_or_insert_with(|| {
                                        Waker::from(Arc::new(UringSlotWaker {
                                            slot_id,
                                            wake_state: Arc::clone(&reactor.wake_state),
                                        }))
                                    })
                                    .clone();
                                let transport: UringTcpTransport =
                                    UringTcpTransport { slot_id, peers };
                                let task: UringTaskFuture = if is_tls {
                                    Box::pin(run_uring_h1_tls(env.clone(), transport))
                                } else {
                                    Box::pin(run_uring_h1(env.clone(), transport))
                                };
                                reactor.slots[slot_id] = Some(UringConnSlot {
                                    fd,
                                    read_buf,
                                    read_pos: 0,
                                    read_len: 0,
                                    write_buf,
                                    write_pos: 0,
                                    write_len: 0,
                                    pending_write: Vec::new(),
                                    send_staged: false,
                                    recv_inflight: false,
                                    send_inflight: false,
                                    close_inflight: false,
                                    eof: false,
                                    write_shutdown: false,
                                    closing: false,
                                    read_err: None,
                                    write_err: None,
                                    waker,
                                    task: Some(task),
                                    in_local_ready: false,
                                });
                                reactor.active_slots += 1;
                                reactor.schedule_local(slot_id);
                            } else if res >= 0 {
                                // SAFETY: Shutdown already started; close the late accepted fd immediately.
                                let _: c_int = unsafe { close(res) };
                            }
                            if !shutdown_initiated {
                                arm_accept_slot(reactor, listener_fd, acc, idx);
                            }
                        }
                    }
                    URING_TAG_RECV => {
                        let mut should_close: bool = false;
                        if let Some(Some(slot)) = reactor.slots.get_mut(idx) {
                            slot.recv_inflight = false;
                            if res > 0 {
                                slot.read_pos = 0;
                                slot.read_len = res as usize;
                            } else if res == 0 {
                                slot.eof = true;
                            } else if res != -ECANCELED {
                                slot.read_err = Some(-res);
                            }
                            should_close = slot.closing;
                        }
                        if should_close {
                            reactor.advance_close(idx);
                        } else {
                            reactor.schedule_local(idx);
                        }
                    }
                    URING_TAG_SEND => {
                        let mut next_send: Option<squeue::Entry> = None;
                        let mut should_close: bool = false;
                        let mut should_wake: bool = false;
                        let mut should_recycle: bool = false;
                        if let Some(Some(slot)) = reactor.slots.get_mut(idx) {
                            slot.send_inflight = false;
                            if slot.close_inflight {
                                // Linked `Close` SQE already submitted; wait for `URING_TAG_CLOSE`.
                                slot.write_pos = 0;
                                slot.write_len = 0;
                                slot.pending_write.clear();
                            } else if slot.fd < 0 && slot.closing && !slot.recv_inflight {
                                should_recycle = true;
                            } else if res > 0 {
                                slot.write_pos += res as usize;
                                if slot.write_pos < slot.write_len {
                                    let remain: usize = slot.write_len - slot.write_pos;
                                    slot.send_inflight = true;
                                    // SAFETY: `write_pos < write_len <= write_buf.len()`.
                                    let ptr: *const u8 =
                                        unsafe { slot.write_buf.as_ptr().add(slot.write_pos) };
                                    next_send = Some(
                                        opcode::Send::new(types::Fd(slot.fd), ptr, remain as u32)
                                            .build()
                                            .user_data(encode_uring_user_data(URING_TAG_SEND, idx)),
                                    );
                                } else {
                                    slot.write_pos = 0;
                                    slot.write_len = 0;
                                    if !slot.pending_write.is_empty() {
                                        let take: usize =
                                            slot.pending_write.len().min(slot.write_buf.len());
                                        slot.write_buf[..take]
                                            .copy_from_slice(&slot.pending_write[..take]);
                                        slot.pending_write.drain(..take);
                                        slot.write_len = take;
                                        slot.send_inflight = true;
                                        next_send = Some(
                                            opcode::Send::new(
                                                types::Fd(slot.fd),
                                                slot.write_buf.as_ptr(),
                                                take as u32,
                                            )
                                            .build()
                                            .user_data(encode_uring_user_data(URING_TAG_SEND, idx)),
                                        );
                                        should_wake = true;
                                    } else if slot.closing {
                                        should_close = true;
                                    } else if slot.write_shutdown {
                                        should_wake = true;
                                    }
                                }
                            } else {
                                slot.write_pos = 0;
                                slot.write_len = 0;
                                slot.pending_write.clear();
                                slot.write_err = Some(if res == 0 { EPIPE } else { -res });
                                if slot.closing {
                                    should_close = true;
                                } else {
                                    should_wake = true;
                                }
                            }
                        }
                        if let Some(entry) = next_send {
                            reactor.push_sqe(entry);
                        }
                        if should_recycle {
                            reactor.recycle_slot(idx);
                        } else if should_close {
                            reactor.advance_close(idx);
                        } else if should_wake {
                            reactor.schedule_local(idx);
                        }
                    }
                    URING_TAG_CLOSE => {
                        let ready_to_recycle: bool = reactor
                            .slots
                            .get_mut(idx)
                            .and_then(|opt: &mut Option<UringConnSlot>| opt.as_mut())
                            .is_some_and(|slot: &mut UringConnSlot| {
                                slot.close_inflight = false;
                                !slot.send_inflight && !slot.recv_inflight
                            });
                        if ready_to_recycle {
                            reactor.recycle_slot(idx);
                        }
                    }
                    URING_TAG_EVENTFD => {
                        let mut counter: u64 = 0;
                        // SAFETY: Reading 8 bytes from a valid nonblocking eventfd drains the counter.
                        let _: isize = unsafe {
                            read(
                                reactor.wake_state.eventfd,
                                (&mut counter as *mut u64).cast::<c_void>(),
                                mem::size_of::<u64>(),
                            )
                        };
                        reactor
                            .wake_state
                            .signaled
                            .store(false, AtomicOrdering::Release);
                        let mut external: Vec<usize> = Vec::new();
                        if let Ok(mut ready_list) = reactor.wake_state.ready.lock() {
                            mem::swap(&mut *ready_list, &mut external);
                        }
                        for slot_id in external {
                            reactor.schedule_local(slot_id);
                        }
                        if reactor.wake_state.shutdown.load(AtomicOrdering::Acquire) {
                            shutdown_now = true;
                        } else {
                            arm_eventfd_poll(reactor);
                        }
                    }
                    _ => {}
                }
            }
            if !shutdown_initiated
                && (shutdown_now || reactor.wake_state.shutdown.load(AtomicOrdering::Acquire))
            {
                shutdown_initiated = true;
                for (acc_idx, acc) in accept_slots.iter().enumerate() {
                    if acc.inflight {
                        let cancel: squeue::Entry = opcode::AsyncCancel::new(
                            encode_uring_user_data(URING_TAG_ACCEPT, acc_idx),
                        )
                        .build()
                        .user_data(encode_uring_user_data(URING_TAG_CANCEL, acc_idx));
                        reactor.push_sqe(cancel);
                    }
                }
                let total_slots: usize = reactor.slots.len();
                for slot_id in 0..total_slots {
                    let needs_close: bool = if let Some(Some(slot)) = reactor.slots.get_mut(slot_id)
                    {
                        slot.task = None;
                        slot.closing = true;
                        true
                    } else {
                        false
                    };
                    if needs_close {
                        reactor.advance_close(slot_id);
                    }
                }
            }
        });
        if !shutdown_initiated {
            poll_local_ready_slots();
        }
    }
    URING_REACTOR.with(|cell: &RefCell<Option<UringReactor>>| {
        *cell.borrow_mut() = None;
    });
}

/// Serves `Http1` or `Http1Tls` TCP listeners on per-worker Linux `io_uring` threads when available.
///
/// Falls back transparently by returning `Err(listeners)` when `io_uring` is not available
/// (for example under a restrictive container seccomp policy or older kernel) or when the
/// protocol is not HTTP/1.1.
///
/// # Arguments
///
/// - `ConnectionEnv` - Shared server state, compiled pipeline, and TLS configuration.
/// - `Vec<TcpListener>` - Exclusively bound listener(s) handed over from `Server::run`.
/// - `Receiver<()>` - Graceful shutdown signal.
/// - `Sender<bool>` - Running-state publisher updated only after every worker exits and the socket closes.
///
/// # Returns
///
/// - `Result<(), Vec<TcpListener>>` - `Ok(())` after serving to shutdown, or `Err(listeners)` to fall back to Tokio.
pub(crate) async fn try_serve_uring_tcp(
    env: ConnectionEnv,
    mut listeners: Vec<TcpListener>,
    mut shutdown_receiver: Receiver<()>,
    running_sender: Sender<bool>,
) -> Result<(), Vec<TcpListener>> {
    if env.shared.protocol != ServerProtocol::Http1 || listeners.len() != 1 || !uring_is_available()
    {
        return Err(listeners);
    }
    let listener: TcpListener = listeners.remove(0);
    let Ok(local_addr) = listener.local_addr() else {
        return Err(vec![listener]);
    };
    let std_listener: StdTcpListener = match listener.into_std() {
        Ok(std_l) => std_l,
        Err(_) => return Ok(()),
    };
    let listener_fd: RawFd = std_listener.as_raw_fd();
    let tokio_handle: Handle = tokio::runtime::Handle::current();
    let worker_count: usize = tokio_handle.metrics().num_workers().clamp(1, 64);
    let accept_slot_count: usize = if worker_count > 1 {
        URING_MULTI_WORKER_ACCEPT_SLOTS
    } else {
        URING_SINGLE_WORKER_ACCEPT_SLOTS
    };
    let mut wake_states: Vec<Arc<UringWakeState>> = Vec::with_capacity(worker_count);
    let mut handles: Vec<ThreadJoinHandle<()>> = Vec::with_capacity(worker_count);
    for worker_idx in 0..worker_count {
        // SAFETY: `eventfd(0, EFD_NONBLOCK | EFD_CLOEXEC)` creates a fresh owned eventfd.
        let efd: RawFd = unsafe { eventfd(0, EFD_NONBLOCK | EFD_CLOEXEC) };
        if efd < 0 {
            for wake in &wake_states {
                wake.trigger_shutdown();
            }
            for handle in handles {
                let _: Result<Result<(), Box<dyn Any + Send>>, JoinError> =
                    spawn_blocking(move || handle.join()).await;
            }
            if let Ok(tokio_l) = TcpListener::from_std(std_listener) {
                return Err(vec![tokio_l]);
            }
            return Ok(());
        }
        let wake_state: Arc<UringWakeState> = Arc::new(UringWakeState {
            eventfd: efd,
            ready: Mutex::new(Vec::with_capacity(64)),
            signaled: AtomicBool::new(false),
            shutdown: AtomicBool::new(false),
        });
        let worker_env: ConnectionEnv = env.clone();
        let worker_wake: Arc<UringWakeState> = Arc::clone(&wake_state);
        let worker_tokio: Handle = tokio_handle.clone();
        let thread_name: String = format!("{URING_THREAD_NAME_PREFIX}{worker_idx}");
        let spawned: io::Result<ThreadJoinHandle<()>> =
            ThreadBuilder::new().name(thread_name).spawn(move || {
                run_uring_worker_thread(
                    worker_env,
                    listener_fd,
                    local_addr,
                    worker_wake,
                    worker_tokio,
                    accept_slot_count,
                );
            });
        match spawned {
            Ok(handle) => {
                wake_states.push(wake_state);
                handles.push(handle);
            }
            Err(_) => {
                for wake in &wake_states {
                    wake.trigger_shutdown();
                }
                for handle in handles {
                    let _: Result<Result<(), Box<dyn Any + Send>>, JoinError> =
                        spawn_blocking(move || handle.join()).await;
                }
                if let Ok(tokio_l) = TcpListener::from_std(std_listener) {
                    return Err(vec![tokio_l]);
                }
                return Ok(());
            }
        }
    }
    let _: Result<(), _> = shutdown_receiver.changed().await;
    for wake in &wake_states {
        wake.trigger_shutdown();
    }
    for handle in handles {
        let _: Result<Result<(), Box<dyn Any + Send>>, JoinError> =
            spawn_blocking(move || handle.join()).await;
    }
    drop(std_listener);
    let _: Result<(), SendError<bool>> = running_sender.send(SERVER_STOPPED);
    Ok(())
}
