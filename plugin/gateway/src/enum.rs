/// Upstream selection algorithm used by a [`crate::GatewayRoute`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LoadBalanceStrategy {
    /// Cycles evenly through healthy upstreams.
    #[default]
    RoundRobin,
    /// Selects the healthy upstream with the fewest active in-flight requests.
    LeastConnections,
    /// Distributes requests proportionally to each [`crate::UpstreamTarget`]'s weight.
    WeightedRoundRobin,
}
