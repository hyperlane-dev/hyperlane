use super::*;

thread_local! {
    /// Thread-local pool of reusable read buffers for `PooledReader`.
    pub(crate) static READ_BUFFER_POOL: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
    /// Thread-local pool of recycled header entries.
    ///
    /// A header entry is a name and the list of values that were sent under
    /// it. Both the name and the values own their buffers, so an entry that
    /// is recycled keeps them: the next request that needs a header reuses
    /// the buffers instead of allocating new ones.
    ///
    /// The pool is what keeps a request from allocating a fresh name, a fresh
    /// value list and a fresh value for every header it carries. Without it
    /// every request allocates and frees dozens of small blocks, and those
    /// frees are what serialise the workers of a multi threaded runtime
    /// against each other inside the allocator.
    pub(crate) static HEADER_ENTRY_POOL: RefCell<Vec<(String, VecDeque<String>)>> =
        const { RefCell::new(Vec::new()) };
    /// Thread-local pool of recycled strings.
    ///
    /// Used for the short lived strings a response builds, such as the
    /// content length it announces.
    pub(crate) static STRING_POOL: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}
