/// The certificate authority file name.
pub(crate) const CA_FILE_NAME: &str = "ca.pem";
/// The leaf certificate file name.
pub(crate) const LEAF_CERT_FILE_NAME: &str = "leaf.pem";
/// The leaf private key file name.
pub(crate) const LEAF_KEY_FILE_NAME: &str = "leaf.key";
/// The default certificate directory name.
pub(crate) const DEFAULT_CERTS_DIR: &str = "certs";
/// The default results directory name.
pub(crate) const DEFAULT_RESULTS_DIR: &str = "results";
/// The default Docker network name.
pub(crate) const DEFAULT_NETWORK: &str = "hyperlane-perf";
/// The default Docker image prefix.
pub(crate) const DEFAULT_IMAGE_PREFIX: &str = "hyperlane-perf";
/// The default server CPU quota.
pub(crate) const DEFAULT_SERVER_CPUS: &str = "4";
/// The default load generator CPU quota.
pub(crate) const DEFAULT_LOADGEN_CPUS: &str = "4";
/// The default container memory limit.
pub(crate) const DEFAULT_MEMORY: &str = "4g";
/// The default keep-alive concurrency levels.
pub(crate) const DEFAULT_CONNECTIONS: &str = "64 256";
/// The default short-connection concurrency levels.
pub(crate) const DEFAULT_CONNECTIONS_SHORT: &str = "16 64";
/// The default measured window in seconds.
pub(crate) const DEFAULT_DURATION: &str = "12";
/// The default warm-up window in seconds.
pub(crate) const DEFAULT_WARMUP: &str = "4";
/// The default repeat count.
pub(crate) const DEFAULT_REPEATS: &str = "3";
/// The default protocol matrix.
pub(crate) const DEFAULT_PROTOCOLS: &str = "h1 h1-tls h2c h2-tls h3";
/// The default connection mode matrix.
pub(crate) const DEFAULT_MODES: &str = "keepalive short";
/// The default cooldown between short repeats.
pub(crate) const DEFAULT_COOLDOWN_SHORT: &str = "20";
/// The default short-connection rate cap.
pub(crate) const DEFAULT_RATE_SHORT: &str = "200";
/// The results directory environment variable.
pub(crate) const ENV_RESULTS: &str = "PERF_RESULTS";
/// The Docker network environment variable.
pub(crate) const ENV_NETWORK: &str = "PERF_NETWORK";
/// The image prefix environment variable.
pub(crate) const ENV_IMAGE_PREFIX: &str = "PERF_IMAGE_PREFIX";
/// The server CPU quota environment variable.
pub(crate) const ENV_SERVER_CPUS: &str = "PERF_SERVER_CPUS";
/// The load generator CPU quota environment variable.
pub(crate) const ENV_LOADGEN_CPUS: &str = "PERF_LOADGEN_CPUS";
/// The container memory environment variable.
pub(crate) const ENV_MEMORY: &str = "PERF_MEMORY";
/// The keep-alive concurrency environment variable.
pub(crate) const ENV_CONNECTIONS: &str = "PERF_CONNECTIONS";
/// The short-connection concurrency environment variable.
pub(crate) const ENV_CONNECTIONS_SHORT: &str = "PERF_CONNECTIONS_SHORT";
/// The measured window environment variable.
pub(crate) const ENV_DURATION: &str = "PERF_DURATION";
/// The warm-up window environment variable.
pub(crate) const ENV_WARMUP: &str = "PERF_WARMUP";
/// The repeat count environment variable.
pub(crate) const ENV_REPEATS: &str = "PERF_REPEATS";
/// The protocol matrix environment variable.
pub(crate) const ENV_PROTOCOLS: &str = "PERF_PROTOCOLS";
/// The connection mode matrix environment variable.
pub(crate) const ENV_MODES: &str = "PERF_MODES";
/// The short-repeat cooldown environment variable.
pub(crate) const ENV_COOLDOWN_SHORT: &str = "PERF_COOLDOWN_SHORT";
/// The short-connection rate environment variable.
pub(crate) const ENV_RATE_SHORT: &str = "PERF_RATE_SHORT";
/// The certificate directory environment variable.
pub(crate) const ENV_CERTS: &str = "PERF_CERTS";
/// The certificate authority environment variable.
pub(crate) const ENV_CA_CERT: &str = "PERF_CA_CERT";
/// The leaf certificate environment variable.
pub(crate) const ENV_TLS_CERT: &str = "PERF_TLS_CERT";
/// The leaf key environment variable.
pub(crate) const ENV_TLS_KEY: &str = "PERF_TLS_KEY";
