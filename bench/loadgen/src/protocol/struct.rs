use super::*;

/// One request of the mix the generator drives.
///
/// The mix carries the shapes a real service answers: a static route, a
/// route with a path parameter, a route with a query, a route that reads a
/// header, and a route that echoes the body.
pub(crate) struct ProtocolCase {
    /// The request line and headers, ready to write.
    pub(crate) bytes: Vec<u8>,
    /// The body the request carries.
    pub(crate) body: Vec<u8>,
    /// The length of the answer the request expects.
    pub(crate) expect_len: usize,
}

/// The parameters every driver of the measurement matrix shares.
///
/// The drivers take one of these instead of a long parameter list, so the
/// matrix can grow a dimension without every driver growing a parameter.
pub(crate) struct Run {
    /// The address of the server under test.
    pub(crate) address: String,
    /// The request mix.
    pub(crate) cases: Vec<ProtocolCase>,
    /// The barrier that starts every connection together.
    pub(crate) barrier: Arc<Barrier>,
    /// The moment the warm-up window ends.
    pub(crate) warmup_until: Instant,
    /// The moment the run ends.
    pub(crate) deadline: Instant,
    /// The wire protocol the connection speaks.
    pub(crate) protocol: Protocol,
    /// The path of the certificate authority, for the secure protocols.
    pub(crate) ca_path: Option<String>,
    /// The ceiling on the number of connections opened per second.
    pub(crate) rate_per_sec: u64,
    /// The number of connections the generator opens.
    pub(crate) connections: usize,
}
