/// The environment variable that names the address to bind.
pub(crate) const ENV_ADDRESS: &str = "BENCH_ADDR";

/// The address the server binds when the environment names none.
pub(crate) const DEFAULT_ADDRESS: &str = "0.0.0.0:8080";

/// The size of the buffer a connection reads into.
pub(crate) const READ_BUFFER_SIZE: usize = 16 * 1024;
