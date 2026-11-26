use super::*;

/// Implements wakeup helpers for [`UringWakeState`].
impl UringWakeState {
    /// Signals the `eventfd` at most once per drain cycle to wake a sleeping `io_uring` worker.
    #[inline(always)]
    pub(crate) fn notify_eventfd(&self) {
        let Self {
            eventfd, signaled, ..
        } = self;
        if !signaled.swap(true, AtomicOrdering::SeqCst) {
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
        shutdown.store(true, AtomicOrdering::SeqCst);
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

    /// Enqueues an operation without dropping it when the kernel SQ is full.
    ///
    /// # Arguments
    ///
    /// - `squeue::Entry` - Prepared submission with buffers retained until its CQE.
    #[inline(always)]
    pub(crate) fn push_sqe(&mut self, entry: squeue::Entry) {
        let Self {
            ring,
            pending_submissions,
            ..
        } = self;
        if pending_submissions.is_empty() {
            // SAFETY: The operation owns stable buffers through its slot until completion.
            if unsafe { ring.submission().push(&entry) }.is_ok() {
                return;
            }
        }
        pending_submissions.push_back(UringSubmission::Single(entry));
    }

    /// Enqueues a linked send/close group atomically, preserving FIFO ordering.
    ///
    /// # Arguments
    ///
    /// - `[squeue::Entry; 2]` - The send and dependent close entries.
    fn push_linked_pair(&mut self, entries: [squeue::Entry; 2]) {
        let Self {
            ring,
            pending_submissions,
            ..
        } = self;
        if pending_submissions.is_empty() {
            // SAFETY: Both entries retain their slot and buffers until both CQEs arrive.
            if unsafe { ring.submission().push_multiple(&entries) }.is_ok() {
                return;
            }
        }
        pending_submissions.push_back(UringSubmission::Linked(entries));
    }

    /// Submits complete queued groups, waiting only after every group reaches the SQ.
    ///
    /// # Returns
    ///
    /// - `io::Result<usize>` - The kernel submission result; queued groups remain owned on failure.
    pub(crate) fn submit_io(&mut self) -> io::Result<usize> {
        let Self {
            ring,
            pending_submissions,
            ..
        } = self;
        while let Some(group) = pending_submissions.front() {
            let entries: &[squeue::Entry] = match group {
                UringSubmission::Single(entry) => slice::from_ref(entry),
                UringSubmission::Linked(pair) => pair,
            };
            // SAFETY: Pending entries retain exactly the same fixed-buffer ownership as direct SQ entries.
            if unsafe { ring.submission().push_multiple(entries) }.is_err() {
                return ring.submit();
            }
            pending_submissions.pop_front();
        }
        ring.submit_and_wait(1)
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
                && !slot.close_inflight
                && slot.fd >= 0
            {
                slot.send_staged = false;
                let len: usize = slot.write_len.saturating_sub(slot.write_pos);
                if len == 0 {
                    return;
                }
                let can_link_close: bool = link_close
                    && !slot.recv_inflight
                    && slot.pending_write.is_empty()
                    && len <= URING_LINKED_SEND_LIMIT;
                slot.send_inflight = true;
                // SAFETY: The initialized unsent interval is write_pos..write_len.
                let data: *const u8 = unsafe { slot.write_buf.as_ptr().add(slot.write_pos) };
                let mut send_op: opcode::Send =
                    opcode::Send::new(types::Fd(slot.fd), data, len as u32);
                if can_link_close {
                    // MSG_WAITALL makes a short send break the link instead of closing early.
                    send_op = send_op.flags(MSG_MORE | MSG_WAITALL);
                }
                let mut entry: squeue::Entry = send_op
                    .build()
                    .user_data(encode_uring_user_data(URING_TAG_SEND, slot_id));
                if can_link_close {
                    entry = entry.flags(squeue::Flags::IO_LINK);
                    slot.close_inflight = true;
                    close_sqe = Some(
                        opcode::Close::new(types::Fd(slot.fd))
                            .build()
                            .user_data(encode_uring_user_data(URING_TAG_CLOSE, slot_id)),
                    );
                }
                send_sqe = Some(entry);
            }
        }
        match (send_sqe, close_sqe) {
            (Some(send), Some(close)) => self.push_linked_pair([send, close]),
            (Some(send), None) => self.push_sqe(send),
            _ => {}
        }
    }

    /// Applies a send CQE while retaining short-send state until a linked close resolves.
    ///
    /// # Arguments
    ///
    /// - `usize` - Connection slot index.
    /// - `i32` - Kernel send result, including negative errno values.
    pub(crate) fn complete_send(&mut self, slot_id: usize, result: i32) {
        let closing: bool;
        let mut staged: bool = false;
        let mut wake: bool = false;
        {
            let Self { slots, .. } = self;
            let Some(Some(slot)) = slots.get_mut(slot_id) else {
                return;
            };
            slot.send_inflight = false;
            if result > 0 {
                slot.write_pos += result as usize;
            } else {
                slot.write_err = Some(if result == 0 { EPIPE } else { -result });
                slot.write_pos = 0;
                slot.write_len = 0;
                slot.pending_write.clear();
                slot.send_staged = false;
                wake = true;
            }
            if slot.close_inflight || slot.shutdown_inflight {
                return;
            }
            closing = slot.closing;
            if slot.write_err.is_none() && slot.fd >= 0 {
                if slot.write_pos < slot.write_len {
                    slot.send_staged = true;
                    staged = true;
                } else {
                    slot.write_pos = 0;
                    slot.write_len = 0;
                    if !slot.pending_write.is_empty() {
                        let count: usize = slot.pending_write.len().min(slot.write_buf.len());
                        slot.write_buf[..count].copy_from_slice(&slot.pending_write[..count]);
                        slot.pending_write.drain(..count);
                        slot.write_len = count;
                        slot.send_staged = true;
                        staged = true;
                        wake = true;
                    } else {
                        wake |= slot.flush_pending || slot.write_shutdown;
                    }
                }
            }
        }
        if staged {
            self.flush_staged_send(slot_id, closing);
        }
        if closing {
            self.advance_close(slot_id);
        } else if wake {
            self.schedule_local(slot_id);
        }
    }

    /// Applies a close CQE, recovering the live descriptor when linkage cancels the close.
    ///
    /// # Arguments
    ///
    /// - `usize` - Connection slot index.
    /// - `i32` - Kernel close result, including negative errno values.
    pub(crate) fn complete_close(&mut self, slot_id: usize, result: i32) {
        let mut recycle: bool = false;
        let mut retry: bool = false;
        {
            let Self { slots, .. } = self;
            if let Some(Some(slot)) = slots.get_mut(slot_id) {
                slot.close_inflight = false;
                if result == -ECANCELED {
                    retry = true;
                } else {
                    slot.fd = -1;
                    recycle = !slot.send_inflight
                        && !slot.recv_inflight
                        && !slot.cancel_inflight
                        && !slot.shutdown_inflight;
                }
            }
        }
        if recycle {
            self.recycle_slot(slot_id);
        } else if retry {
            self.advance_close(slot_id);
        }
    }

    /// Releases a receive cancellation only after its own CQE arrives.
    ///
    /// # Arguments
    ///
    /// - `usize` - Connection slot whose cancellation completed.
    pub(crate) fn complete_cancel(&mut self, slot_id: usize) {
        let closing: bool = {
            let Self { slots, .. } = self;
            let Some(Some(slot)) = slots.get_mut(slot_id) else {
                return;
            };
            if !slot.cancel_inflight {
                return;
            }
            slot.cancel_inflight = false;
            slot.closing
        };
        if closing {
            self.advance_close(slot_id);
        }
    }

    /// Applies an `opcode::Shutdown(SHUT_WR)` CQE and resumes pending shutdown or close steps.
    ///
    /// # Arguments
    ///
    /// - `usize` - Connection slot index.
    /// - `i32` - Kernel shutdown result, including negative errno values.
    pub(crate) fn complete_shutdown(&mut self, slot_id: usize, result: i32) {
        let closing: bool;
        let mut staged: bool = false;
        let mut wake: bool = false;
        {
            let Self { slots, .. } = self;
            let Some(Some(slot)) = slots.get_mut(slot_id) else {
                return;
            };
            slot.shutdown_inflight = false;
            if result == 0 {
                slot.shutdown_done = true;
            } else if result != -ECANCELED && slot.write_err.is_none() {
                slot.write_err = Some(-result);
            }
            closing = slot.closing;
            if !closing && slot.write_err.is_none() && slot.fd >= 0 {
                if slot.write_pos < slot.write_len && !slot.send_inflight {
                    slot.send_staged = true;
                    staged = true;
                } else {
                    wake = true;
                }
            } else if !closing {
                wake = true;
            }
        }
        if staged {
            self.flush_staged_send_for_shutdown(slot_id);
        }
        if closing {
            self.advance_close(slot_id);
        } else if wake {
            self.schedule_local(slot_id);
        }
    }

    /// Submits staged bytes linked atomically with `opcode::Shutdown(SHUT_WR)` so response + `FIN` coalesce.
    ///
    /// # Arguments
    ///
    /// - `usize` - Connection slot index.
    pub(crate) fn flush_staged_send_for_shutdown(&mut self, slot_id: usize) {
        let mut send_sqe: Option<squeue::Entry> = None;
        let mut shutdown_sqe: Option<squeue::Entry> = None;
        {
            let Self { slots, .. } = self;
            if let Some(Some(slot)) = slots.get_mut(slot_id)
                && slot.send_staged
                && !slot.send_inflight
                && !slot.close_inflight
                && !slot.shutdown_inflight
                && slot.fd >= 0
            {
                slot.send_staged = false;
                let len: usize = slot.write_len.saturating_sub(slot.write_pos);
                if len == 0 {
                    return;
                }
                let can_link_shutdown: bool = !slot.shutdown_done
                    && slot.pending_write.is_empty()
                    && len <= URING_LINKED_SEND_LIMIT;
                slot.send_inflight = true;
                // SAFETY: The initialized unsent interval is write_pos..write_len.
                let data: *const u8 = unsafe { slot.write_buf.as_ptr().add(slot.write_pos) };
                let mut send_op: opcode::Send =
                    opcode::Send::new(types::Fd(slot.fd), data, len as u32);
                if can_link_shutdown {
                    send_op = send_op.flags(MSG_MORE | MSG_WAITALL);
                }
                let mut entry: squeue::Entry = send_op
                    .build()
                    .user_data(encode_uring_user_data(URING_TAG_SEND, slot_id));
                if can_link_shutdown {
                    entry = entry.flags(squeue::Flags::IO_LINK);
                    slot.shutdown_inflight = true;
                    shutdown_sqe = Some(
                        opcode::Shutdown::new(types::Fd(slot.fd), SHUT_WR)
                            .build()
                            .user_data(encode_uring_user_data(URING_TAG_SHUTDOWN, slot_id)),
                    );
                }
                send_sqe = Some(entry);
            }
        }
        match (send_sqe, shutdown_sqe) {
            (Some(send), Some(shut)) => self.push_linked_pair([send, shut]),
            (Some(send), None) => self.push_sqe(send),
            _ => {}
        }
    }

    /// Advances a closing connection slot until all in-flight I/O finishes, then submits `Close`.
    ///
    /// # Arguments
    ///
    /// - `usize` - Closing connection slot index.
    pub(crate) fn advance_close(&mut self, slot_id: usize) {
        let mut next_sqe: Option<squeue::Entry> = None;
        let mut staged: bool = false;
        let mut recycle: bool = false;
        {
            let Self { slots, .. } = self;
            if let Some(Some(slot)) = slots.get_mut(slot_id) {
                let active_close: bool = slot.close_inflight || slot.shutdown_inflight;
                if active_close || slot.send_inflight {
                    return;
                }
                if slot.write_err.is_none() && slot.write_pos < slot.write_len {
                    slot.send_staged = true;
                    staged = true;
                } else if slot.write_err.is_none() && !slot.pending_write.is_empty() {
                    let count: usize = slot.pending_write.len().min(slot.write_buf.len());
                    slot.write_buf[..count].copy_from_slice(&slot.pending_write[..count]);
                    slot.pending_write.drain(..count);
                    slot.write_pos = 0;
                    slot.write_len = count;
                    slot.send_staged = true;
                    staged = true;
                } else if slot.recv_inflight {
                    if slot.cancel_inflight {
                        return;
                    }
                    slot.cancel_inflight = true;
                    next_sqe = Some(
                        opcode::AsyncCancel::new(encode_uring_user_data(URING_TAG_RECV, slot_id))
                            .build()
                            .user_data(encode_uring_user_data(URING_TAG_CANCEL, slot_id)),
                    );
                } else if slot.cancel_inflight {
                    return;
                } else if slot.fd >= 0 {
                    slot.close_inflight = true;
                    next_sqe = Some(
                        opcode::Close::new(types::Fd(slot.fd))
                            .build()
                            .user_data(encode_uring_user_data(URING_TAG_CLOSE, slot_id)),
                    );
                } else {
                    recycle = true;
                }
            }
        }
        if staged {
            self.flush_staged_send(slot_id, true);
        }
        if let Some(entry) = next_sqe {
            self.push_sqe(entry);
        }
        if recycle {
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
        let Self {
            slot_id,
            connection_id,
            owner_thread,
            ..
        } = *self;
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
            if reactor.owner_thread != owner_thread {
                return Poll::Ready(Err(io::Error::from_raw_os_error(EPIPE)));
            }
            let Some(Some(slot)) = reactor.slots.get_mut(slot_id) else {
                return Poll::Ready(Ok(()));
            };
            if slot.connection_id != connection_id {
                return Poll::Ready(Err(io::Error::from_raw_os_error(EPIPE)));
            }
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
        let Self {
            slot_id,
            connection_id,
            owner_thread,
            ..
        } = *self;
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
            if reactor.owner_thread != owner_thread {
                return Poll::Ready(Err(io::Error::from_raw_os_error(EPIPE)));
            }
            let mut flush_entry: Option<squeue::Entry> = None;
            let result: Poll<io::Result<usize>> = {
                let Some(Some(slot)) = reactor.slots.get_mut(slot_id) else {
                    return Poll::Ready(Err(io::Error::from_raw_os_error(EPIPE)));
                };
                if slot.connection_id != connection_id {
                    return Poll::Ready(Err(io::Error::from_raw_os_error(EPIPE)));
                }
                if let Some(errno) = slot.write_err.take() {
                    return Poll::Ready(Err(io::Error::from_raw_os_error(errno)));
                }
                let closed: bool = slot.fd < 0 || slot.closing;
                if closed || slot.write_shutdown {
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
        let Self {
            slot_id,
            connection_id,
            owner_thread,
            ..
        } = *self;
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
            if reactor.owner_thread != owner_thread {
                return Poll::Ready(Err(io::Error::from_raw_os_error(EPIPE)));
            }
            let mut flush_entry: Option<squeue::Entry> = None;
            let result: Poll<io::Result<usize>> = {
                let Some(Some(slot)) = reactor.slots.get_mut(slot_id) else {
                    return Poll::Ready(Err(io::Error::from_raw_os_error(EPIPE)));
                };
                if slot.connection_id != connection_id {
                    return Poll::Ready(Err(io::Error::from_raw_os_error(EPIPE)));
                }
                if let Some(errno) = slot.write_err.take() {
                    return Poll::Ready(Err(io::Error::from_raw_os_error(errno)));
                }
                let closed: bool = slot.fd < 0 || slot.closing;
                if closed || slot.write_shutdown {
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

    /// Completes only after all accepted bytes have completed their kernel send operations.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned transport.
    /// - `&mut task::Context<'_>` - The current task context.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<()>>` - Pending while buffered or in-flight bytes remain, or the send error.
    fn poll_flush(self: Pin<&mut Self>, _cx: &mut task::Context<'_>) -> Poll<io::Result<()>> {
        let Self {
            slot_id,
            connection_id,
            owner_thread,
            ..
        } = *self;
        URING_REACTOR.with(|cell: &RefCell<Option<UringReactor>>| {
            let mut guard: RefMut<'_, Option<UringReactor>> = cell.borrow_mut();
            let Some(reactor) = guard.as_mut() else {
                return Poll::Ready(Err(io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    URING_REACTOR_UNAVAILABLE,
                )));
            };
            if reactor.owner_thread != owner_thread {
                return Poll::Ready(Err(io::Error::from_raw_os_error(EPIPE)));
            }
            let mut read_ahead: Option<squeue::Entry> = None;
            let pending: bool = {
                let Some(Some(slot)) = reactor.slots.get_mut(slot_id) else {
                    return Poll::Ready(Err(io::Error::from_raw_os_error(EPIPE)));
                };
                if slot.connection_id != connection_id {
                    return Poll::Ready(Err(io::Error::from_raw_os_error(EPIPE)));
                }
                if let Some(errno) = slot.write_err {
                    return Poll::Ready(Err(io::Error::from_raw_os_error(errno)));
                }
                if slot.fd < 0 || slot.close_inflight {
                    return Poll::Ready(Err(io::Error::from_raw_os_error(EPIPE)));
                }
                let active: bool = slot.send_staged || slot.send_inflight;
                let buffered: bool =
                    slot.write_pos < slot.write_len || !slot.pending_write.is_empty();
                let pending: bool = active || buffered;
                slot.flush_pending = pending;
                if pending
                    && !slot.recv_inflight
                    && !slot.eof
                    && !slot.closing
                    && !slot.write_shutdown
                    && slot.read_err.is_none()
                    && slot.read_pos == slot.read_len
                {
                    slot.recv_inflight = true;
                    read_ahead = Some(
                        opcode::Recv::new(
                            types::Fd(slot.fd),
                            slot.read_buf.as_mut_ptr(),
                            slot.read_buf.len() as u32,
                        )
                        .build()
                        .user_data(encode_uring_user_data(URING_TAG_RECV, slot_id)),
                    );
                }
                pending
            };
            if pending {
                reactor.flush_staged_send(slot_id, false);
                if let Some(entry) = read_ahead {
                    reactor.push_sqe(entry);
                }
                Poll::Pending
            } else {
                Poll::Ready(Ok(()))
            }
        })
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
        let Self {
            slot_id,
            connection_id,
            owner_thread,
            ..
        } = *self;
        URING_REACTOR.with(|cell: &RefCell<Option<UringReactor>>| {
            let mut guard: RefMut<'_, Option<UringReactor>> = cell.borrow_mut();
            let Some(reactor) = guard.as_mut() else {
                return Poll::Ready(Err(io::Error::from_raw_os_error(EPIPE)));
            };
            if reactor.owner_thread != owner_thread {
                return Poll::Ready(Err(io::Error::from_raw_os_error(EPIPE)));
            }
            let mut link_staged: bool = false;
            let mut still_pending: bool = false;
            {
                let Some(Some(slot)) = reactor.slots.get_mut(slot_id) else {
                    return Poll::Ready(Err(io::Error::from_raw_os_error(EPIPE)));
                };
                if slot.connection_id != connection_id {
                    return Poll::Ready(Err(io::Error::from_raw_os_error(EPIPE)));
                }
                slot.write_shutdown = true;
                if let Some(errno) = slot.write_err {
                    return Poll::Ready(Err(io::Error::from_raw_os_error(errno)));
                }
                if slot.fd < 0 || slot.close_inflight {
                    return Poll::Ready(Err(io::Error::from_raw_os_error(EPIPE)));
                }
                if slot.shutdown_inflight {
                    return Poll::Pending;
                }
                if slot.send_staged && !slot.send_inflight {
                    link_staged = true;
                    still_pending = true;
                } else {
                    let active: bool = slot.send_inflight || !slot.pending_write.is_empty();
                    let buffered: bool = slot.write_pos < slot.write_len;
                    if active || buffered {
                        slot.flush_pending = true;
                        still_pending = true;
                    } else if slot.shutdown_done {
                        return Poll::Ready(Ok(()));
                    }
                }
            }
            if link_staged {
                reactor.flush_staged_send_for_shutdown(slot_id);
                return Poll::Pending;
            }
            if still_pending {
                return Poll::Pending;
            }
            let Some(Some(slot)) = reactor.slots.get_mut(slot_id) else {
                return Poll::Ready(Err(io::Error::from_raw_os_error(EPIPE)));
            };
            // SAFETY: Every send completed and the slot still owns this live TCP descriptor.
            if unsafe { shutdown(slot.fd, SHUT_WR) } < 0 {
                Poll::Ready(Err(io::Error::last_os_error()))
            } else {
                slot.shutdown_done = true;
                Poll::Ready(Ok(()))
            }
        })
    }
}
