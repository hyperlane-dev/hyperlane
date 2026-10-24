/// The Docker executable.
pub(crate) const DOCKER_PROGRAM: &str = "docker";
/// The Cargo executable.
pub(crate) const CARGO_PROGRAM: &str = "cargo";
/// The Dockerfile path within the harness.
pub(crate) const DOCKERFILE_PATH: &str = "docker/Dockerfile.perf";
/// The library manifest name.
pub(crate) const LIBRARY_MANIFEST: &str = "Cargo.toml";
/// The harness directory name.
pub(crate) const HARNESS_DIR: &str = "bench";
/// The raw measurement file name.
pub(crate) const RAW_FILE_NAME: &str = "raw.jsonl";
/// The server readiness marker.
pub(crate) const READY_MARKER: &str = "ready on";
/// The instrument image tag.
pub(crate) const INSTRUMENT_TAG: &str = "1.0";
/// The server image label.
pub(crate) const SERVER_IMAGE: &str = "server";
/// The load generator image label.
pub(crate) const LOADGEN_IMAGE: &str = "loadgen";
/// The control image label.
pub(crate) const CONTROL_IMAGE: &str = "control";
/// The server runtime target.
pub(crate) const SERVER_RUNTIME: &str = "server-runtime";
/// The load generator runtime target.
pub(crate) const LOADGEN_RUNTIME: &str = "loadgen-runtime";
/// The control runtime target.
pub(crate) const CONTROL_RUNTIME: &str = "control-runtime";
/// The control variant name.
pub(crate) const CONTROL_VARIANT: &str = "control";
/// The short connection mode.
pub(crate) const SHORT_MODE: &str = "short";
/// The HTTP/1 protocol variant.
pub(crate) const H1_PROTOCOL: &str = "h1";
/// The HTTP/1 TLS protocol variant.
pub(crate) const H1_TLS_PROTOCOL: &str = "h1-tls";
/// The cleartext HTTP/2 protocol variant.
pub(crate) const H2C_PROTOCOL: &str = "h2c";
/// The HTTP/2 TLS protocol variant.
pub(crate) const H2_TLS_PROTOCOL: &str = "h2-tls";
/// The HTTP/3 protocol variant.
pub(crate) const H3_PROTOCOL: &str = "h3";
/// The server HTTP/1 protocol name.
pub(crate) const SERVER_H1_PROTOCOL: &str = "http1";
/// The server HTTP/1 TLS protocol name.
pub(crate) const SERVER_H1_TLS_PROTOCOL: &str = "http1-tls";
/// The server HTTP/2 protocol name.
pub(crate) const SERVER_H2_PROTOCOL: &str = "http2";
/// The server HTTP/2 TLS protocol name.
pub(crate) const SERVER_H2_TLS_PROTOCOL: &str = "http2-tls";
/// The server HTTP/3 protocol name.
pub(crate) const SERVER_H3_PROTOCOL: &str = "http3";
/// The instrument subcommand.
pub(crate) const INSTRUMENT_COMMAND: &str = "instrument";
/// The build subcommand.
pub(crate) const BUILD_COMMAND: &str = "build";
/// The measurement subcommand.
pub(crate) const BENCH_COMMAND: &str = "bench";
/// The report subcommand.
pub(crate) const REPORT_COMMAND: &str = "report";
/// The certificate generation subcommand.
pub(crate) const CERTIFICATES_COMMAND: &str = "certificates";
/// The command line usage.
pub(crate) const USAGE: &str = "usage: bench-perf <instrument|build|bench|report|certificates> ...";
/// The Docker build subcommand.
pub(crate) const DOCKER_BUILD: &str = "build";
/// The Docker run subcommand.
pub(crate) const DOCKER_RUN: &str = "run";
/// The Docker remove subcommand.
pub(crate) const DOCKER_REMOVE: &str = "rm";
/// The Docker logs subcommand.
pub(crate) const DOCKER_LOGS: &str = "logs";
/// The Docker inspect subcommand.
pub(crate) const DOCKER_INSPECT: &str = "inspect";
/// The Docker network subcommand.
pub(crate) const DOCKER_NETWORK: &str = "network";
/// The Docker create subcommand.
pub(crate) const DOCKER_CREATE: &str = "create";
/// The Docker image subcommand.
pub(crate) const DOCKER_IMAGE: &str = "image";
/// The Docker process list subcommand.
pub(crate) const DOCKER_PS: &str = "ps";
/// The Docker target flag.
pub(crate) const TARGET_FLAG: &str = "--target";
/// The Docker name flag.
pub(crate) const NAME_FLAG: &str = "--name";
/// The Docker network flag.
pub(crate) const NETWORK_FLAG: &str = "--network";
/// The Docker CPU quota flag.
pub(crate) const CPUS_FLAG: &str = "--cpus";
/// The Docker memory flag.
pub(crate) const MEMORY_FLAG: &str = "--memory";
/// The Docker automatic removal flag.
pub(crate) const REMOVE_AFTER_RUN_FLAG: &str = "--rm";
/// The Docker format flag.
pub(crate) const FORMAT_FLAG: &str = "--format";
/// The Docker image ID format.
pub(crate) const IMAGE_ID_FORMAT: &str = "{{.Id}}";
/// The Docker list filter flag.
pub(crate) const FILTER_FLAG: &str = "--filter";
/// The benchmark container name filter.
pub(crate) const PERF_NAME_FILTER: &str = "name=perf-";
/// The Docker all and quiet list flags.
pub(crate) const PS_ALL_QUIET_FLAG: &str = "-aq";
/// The Cargo update subcommand.
pub(crate) const CARGO_UPDATE: &str = "update";
/// The Cargo workspace flag.
pub(crate) const WORKSPACE_FLAG: &str = "--workspace";
/// The Cargo offline flag.
pub(crate) const OFFLINE_FLAG: &str = "--offline";
/// The Cargo manifest path flag.
pub(crate) const MANIFEST_PATH_FLAG: &str = "--manifest-path";
/// The server listen address environment setting.
pub(crate) const SERVER_ADDRESS_ENV: &str = "BENCH_ADDR=0.0.0.0:8080";
/// The server protocol environment prefix.
pub(crate) const SERVER_PROTOCOL_ENV: &str = "BENCH_PROTOCOL=";
/// The server certificate environment setting.
pub(crate) const SERVER_CERT_ENV: &str = "BENCH_TLS_CERT=/certs/leaf.pem";
/// The server key environment setting.
pub(crate) const SERVER_KEY_ENV: &str = "BENCH_TLS_KEY=/certs/leaf.key";
/// The container certificate authority path.
pub(crate) const CONTAINER_CA_PATH: &str = "/certs/ca.pem";
/// The load generator address flag.
pub(crate) const ADDR_FLAG: &str = "--addr";
/// The load generator connections flag.
pub(crate) const CONNECTIONS_FLAG: &str = "--connections";
/// The load generator duration flag.
pub(crate) const DURATION_FLAG: &str = "--duration";
/// The load generator warm-up flag.
pub(crate) const WARMUP_FLAG: &str = "--warmup";
/// The load generator protocol flag.
pub(crate) const PROTOCOL_FLAG: &str = "--protocol";
/// The load generator mode flag.
pub(crate) const MODE_FLAG: &str = "--mode";
/// The load generator CA flag.
pub(crate) const CA_FLAG: &str = "--ca";
/// The load generator rate flag.
pub(crate) const RATE_FLAG: &str = "--rate";
/// The server container prefix.
pub(crate) const SERVER_CONTAINER_PREFIX: &str = "perf-server-";
/// The load generator container prefix.
pub(crate) const LOADGEN_CONTAINER_PREFIX: &str = "perf-loadgen-";
/// The first arm output prefix.
pub(crate) const FIRST_OUTPUT_PREFIX: &str = "first-";
/// The second arm output prefix.
pub(crate) const SECOND_OUTPUT_PREFIX: &str = "second-";
/// The readiness poll count.
pub(crate) const READY_ATTEMPTS: u32 = 300;
/// The readiness poll delay in milliseconds.
pub(crate) const READY_DELAY_MS: u64 = 200;
/// The container port.
pub(crate) const CONTAINER_PORT: u32 = 8080;
