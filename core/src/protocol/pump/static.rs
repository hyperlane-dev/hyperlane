use super::*;

thread_local! {
    /// Thread-local pool of reusable request contexts.
    ///
    /// Multiplexed protocol tasks and short connections each run one request
    /// lifecycle and then finish. Recycling the context retains the backing
    /// allocations of its request, response and route-parameter maps across
    /// requests on the same worker thread.
    pub(crate) static CONTEXT_POOL: RefCell<Vec<Context>> = const { RefCell::new(Vec::new()) };
    /// Thread-local pool of recycled protocol request header entries.
    ///
    /// Keeps the name and value list buffers alive across HTTP/2 and HTTP/3
    /// requests so parsing decoded protocol headers avoids per-header heap
    /// allocations.
    pub(crate) static PROTOCOL_HEADER_POOL: RefCell<Vec<(String, VecDeque<String>)>> =
        const { RefCell::new(Vec::new()) };
    /// Thread-local pool of reusable request target strings.
    pub(crate) static TARGET_BUFFER_POOL: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
    /// Thread-local pool of reusable route parameter key/value string pairs.
    pub(crate) static ROUTE_PARAM_POOL: RefCell<Vec<(String, String)>> =
        const { RefCell::new(Vec::new()) };
}
