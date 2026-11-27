use super::*;

/// Aggregate or individual probe health state.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    /// Service or dependency is healthy and accepting traffic.
    #[default]
    Up,
    /// Service is operational with non-fatal degradation.
    Degraded,
    /// Service or dependency is unhealthy (`503 Service Unavailable`).
    Down,
}
