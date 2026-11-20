use super::*;

/// Implements [`ConnectionIdGenerator`] for [`ShardCidGenerator`].
impl ConnectionIdGenerator for ShardCidGenerator {
    /// Generates an 8-byte cryptographically random Connection ID steering traffic to `shard_id`.
    ///
    /// # Returns
    ///
    /// - `ConnectionId` - A high-entropy 8-byte CID with `cid[0] % shard_count == shard_id`.
    fn generate_cid(&mut self) -> ConnectionId {
        let Self {
            shard_id,
            shard_count,
        } = self;
        let mut bytes: [u8; QUIC_SHARD_CID_LEN] = [0; QUIC_SHARD_CID_LEN];
        let secure_random: &dyn SecureRandom =
            *SHARD_SECURE_RANDOM.get_or_init(|| default_provider().secure_random);
        if secure_random.fill(&mut bytes).is_err() {
            // This trait cannot report entropy errors; never return a predictable CID.
            abort();
        }
        let count: u16 = u16::from((*shard_count).max(1));
        let buckets: u16 = 256 / count;
        let bucket: u16 = u16::from(bytes[0]) % buckets;
        bytes[0] = (bucket * count + u16::from(*shard_id)) as u8;
        ConnectionId::new(&bytes)
    }

    /// Returns the fixed 8-byte Connection ID length.
    ///
    /// # Returns
    ///
    /// - `usize` - Always `QUIC_SHARD_CID_LEN`.
    fn cid_len(&self) -> usize {
        QUIC_SHARD_CID_LEN
    }

    /// Returns no fixed retirement lifetime.
    ///
    /// # Returns
    ///
    /// - `Option<Duration>` - Always `None`.
    fn cid_lifetime(&self) -> Option<Duration> {
        None
    }
}

/// Implements [`fmt::Debug`] for [`ShardUdpSocket`].
impl fmt::Debug for ShardUdpSocket {
    /// Formats the virtual shard UDP socket for diagnostics.
    ///
    /// # Arguments
    ///
    /// - `&mut fmt::Formatter<'_>` - The destination formatter.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - The formatting result.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("ShardUdpSocket").finish()
    }
}

/// Implements [`AsyncUdpSocket`] for [`ShardUdpSocket`].
impl AsyncUdpSocket for ShardUdpSocket {
    /// Delegates write-readiness polling directly to the real exclusive OS UDP socket.
    ///
    /// # Arguments
    ///
    /// - `Arc<Self>` - Shared handle to this shard socket.
    ///
    /// # Returns
    ///
    /// - `Pin<Box<dyn UdpPoller>>` - The underlying OS socket write poller.
    fn create_io_poller(self: Arc<Self>) -> Pin<Box<dyn UdpPoller>> {
        let Self { real_socket, .. } = &*self;
        Arc::clone(real_socket).create_io_poller()
    }

    /// Sends outbound QUIC datagrams directly on the shared exclusive OS UDP socket.
    ///
    /// # Arguments
    ///
    /// - `&Transmit<'_>` - The outbound UDP transmit batch.
    ///
    /// # Returns
    ///
    /// - `io::Result<()>` - The direct OS send result.
    fn try_send(&self, transmit: &Transmit<'_>) -> io::Result<()> {
        let Self { real_socket, .. } = self;
        real_socket.try_send(transmit)
    }

    /// Drains CID-steered inbound datagrams from this shard's queue into Quinn's receive buffers.
    ///
    /// # Arguments
    ///
    /// - `&mut task::Context<'_>` - The `EndpointDriver` task context.
    /// - `&mut [IoSliceMut<'_>]` - Destination receive slices supplied by Quinn.
    /// - `&mut [RecvMeta]` - Destination metadata slots supplied by Quinn.
    ///
    /// # Returns
    ///
    /// - `Poll<io::Result<usize>>` - Number of datagrams delivered, or pending.
    fn poll_recv(
        &self,
        cx: &mut task::Context<'_>,
        bufs: &mut [IoSliceMut<'_>],
        meta: &mut [RecvMeta],
    ) -> Poll<io::Result<usize>> {
        let Self { queue, .. } = self;
        let max_take: usize = bufs.len().min(meta.len());
        if max_take == 0 {
            return Poll::Ready(Ok(0));
        }
        let Ok(mut state) = queue.lock() else {
            return Poll::Ready(Err(io::Error::other("quic shard queue poisoned")));
        };
        let mut delivered: usize = 0;
        while delivered < max_take {
            let Some(mut datagram) = state.queue.pop_front() else {
                break;
            };
            let copy_len: usize = datagram.data.len().min(bufs[delivered].len());
            bufs[delivered][..copy_len].copy_from_slice(&datagram.data[..copy_len]);
            let mut item_meta: RecvMeta = datagram.meta;
            item_meta.len = copy_len;
            item_meta.stride = copy_len;
            meta[delivered] = item_meta;
            delivered += 1;
            if state.pool.len() < QUIC_SHARD_POOL_CAP
                && datagram.data.capacity() <= QUIC_DATAGRAM_INIT_CAP * 4
            {
                datagram.data.clear();
                state.pool.push(datagram.data);
            }
        }
        if delivered == 0 {
            if state.closed {
                return Poll::Ready(Err(io::Error::new(
                    io::ErrorKind::ConnectionAborted,
                    "quic shard dispatcher stopped",
                )));
            }
            state.waker = Some(cx.waker().clone());
            return Poll::Pending;
        }
        Poll::Ready(Ok(delivered))
    }

    /// Reports the local socket address of the exclusively bound OS UDP socket.
    ///
    /// # Returns
    ///
    /// - `io::Result<SocketAddr>` - The bound local address.
    fn local_addr(&self) -> io::Result<SocketAddr> {
        let Self { real_socket, .. } = self;
        real_socket.local_addr()
    }

    /// Preserves the real socket's outbound GSO segment capability.
    ///
    /// # Returns
    ///
    /// - `usize` - Maximum outbound segments per transmit.
    fn max_transmit_segments(&self) -> usize {
        let Self { real_socket, .. } = self;
        real_socket.max_transmit_segments()
    }

    /// Each forwarded shard item is a single uncoalesced UDP datagram.
    ///
    /// # Returns
    ///
    /// - `usize` - Always `1`.
    fn max_receive_segments(&self) -> usize {
        1
    }

    /// Preserves the real socket's DF / fragmentation capability.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether datagrams may fragment.
    fn may_fragment(&self) -> bool {
        let Self { real_socket, .. } = self;
        real_socket.may_fragment()
    }
}
