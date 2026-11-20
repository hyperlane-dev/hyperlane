use super::*;

/// Implements wakeup helpers for [`UringWakeState`].
impl UringWakeState {
    /// Signals the `eventfd` at most once per drain cycle to wake a sleeping `io_uring` worker.
    #[inline(always)]
    pub(crate) fn notify_eventfd(&self) {
        let Self {
            eventfd, signaled, ..
        } = self;
        if !signaled.swap(true, AtomicOrdering::AcqRel) {
            let val: u64 = 1;
            // SAFETY: `eventfd` is a valid open nonblocking eventfd and `val` is an 8-byte u64.
            let _: isize = unsafe {
                write(
                    *eventfd,
                    (&val as *const u64).cast::<c_void>(),
                    mem::size_of::<u64>(),
                )
            };
        }
    }

    /// Marks `slot_id` ready from any thread and wakes the owning `io_uring` worker.
    ///
    /// # Arguments
    ///
    /// - `usize` - Connection slot index to poll.
    #[inline(always)]
    pub(crate) fn wake_slot(&self, slot_id: usize) {
        let Self { ready, .. } = self;
        if let Ok(mut list) = ready.lock() {
            list.push(slot_id);
        }
        self.notify_eventfd();
    }

    /// Requests graceful shutdown and wakes the owning `io_uring` worker immediately.
    pub(crate) fn trigger_shutdown(&self) {
        let Self { shutdown, .. } = self;
        shutdown.store(true, AtomicOrdering::Release);
        self.notify_eventfd();
    }
}

/// Closes the underlying Linux `eventfd` when [`UringWakeState`] is dropped.
impl Drop for UringWakeState {
    /// Releases the Linux `eventfd` descriptor.
    fn drop(&mut self) {
        let Self { eventfd, .. } = self;
        if *eventfd >= 0 {
            // SAFETY: `eventfd` was created by `libc::eventfd` and owned exclusively by `UringWakeState`.
            let _: c_int = unsafe { close(*eventfd) };
            *eventfd = -1;
        }
    }
}

/// Implements [`Wake`] for [`UringSlotWaker`] so Tokio timers, channels, and async hooks resume seamlessly.
impl Wake for UringSlotWaker {
    /// Schedules the owning connection slot for polling on its `io_uring` worker.
    ///
    /// # Arguments
    ///
    /// - `Arc<Self>` - Shared handle to the slot waker.
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    /// Schedules the owning connection slot for polling without consuming the `Arc`.
    ///
    /// # Arguments
    ///
    /// - `&Arc<Self>` - Shared handle to the slot waker.
    fn wake_by_ref(self: &Arc<Self>) {
        let Self {
            slot_id,
            wake_state,
        } = &**self;
        wake_state.wake_slot(*slot_id);
    }
}

/// Inherent reactor helpers for [`UringReactor`].
impl UringReactor {
    /// Pops a recycled 16 KiB buffer or allocates a fresh zeroed fixed-address slice.
    ///
    /// # Returns
    ///
    /// - `Box<[u8]>` - A fixed-address heap buffer safe to pass to `io_uring`.
    #[inline(always)]
    pub(crate) fn take_buf(&mut self) -> Box<[u8]> {
        let Self { buf_pool, .. } = self;
        buf_pool
            .pop()
            .unwrap_or_else(|| vec![0u8; URING_IO_BUF_SIZE].into_boxed_slice())
    }

    /// Recycles a 16 KiB fixed-address buffer into the worker's local pool.
    ///
    /// # Arguments
    ///
    /// - `Box<[u8]>` - The buffer to recycle once no SQE references it.
    #[inline(always)]
    pub(crate) fn return_buf(&mut self, buf: Box<[u8]>) {
        let Self { buf_pool, .. } = self;
        if buf.len() == URING_IO_BUF_SIZE && buf_pool.len() < URING_MAX_POOLED_BUFS {
            buf_pool.push(buf);
        }
    }

    /// Appends one submission queue entry to the worker's ring, flushing if the SQ is full.
    ///
    /// # Arguments
    ///
    /// - `squeue::Entry` - Prepared `io_uring` submission entry.
    #[inline(always)]
    pub(crate) fn push_sqe(&mut self, entry: squeue::Entry) {
        let Self { ring, .. } = self;
        // SAFETY: Every pointer passed inside `entry` points into a fixed-address `Box`
        // owned by an active `UringAcceptSlot` or `UringConnSlot` kept alive until its CQE returns.
        let pushed: bool = unsafe { ring.submission().push(&entry).is_ok() };
        if !pushed {
            let _: io::Result<usize> = ring.submit();
            // SAFETY: Same lifetime invariant as above after freeing SQ space via `submit()`.
            let _: Result<(), _> = unsafe { ring.submission().push(&entry) };
        }
    }

    /// Schedules a connection slot for polling on the current worker tick without duplicates.
    ///
    /// # Arguments
    ///
    /// - `usize` - Connection slot index.
    #[inline(always)]
    pub(crate) fn schedule_local(&mut self, slot_id: usize) {
        let Self {
            slots, local_ready, ..
        } = self;
        if let Some(Some(slot)) = slots.get_mut(slot_id)
            && !slot.closing
            && !slot.in_local_ready
        {
            slot.in_local_ready = true;
            local_ready.push(slot_id);
        }
    }

    /// Submits any staged `write_buf` bytes, linking `Close` in-kernel when finishing a small response.
    ///
    /// # Arguments
    ///
    /// - `usize` - Connection slot index.
    /// - `bool` - Whether the connection task just completed (`Poll::Ready`).
    pub(crate) fn flush_staged_send(&mut self, slot_id: usize, link_close: bool) {
        let mut send_sqe: Option<squeue::Entry> = None;
        let mut close_sqe: Option<squeue::Entry> = None;
        {
            let Self { slots, .. } = self;
            if let Some(Some(slot)) = slots.get_mut(slot_id)
                && slot.send_staged
                && !slot.send_inflight
                && slot.fd >= 0
            {
                slot.send_staged = false;
                let len: usize = slot.write_len;
                let can_link_close: bool = link_close
                    && !slot.recv_inflight
                    && !slot.close_inflight
                    && slot.pending_write.is_empty()
                    && len <= 4096;
                slot.send_inflight = true;
                let mut send_op: opcode::Send =
                    opcode::Send::new(types::Fd(slot.fd), slot.write_buf.as_ptr(), len as u32);
                if can_link_close {
                    send_op = send_op.flags(MSG_MORE);
                }
                let mut entry: squeue::Entry = send_op
                    .build()
                    .user_data(encode_uring_user_data(URING_TAG_SEND, slot_id));
                if can_link_close {
                    entry = entry.flags(squeue::Flags::IO_LINK);
                    let fd: RawFd = slot.fd;
                    slot.fd = -1;
                    slot.close_inflight = true;
                    close_sqe = Some(
                        opcode::Close::new(types::Fd(fd))
                            .build()
                            .user_data(encode_uring_user_data(URING_TAG_CLOSE, slot_id)),
                    );
                }
                send_sqe = Some(entry);
            }
        }
        if let Some(entry) = send_sqe {
            self.push_sqe(entry);
        }
        if let Some(entry) = close_sqe {
            self.push_sqe(entry);
        }
    }

    /// Advances a closing connection slot until all in-flight I/O finishes, then submits `Close`.
    ///
    /// # Arguments
    ///
    /// - `usize` - Closing connection slot index.
    pub(crate) fn advance_close(&mut self, slot_id: usize) {
        let mut next_sqe: Option<squeue::Entry> = None;
        let mut cancel_recv: bool = false;
        let mut recycle_now: bool = false;
        {
            let Self { slots, .. } = self;
            if let Some(Some(slot)) = slots.get_mut(slot_id) {
                if slot.close_inflight {
                    return;
                }
                if !slot.send_inflight && !slot.pending_write.is_empty() && slot.write_err.is_none()
                {
                    let take: usize = slot.pending_write.len().min(slot.write_buf.len());
                    slot.write_buf[..take].copy_from_slice(&slot.pending_write[..take]);
                    slot.pending_write.drain(..take);
                    slot.write_pos = 0;
                    slot.write_len = take;
                    slot.send_inflight = true;
                    next_sqe = Some(
                        opcode::Send::new(types::Fd(slot.fd), slot.write_buf.as_ptr(), take as u32)
                            .build()
                            .user_data(encode_uring_user_data(URING_TAG_SEND, slot_id)),
                    );
                } else if slot.send_inflight {
                    return;
                } else if slot.recv_inflight {
                    cancel_recv = true;
                } else if slot.fd >= 0 {
                    let fd: RawFd = slot.fd;
                    slot.fd = -1;
                    slot.close_inflight = true;
                    next_sqe = Some(
                        opcode::Close::new(types::Fd(fd))
                            .build()
                            .user_data(encode_uring_user_data(URING_TAG_CLOSE, slot_id)),
                    );
                } else {
                    recycle_now = true;
                }
            }
        }
        if let Some(entry) = next_sqe {
            self.push_sqe(entry);
        }
        if cancel_recv {
            let cancel: squeue::Entry =
                opcode::AsyncCancel::new(encode_uring_user_data(URING_TAG_RECV, slot_id))
                    .build()
                    .user_data(encode_uring_user_data(URING_TAG_CANCEL, slot_id));
            self.push_sqe(cancel);
        }
        if recycle_now {
            self.recycle_slot(slot_id);
        }
    }

    /// Reclaims a fully closed connection slot and recycles its fixed buffers into the pool.
    ///
    /// # Arguments
    ///
    /// - `usize` - Finished connection slot index.
    pub(crate) fn recycle_slot(&mut self, slot_id: usize) {
        let taken: Option<UringConnSlot> = {
            let Self {
                slots,
                active_slots,
                free_slots,
                ..
            } = self;
            if let Some(slot_opt) = slots.get_mut(slot_id)
                && let Some(slot) = slot_opt.take()
            {
                *active_slots = active_slots.saturating_sub(1);
                free_slots.push(slot_id);
                Some(slot)
            } else {
                None
            }
        };
        if let Some(slot) = taken {
            self.return_buf(slot.read_buf);
            self.return_buf(slot.write_buf);
        }
    }
}

/// Implements [`Transport`] for [`UringTcpTransport`].
impl Transport for UringTcpTransport {
    /// Reports the actual remote peer address captured when the socket was accepted.
    ///
    /// # Returns
    ///
    /// - `io::Result<SocketAddr>` - The remote peer socket address.
    fn peer_addr(&self) -> io::Result<SocketAddr> {
        let Self { peers, .. } = self;
        Ok(peers.remote)
    }

    /// Reports the actual local listener address of the accepted socket.
    ///
    /// # Returns
    ///
    /// - `io::Result<SocketAddr>` - The local socket address.
    fn local_addr(&self) -> io::Result<SocketAddr> {
        let Self { peers, .. } = self;
        Ok(peers.local)
    }
}

/// Implements [`AsyncRead`] for [`UringTcpTransport`] backed by batched `opcode::Recv`.
impl AsyncRead for UringTcpTransport {
    /// Drains already-received bytes or arms one `opcode::Recv` SQE without issuing `recv(EAGAIN)`.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned transport.
    /// - `&mut task::Context<'_>` - The current task context.
    /// - `&mut ReadBuf<'_>` - Destination read buffer.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<()>>` - Ready when bytes, EOF, or an error is available.
    fn poll_read(
        self: Pin<&mut Self>,
        _cx: &mut task::Context<'_>,
        output: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let Self { slot_id, .. } = *self;
        if output.remaining() == 0 {
            return Poll::Ready(Ok(()));
        }
        URING_REACTOR.with(|cell: &RefCell<Option<UringReactor>>| {
            let mut guard: RefMut<'_, Option<UringReactor>> = cell.borrow_mut();
            let Some(reactor) = guard.as_mut() else {
                return Poll::Ready(Err(io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    URING_REACTOR_UNAVAILABLE,
                )));
            };
            let Some(Some(slot)) = reactor.slots.get_mut(slot_id) else {
                return Poll::Ready(Ok(()));
            };
            if slot.read_pos < slot.read_len {
                let avail: usize = slot.read_len - slot.read_pos;
                let take: usize = avail.min(output.remaining());
                output.put_slice(&slot.read_buf[slot.read_pos..slot.read_pos + take]);
                slot.read_pos += take;
                if slot.read_pos == slot.read_len {
                    slot.read_pos = 0;
                    slot.read_len = 0;
                }
                return Poll::Ready(Ok(()));
            }
            if let Some(errno) = slot.read_err.take() {
                return Poll::Ready(Err(io::Error::from_raw_os_error(errno)));
            }
            let closed_or_eof: bool = slot.eof || slot.fd < 0;
            if closed_or_eof || slot.closing {
                return Poll::Ready(Ok(()));
            }
            if !slot.recv_inflight {
                slot.recv_inflight = true;
                let entry: squeue::Entry = opcode::Recv::new(
                    types::Fd(slot.fd),
                    slot.read_buf.as_mut_ptr(),
                    slot.read_buf.len() as u32,
                )
                .build()
                .user_data(encode_uring_user_data(URING_TAG_RECV, slot_id));
                reactor.push_sqe(entry);
            }
            Poll::Pending
        })
    }
}

/// Implements [`AsyncWrite`] for [`UringTcpTransport`] backed by batched `opcode::Send`.
impl AsyncWrite for UringTcpTransport {
    /// Queues `bytes` directly into the connection's fixed send buffer and submits one `Send` SQE.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned transport.
    /// - `&mut task::Context<'_>` - The current task context.
    /// - `&[u8]` - Bytes to transmit.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<usize>>` - Number of bytes accepted for transmission.
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut task::Context<'_>,
        bytes: &[u8],
    ) -> Poll<io::Result<usize>> {
        let Self { slot_id, .. } = *self;
        if bytes.is_empty() {
            return Poll::Ready(Ok(0));
        }
        URING_REACTOR.with(|cell: &RefCell<Option<UringReactor>>| {
            let mut guard: RefMut<'_, Option<UringReactor>> = cell.borrow_mut();
            let Some(reactor) = guard.as_mut() else {
                return Poll::Ready(Err(io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    URING_REACTOR_UNAVAILABLE,
                )));
            };
            let mut flush_entry: Option<squeue::Entry> = None;
            let result: Poll<io::Result<usize>> = {
                let Some(Some(slot)) = reactor.slots.get_mut(slot_id) else {
                    return Poll::Ready(Err(io::Error::from_raw_os_error(EPIPE)));
                };
                if let Some(errno) = slot.write_err.take() {
                    return Poll::Ready(Err(io::Error::from_raw_os_error(errno)));
                }
                if slot.fd < 0 || slot.write_shutdown {
                    return Poll::Ready(Err(io::Error::from_raw_os_error(EPIPE)));
                }
                if !slot.send_inflight {
                    let space: usize = slot.write_buf.len().saturating_sub(slot.write_len);
                    if space > 0 {
                        let take: usize = bytes.len().min(space);
                        slot.write_buf[slot.write_len..slot.write_len + take]
                            .copy_from_slice(&bytes[..take]);
                        slot.write_len += take;
                        slot.send_staged = true;
                        return Poll::Ready(Ok(take));
                    }
                    let len: usize = slot.write_len;
                    slot.send_staged = false;
                    slot.send_inflight = true;
                    flush_entry = Some(
                        opcode::Send::new(types::Fd(slot.fd), slot.write_buf.as_ptr(), len as u32)
                            .build()
                            .user_data(encode_uring_user_data(URING_TAG_SEND, slot_id)),
                    );
                }
                let room: usize =
                    URING_PENDING_WRITE_LIMIT.saturating_sub(slot.pending_write.len());
                if room == 0 {
                    Poll::Pending
                } else {
                    let take: usize = bytes.len().min(room);
                    slot.pending_write.extend_from_slice(&bytes[..take]);
                    Poll::Ready(Ok(take))
                }
            };
            if let Some(entry) = flush_entry {
                reactor.push_sqe(entry);
            }
            result
        })
    }

    /// Coalesces multiple byte slices into one fixed-buffer `Send` SQE.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned transport.
    /// - `&mut task::Context<'_>` - The current task context.
    /// - `&[IoSlice<'_>]` - Ordered outbound slices.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<usize>>` - Total bytes queued for transmission.
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        _cx: &mut task::Context<'_>,
        slices: &[IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        let Self { slot_id, .. } = *self;
        let total: usize = slices.iter().map(|s: &IoSlice<'_>| s.len()).sum();
        if total == 0 {
            return Poll::Ready(Ok(0));
        }
        URING_REACTOR.with(|cell: &RefCell<Option<UringReactor>>| {
            let mut guard: RefMut<'_, Option<UringReactor>> = cell.borrow_mut();
            let Some(reactor) = guard.as_mut() else {
                return Poll::Ready(Err(io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    URING_REACTOR_UNAVAILABLE,
                )));
            };
            let mut flush_entry: Option<squeue::Entry> = None;
            let result: Poll<io::Result<usize>> = {
                let Some(Some(slot)) = reactor.slots.get_mut(slot_id) else {
                    return Poll::Ready(Err(io::Error::from_raw_os_error(EPIPE)));
                };
                if let Some(errno) = slot.write_err.take() {
                    return Poll::Ready(Err(io::Error::from_raw_os_error(errno)));
                }
                if slot.fd < 0 || slot.write_shutdown {
                    return Poll::Ready(Err(io::Error::from_raw_os_error(EPIPE)));
                }
                if !slot.send_inflight {
                    let cap: usize = slot.write_buf.len();
                    let mut copied: usize = 0;
                    for slice_item in slices {
                        let data: &[u8] = slice_item.as_ref();
                        if data.is_empty() {
                            continue;
                        }
                        let space: usize = cap.saturating_sub(slot.write_len);
                        let take: usize = data.len().min(space);
                        if take == 0 {
                            break;
                        }
                        slot.write_buf[slot.write_len..slot.write_len + take]
                            .copy_from_slice(&data[..take]);
                        slot.write_len += take;
                        copied += take;
                        if slot.write_len == cap {
                            break;
                        }
                    }
                    if copied > 0 {
                        slot.send_staged = true;
                        return Poll::Ready(Ok(copied));
                    }
                    let len: usize = slot.write_len;
                    slot.send_staged = false;
                    slot.send_inflight = true;
                    flush_entry = Some(
                        opcode::Send::new(types::Fd(slot.fd), slot.write_buf.as_ptr(), len as u32)
                            .build()
                            .user_data(encode_uring_user_data(URING_TAG_SEND, slot_id)),
                    );
                }
                let room: usize =
                    URING_PENDING_WRITE_LIMIT.saturating_sub(slot.pending_write.len());
                if room == 0 {
                    Poll::Pending
                } else {
                    let mut copied: usize = 0;
                    for slice_item in slices {
                        let data: &[u8] = slice_item.as_ref();
                        if data.is_empty() {
                            continue;
                        }
                        let take: usize = data.len().min(room - copied);
                        if take == 0 {
                            break;
                        }
                        slot.pending_write.extend_from_slice(&data[..take]);
                        copied += take;
                        if copied == room {
                            break;
                        }
                    }
                    Poll::Ready(Ok(copied))
                }
            };
            if let Some(entry) = flush_entry {
                reactor.push_sqe(entry);
            }
            result
        })
    }

    /// Reports vectored-write coalescing support.
    ///
    /// # Returns
    ///
    /// - `bool` - Always `true`.
    fn is_write_vectored(&self) -> bool {
        true
    }

    /// Matches Tokio `TcpStream::poll_flush` so keep-alive loops arm `Recv` in the same SQ batch.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned transport.
    /// - `&mut task::Context<'_>` - The current task context.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<()>>` - Always `Poll::Ready(Ok(()))`.
    fn poll_flush(self: Pin<&mut Self>, _cx: &mut task::Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    /// Drains queued outbound bytes and shuts down the write half of the TCP socket.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned transport.
    /// - `&mut task::Context<'_>` - The current task context.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<()>>` - Ready once pending writes are flushed and `SHUT_WR` is issued.
    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut task::Context<'_>) -> Poll<io::Result<()>> {
        let Self { slot_id, .. } = *self;
        URING_REACTOR.with(|cell: &RefCell<Option<UringReactor>>| {
            let mut guard: RefMut<'_, Option<UringReactor>> = cell.borrow_mut();
            let Some(reactor) = guard.as_mut() else {
                return Poll::Ready(Ok(()));
            };
            let Some(Some(slot)) = reactor.slots.get_mut(slot_id) else {
                return Poll::Ready(Ok(()));
            };
            slot.write_shutdown = true;
            if slot.send_staged && !slot.send_inflight && slot.fd >= 0 {
                let len: usize = slot.write_len;
                slot.send_staged = false;
                slot.send_inflight = true;
                let entry: squeue::Entry =
                    opcode::Send::new(types::Fd(slot.fd), slot.write_buf.as_ptr(), len as u32)
                        .build()
                        .user_data(encode_uring_user_data(URING_TAG_SEND, slot_id));
                reactor.push_sqe(entry);
                return Poll::Pending;
            }
            if slot.send_inflight || !slot.pending_write.is_empty() {
                return Poll::Pending;
            }
            if slot.fd >= 0 {
                // SAFETY: `slot.fd` is a live TCP socket owned by this connection slot.
                let _: c_int = unsafe { shutdown(slot.fd, SHUT_WR) };
            }
            Poll::Ready(Ok(()))
        })
    }
}
