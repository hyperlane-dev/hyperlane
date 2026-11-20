use super::*;

impl Drop for UringDropProbe {
    fn drop(&mut self) {
        let Self { state } = self;
        let available: bool =
            URING_REACTOR.with(|cell: &RefCell<Option<UringReactor>>| cell.try_borrow().is_ok());
        state
            .borrow_available
            .store(available, AtomicOrdering::Release);
        state.dropped.store(true, AtomicOrdering::Release);
    }
}

impl ServerHook for UringPendingDropRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        let state: Arc<UringDropState> =
            Arc::clone(URING_DROP_STATE.get_or_init(|| Arc::new(UringDropState::default())));
        Self {
            probe: UringDropProbe { state },
        }
    }

    async fn handle(self, _: &mut Stream, _: &mut Context) -> Status {
        let Self { probe } = self;
        let UringDropProbe { state } = &probe;
        let entered: bool =
            URING_REACTOR.with(|cell: &RefCell<Option<UringReactor>>| cell.borrow().is_some());
        state.entered_uring.store(entered, AtomicOrdering::Release);
        state.started.notify_one();
        pending::<()>().await;
        drop(probe);
        Status::Reject
    }
}
