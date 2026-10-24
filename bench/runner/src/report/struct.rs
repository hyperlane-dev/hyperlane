use super::*;

/// The load generator result fields used by the report.
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct MeasurementResult {
    /// The number of failed requests.
    pub(crate) errors: u64,
    /// The achieved requests per second.
    pub(crate) rps: f64,
    /// The median latency in microseconds.
    pub(crate) p50_us: u64,
    /// The 99th percentile latency in microseconds.
    pub(crate) p99_us: u64,
}

/// One raw measurement row.
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct MeasurementRow {
    /// The variant that produced the row.
    pub(crate) variant: String,
    /// The protocol used by the row.
    pub(crate) protocol: String,
    /// The connection mode used by the row.
    pub(crate) mode: String,
    /// The concurrency level of the row.
    pub(crate) connections: u32,
    /// The repeat number of the row.
    pub(crate) repeat: u32,
    /// The load generator result.
    pub(crate) result: MeasurementResult,
}

/// The key of one workload matrix cell.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct Cell {
    /// The protocol axis.
    pub(crate) protocol: String,
    /// The connection mode axis.
    pub(crate) mode: String,
    /// The concurrency axis.
    pub(crate) connections: u32,
}
