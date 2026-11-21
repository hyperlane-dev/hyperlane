use super::*;

thread_local! {
    /// Worker-local Linux `io_uring` reactor accessed by [`UringTcpTransport`] during polls.
    pub(crate) static URING_REACTOR: RefCell<Option<UringReactor>> = const { RefCell::new(None) };
}
