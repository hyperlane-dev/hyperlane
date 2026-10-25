use super::*;

/// The frozen configuration of one runner invocation.
pub(crate) struct Config {
    /// The benchmark harness root.
    pub(crate) root: PathBuf,
    /// The directory that receives measurements.
    pub(crate) results: PathBuf,
    /// The Docker network name.
    pub(crate) network: String,
    /// The Docker image prefix.
    pub(crate) image_prefix: String,
    /// The server CPU quota.
    pub(crate) server_cpus: String,
    /// The load generator CPU quota.
    pub(crate) loadgen_cpus: String,
    /// The memory limit of each container.
    pub(crate) memory: String,
    /// The keep-alive concurrency levels.
    pub(crate) connections: Vec<u32>,
    /// The short-connection concurrency levels.
    pub(crate) connections_short: Vec<u32>,
    /// The measured window in seconds.
    pub(crate) duration: String,
    /// The warm-up window in seconds.
    pub(crate) warmup: String,
    /// The number of repeats per cell.
    pub(crate) repeats: u32,
    /// The protocols to measure.
    pub(crate) protocols: Vec<String>,
    /// The connection modes to measure.
    pub(crate) modes: Vec<String>,
    /// The cooldown between short repeats.
    pub(crate) cooldown_short: u64,
    /// The short-connection rate cap.
    pub(crate) rate_short: String,
    /// The certificate directory.
    pub(crate) certs: PathBuf,
    /// The certificate authority path.
    pub(crate) ca_cert: PathBuf,
    /// The leaf certificate path.
    pub(crate) tls_certificate: PathBuf,
    /// The leaf private key path.
    pub(crate) tls_key: PathBuf,
}
