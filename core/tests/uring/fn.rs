use super::*;

fn with_uring_slot(check: impl FnOnce(&mut UringTcpTransport)) -> bool {
    let Some(ring) = try_create_uring() else {
        assert!(!uring_is_available());
        eprintln!("io_uring unavailable; the missing-reactor regression remains active");
        return false;
    };
    let listener: StdTcpListener = StdTcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
    let client: net::TcpStream = net::TcpStream::connect(listener.local_addr().unwrap()).unwrap();
    let (server, remote): (net::TcpStream, SocketAddr) = listener.accept().unwrap();
    server.set_nonblocking(true).unwrap();
    let peers: StreamPeers = StreamPeers {
        local: server.local_addr().unwrap(),
        remote,
    };
    let wake_state: Arc<UringWakeState> = Arc::new(UringWakeState {
        eventfd: -1,
        ready: Mutex::new(Vec::new()),
        signaled: AtomicBool::new(false),
        shutdown: AtomicBool::new(false),
    });
    let slot: UringConnSlot = UringConnSlot {
        fd: server.as_raw_fd(),
        read_buf: vec![0; URING_IO_BUF_SIZE].into_boxed_slice(),
        read_pos: 0,
        read_len: 0,
        write_buf: vec![0; URING_IO_BUF_SIZE].into_boxed_slice(),
        write_pos: 0,
        write_len: 0,
        pending_write: Vec::new(),
        send_staged: false,
        recv_inflight: false,
        send_inflight: false,
        close_inflight: false,
        flush_pending: false,
        eof: false,
        write_shutdown: false,
        closing: false,
        read_err: None,
        write_err: None,
        waker: Waker::noop().clone(),
        task: None,
        in_local_ready: false,
    };
    let reactor: UringReactor = UringReactor {
        ring,
        pending_submissions: VecDeque::new(),
        slots: vec![Some(slot)],
        free_slots: Vec::new(),
        buf_pool: Vec::new(),
        waker_cache: vec![None],
        local_ready: Vec::new(),
        wake_state,
        active_slots: 1,
    };
    URING_REACTOR.with(|cell: &RefCell<Option<UringReactor>>| {
        assert!(cell.borrow().is_none());
        *cell.borrow_mut() = Some(reactor);
    });
    let mut transport: UringTcpTransport = UringTcpTransport { slot_id: 0, peers };
    let outcome: Result<(), Box<dyn Any + Send>> =
        catch_unwind(AssertUnwindSafe(|| check(&mut transport)));
    let reactor: Option<UringReactor> =
        URING_REACTOR.with(|cell: &RefCell<Option<UringReactor>>| cell.borrow_mut().take());
    drop(reactor);
    drop(server);
    drop(client);
    drop(listener);
    if let Err(panic) = outcome {
        resume_unwind(panic);
    }
    true
}

#[test]
fn uring_flush_without_a_reactor_reports_broken_pipe() {
    let address: SocketAddr = SocketAddr::from((Ipv4Addr::LOCALHOST, 0));
    let mut transport: UringTcpTransport = UringTcpTransport {
        slot_id: 0,
        peers: StreamPeers {
            local: address,
            remote: address,
        },
    };
    let mut context: task::Context<'_> = task::Context::from_waker(Waker::noop());
    let outcome: Poll<io::Result<()>> = Pin::new(&mut transport).poll_flush(&mut context);
    assert!(
        matches!(outcome, Poll::Ready(Err(error)) if error.kind() == io::ErrorKind::BrokenPipe)
    );
}

#[test]
fn uring_flush_waits_for_an_inflight_send() {
    let exercised: bool = with_uring_slot(|transport: &mut UringTcpTransport| {
        URING_REACTOR.with(|cell: &RefCell<Option<UringReactor>>| {
            let mut guard: RefMut<'_, Option<UringReactor>> = cell.borrow_mut();
            let slot: &mut UringConnSlot = guard.as_mut().unwrap().slots[0].as_mut().unwrap();
            slot.write_len = 4;
            slot.send_inflight = true;
        });
        let mut context: task::Context<'_> = task::Context::from_waker(Waker::noop());
        assert!(Pin::new(transport).poll_flush(&mut context).is_pending());
    });
    if !exercised {
        assert!(!uring_is_available());
    }
}

#[test]
fn uring_flush_propagates_a_completed_send_error() {
    let exercised: bool = with_uring_slot(|transport: &mut UringTcpTransport| {
        URING_REACTOR.with(|cell: &RefCell<Option<UringReactor>>| {
            let mut guard: RefMut<'_, Option<UringReactor>> = cell.borrow_mut();
            let slot: &mut UringConnSlot = guard.as_mut().unwrap().slots[0].as_mut().unwrap();
            slot.write_err = Some(EPIPE);
        });
        let mut context: task::Context<'_> = task::Context::from_waker(Waker::noop());
        let outcome: Poll<io::Result<()>> = Pin::new(transport).poll_flush(&mut context);
        assert!(matches!(outcome, Poll::Ready(Err(error)) if error.raw_os_error() == Some(EPIPE)));
    });
    if !exercised {
        assert!(!uring_is_available());
    }
}

#[test]
fn uring_linked_close_retains_the_fd_until_completion() {
    let exercised: bool = with_uring_slot(|_: &mut UringTcpTransport| {
        URING_REACTOR.with(|cell: &RefCell<Option<UringReactor>>| {
            let mut guard: RefMut<'_, Option<UringReactor>> = cell.borrow_mut();
            let reactor: &mut UringReactor = guard.as_mut().unwrap();
            let slot: &mut UringConnSlot = reactor.slots[0].as_mut().unwrap();
            let fd: RawFd = slot.fd;
            slot.write_buf[..4].copy_from_slice(b"test");
            slot.write_len = 4;
            slot.send_staged = true;
            slot.closing = true;
            reactor.flush_staged_send(0, true);
            assert_eq!(reactor.slots[0].as_ref().unwrap().fd, fd);
            assert!(reactor.slots[0].as_ref().unwrap().close_inflight);
        });
    });
    if !exercised {
        assert!(!uring_is_available());
    }
}

#[test]
fn uring_partial_linked_send_survives_a_cancelled_close() {
    let exercised: bool = with_uring_slot(|_: &mut UringTcpTransport| {
        URING_REACTOR.with(|cell: &RefCell<Option<UringReactor>>| {
            let mut guard: RefMut<'_, Option<UringReactor>> = cell.borrow_mut();
            let reactor: &mut UringReactor = guard.as_mut().unwrap();
            let slot: &mut UringConnSlot = reactor.slots[0].as_mut().unwrap();
            let fd: RawFd = slot.fd;
            slot.write_buf[..8].copy_from_slice(b"abcdefgh");
            slot.write_len = 8;
            slot.send_inflight = true;
            slot.close_inflight = true;
            slot.closing = true;
            reactor.complete_send(0, 3);
            assert_eq!(reactor.slots[0].as_ref().unwrap().write_pos, 3);
            assert_eq!(reactor.slots[0].as_ref().unwrap().write_len, 8);
            assert!(reactor.free_slots.is_empty());
            reactor.complete_close(0, -ECANCELED);
            let slot: &UringConnSlot = reactor.slots[0].as_ref().unwrap();
            assert_eq!(slot.fd, fd);
            assert_eq!(slot.write_pos, 3);
            assert!(slot.send_inflight && slot.close_inflight);
            assert_eq!(reactor.active_slots, 1);
            assert!(reactor.free_slots.is_empty());
        });
    });
    if !exercised {
        assert!(!uring_is_available());
    }
}

#[test]
fn uring_failed_linked_send_retries_only_the_cancelled_close() {
    let exercised: bool = with_uring_slot(|_: &mut UringTcpTransport| {
        URING_REACTOR.with(|cell: &RefCell<Option<UringReactor>>| {
            let mut guard: RefMut<'_, Option<UringReactor>> = cell.borrow_mut();
            let reactor: &mut UringReactor = guard.as_mut().unwrap();
            let slot: &mut UringConnSlot = reactor.slots[0].as_mut().unwrap();
            let fd: RawFd = slot.fd;
            slot.write_len = 4;
            slot.send_inflight = true;
            slot.close_inflight = true;
            slot.closing = true;
            reactor.complete_send(0, -EPIPE);
            reactor.complete_close(0, -ECANCELED);
            let slot: &UringConnSlot = reactor.slots[0].as_ref().unwrap();
            assert_eq!(slot.fd, fd);
            assert_eq!(slot.write_err, Some(EPIPE));
            assert!(!slot.send_inflight);
            assert!(slot.close_inflight);
            assert_eq!(reactor.active_slots, 1);
        });
    });
    if !exercised {
        assert!(!uring_is_available());
    }
}

#[test]
fn uring_full_sq_retains_a_complete_linked_group() {
    let exercised: bool = with_uring_slot(|_: &mut UringTcpTransport| {
        URING_REACTOR.with(|cell: &RefCell<Option<UringReactor>>| {
            let mut guard: RefMut<'_, Option<UringReactor>> = cell.borrow_mut();
            let reactor: &mut UringReactor = guard.as_mut().unwrap();
            let capacity: usize = reactor.ring.submission().capacity();
            for _ in 0..capacity - 1 {
                reactor.push_sqe(opcode::Nop::new().build());
            }
            let slot: &mut UringConnSlot = reactor.slots[0].as_mut().unwrap();
            slot.write_len = 4;
            slot.send_staged = true;
            slot.closing = true;
            reactor.flush_staged_send(0, true);
            assert_eq!(reactor.ring.submission().len(), capacity - 1);
            assert_eq!(reactor.pending_submissions.len(), 1);
            assert!(matches!(
                reactor.pending_submissions.front(),
                Some(UringSubmission::Linked(_))
            ));
        });
    });
    if !exercised {
        assert!(!uring_is_available());
    }
}

#[test]
fn uring_full_sq_does_not_lose_a_standalone_operation() {
    let exercised: bool = with_uring_slot(|_: &mut UringTcpTransport| {
        URING_REACTOR.with(|cell: &RefCell<Option<UringReactor>>| {
            let mut guard: RefMut<'_, Option<UringReactor>> = cell.borrow_mut();
            let reactor: &mut UringReactor = guard.as_mut().unwrap();
            let capacity: usize = reactor.ring.submission().capacity();
            for _ in 0..capacity + 1 {
                reactor.push_sqe(opcode::Nop::new().build());
            }
            assert_eq!(reactor.pending_submissions.len(), 1);
            let mut completed: usize = 0;
            while completed < capacity + 1 {
                reactor.submit_io().unwrap();
                completed += reactor.ring.completion().count();
            }
            assert!(reactor.pending_submissions.is_empty());
            assert_eq!(completed, capacity + 1);
        });
    });
    if !exercised {
        assert!(!uring_is_available());
    }
}

#[test]
fn uring_shutdown_propagates_a_completed_send_error() {
    let exercised: bool = with_uring_slot(|transport: &mut UringTcpTransport| {
        URING_REACTOR.with(|cell: &RefCell<Option<UringReactor>>| {
            let mut guard: RefMut<'_, Option<UringReactor>> = cell.borrow_mut();
            let slot: &mut UringConnSlot = guard.as_mut().unwrap().slots[0].as_mut().unwrap();
            slot.write_err = Some(EPIPE);
        });
        let mut context: task::Context<'_> = task::Context::from_waker(Waker::noop());
        let outcome: Poll<io::Result<()>> = Pin::new(transport).poll_shutdown(&mut context);
        assert!(matches!(outcome, Poll::Ready(Err(error)) if error.raw_os_error() == Some(EPIPE)));
    });
    if !exercised {
        assert!(!uring_is_available());
    }
}
