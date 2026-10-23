//! bench-loadgen
//!
//! A closed-loop HTTP/1.1 load generator for the hyperlane benchmark harness.
mod r#const;

pub(crate) use r#const::*;

use std::{
    io::Error,
    sync::Arc,
    time::{Duration, Instant},
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::{Barrier, BarrierWaitResult},
    task::JoinHandle,
};

/// The configuration of one generator run.
#[derive(Clone, Debug)]
pub struct Config {
    /// The address of the server under test.
    address: String,
    /// The number of keep-alive connections the generator opens.
    connections: usize,
    /// The length of the measured window, in seconds.
    duration_secs: u64,
    /// The length of the warm-up window, in seconds.
    warmup_secs: u64,
}

impl Config {
    /// The address of the server under test.
    ///
    /// # Returns
    ///
    /// - `&String` - The address.
    pub fn get_address(&self) -> &String {
        &self.address
    }

    /// The number of keep-alive connections the generator opens.
    ///
    /// # Returns
    ///
    /// - `usize` - The number of connections.
    pub fn get_connections(&self) -> usize {
        self.connections
    }

    /// The length of the measured window, in seconds.
    ///
    /// # Returns
    ///
    /// - `u64` - The length of the window.
    pub fn get_duration_secs(&self) -> u64 {
        self.duration_secs
    }

    /// The length of the warm-up window, in seconds.
    ///
    /// # Returns
    ///
    /// - `u64` - The length of the window.
    pub fn get_warmup_secs(&self) -> u64 {
        self.warmup_secs
    }

    /// Sets the address of the server under test.
    ///
    /// # Arguments
    ///
    /// - `String` - The address.
    fn set_address(&mut self, address: String) {
        self.address = address;
    }

    /// Sets the number of keep-alive connections the generator opens.
    ///
    /// # Arguments
    ///
    /// - `usize` - The number of connections.
    fn set_connections(&mut self, connections: usize) {
        self.connections = connections;
    }

    /// Sets the length of the measured window.
    ///
    /// # Arguments
    ///
    /// - `u64` - The length of the window, in seconds.
    fn set_duration_secs(&mut self, duration_secs: u64) {
        self.duration_secs = duration_secs;
    }

    /// Sets the length of the warm-up window.
    ///
    /// # Arguments
    ///
    /// - `u64` - The length of the window, in seconds.
    fn set_warmup_secs(&mut self, warmup_secs: u64) {
        self.warmup_secs = warmup_secs;
    }
}

impl Default for Config {
    /// Builds the configuration the generator uses when nothing is given.
    ///
    /// # Returns
    ///
    /// - `Self` - The default configuration.
    fn default() -> Self {
        Self {
            address: DEFAULT_ADDR.to_owned(),
            connections: DEFAULT_CONNECTIONS,
            duration_secs: DEFAULT_DURATION_SECS,
            warmup_secs: DEFAULT_WARMUP_SECS,
        }
    }
}

impl Config {
    /// Reads the configuration from the command line arguments.
    ///
    /// # Arguments
    ///
    /// - `&[String]` - The arguments, without the program name.
    ///
    /// # Returns
    ///
    /// - `Result<Config, String>` - The configuration, or the reason it is not
    ///   usable.
    pub fn from_args(args: &[String]) -> Result<Config, String> {
        let mut config: Config = Config::default();
        let mut index: usize = 0;
        while index < args.len() {
            let flag: &str = args[index].as_str();
            let value: Option<&str> = match args.get(index + 1) {
                Some(value) => Some(value.as_str()),
                None => None,
            };
            match flag {
                ARG_ADDR => {
                    let Some(value) = value else {
                        return Err(format!("{ARG_ADDR} needs a value"));
                    };
                    config.set_address(value.to_owned());
                    index += 2;
                }
                ARG_CONNECTIONS => {
                    let Some(value) = value else {
                        return Err(format!("{ARG_CONNECTIONS} needs a value"));
                    };
                    let connections: usize = match value.parse::<usize>() {
                        Ok(connections) => connections,
                        Err(error) => return Err(error.to_string()),
                    };
                    config.set_connections(connections);
                    index += 2;
                }
                ARG_DURATION => {
                    let Some(value) = value else {
                        return Err(format!("{ARG_DURATION} needs a value"));
                    };
                    let duration: u64 = match value.parse::<u64>() {
                        Ok(duration) => duration,
                        Err(error) => return Err(error.to_string()),
                    };
                    config.set_duration_secs(duration);
                    index += 2;
                }
                ARG_WARMUP => {
                    let Some(value) = value else {
                        return Err(format!("{ARG_WARMUP} needs a value"));
                    };
                    let warmup: u64 = match value.parse::<u64>() {
                        Ok(warmup) => warmup,
                        Err(error) => return Err(error.to_string()),
                    };
                    config.set_warmup_secs(warmup);
                    index += 2;
                }
                other => {
                    return Err(format!("unknown argument {other}"));
                }
            }
        }
        Ok(config)
    }
}

/// The result of one generator run.
#[derive(Clone, Debug, Default)]
pub struct Outcome {
    /// The number of requests that completed with the expected body.
    completed: u64,
    /// The number of requests that failed.
    errors: u64,
    /// The latency of every completed request, in microseconds.
    latencies_us: Vec<u64>,
}

impl Outcome {
    /// The number of requests that completed with the expected body.
    ///
    /// # Returns
    ///
    /// - `u64` - The number of requests.
    fn get_completed(&self) -> u64 {
        self.completed
    }

    /// The number of requests that failed.
    ///
    /// # Returns
    ///
    /// - `u64` - The number of requests.
    fn get_errors(&self) -> u64 {
        self.errors
    }

    /// The latency of every completed request, in microseconds.
    ///
    /// # Returns
    ///
    /// - `&Vec<u64>` - The latencies.
    fn get_latencies_us(&self) -> &Vec<u64> {
        &self.latencies_us
    }

    /// The throughput the run reached, in requests per second.
    ///
    /// # Arguments
    ///
    /// - `&Config` - The configuration the run used.
    ///
    /// # Returns
    ///
    /// - `f64` - The throughput.
    fn rps(&self, config: &Config) -> f64 {
        if config.get_duration_secs() == 0 {
            return 0.0;
        }
        self.get_completed() as f64 / config.get_duration_secs() as f64
    }

    /// The mean latency of the run, in microseconds.
    ///
    /// # Returns
    ///
    /// - `f64` - The mean latency.
    fn mean_us(&self) -> f64 {
        let latencies: &Vec<u64> = self.get_latencies_us();
        if latencies.is_empty() {
            return 0.0;
        }
        let total: u64 = latencies.iter().sum();
        total as f64 / latencies.len() as f64
    }

    /// Renders the outcome as one line of JSON.
    ///
    /// # Arguments
    ///
    /// - `&Config` - The configuration the run used.
    ///
    /// # Returns
    ///
    /// - `String` - The JSON line.
    pub fn to_json(&self, config: &Config) -> String {
        let mut sorted: Vec<u64> = self.get_latencies_us().clone();
        sorted.sort_unstable();
        let max_us: u64 = match sorted.last() {
            Some(value) => *value,
            None => 0,
        };
        format!(
            "{{\"connections\":{},\"duration_s\":{},\"requests\":{},\"errors\":{},\"rps\":{:.1},\"p50_us\":{},\"p90_us\":{},\"p99_us\":{},\"p999_us\":{},\"max_us\":{},\"mean_us\":{:.1}}}",
            config.get_connections(),
            config.get_duration_secs(),
            self.get_completed(),
            self.get_errors(),
            self.rps(config),
            percentile(&sorted, 0.5),
            percentile(&sorted, 0.9),
            percentile(&sorted, 0.99),
            percentile(&sorted, 0.999),
            max_us,
            self.mean_us(),
        )
    }
}

/// Computes a percentile from a sorted slice of samples.
///
/// # Arguments
///
/// - `&[u64]` - The sorted samples.
/// - `f64` - The fraction of the slice, between zero and one.
///
/// # Returns
///
/// - `u64` - The sample at that fraction, interpolated between its neighbours.
#[inline(always)]
pub fn percentile(sorted: &[u64], fraction: f64) -> u64 {
    if sorted.is_empty() {
        return 0;
    }
    let position: f64 = fraction * (sorted.len() - 1) as f64;
    let lower: usize = position.floor() as usize;
    let upper: usize = position.ceil() as usize;
    let Some(lower_value) = sorted.get(lower) else {
        return 0;
    };
    if lower == upper {
        return *lower_value;
    }
    let Some(upper_value) = sorted.get(upper) else {
        return *lower_value;
    };
    let weight: f64 = position - lower as f64;
    let interpolated: f64 = *lower_value as f64 * (1.0 - weight) + *upper_value as f64 * weight;
    interpolated.round() as u64
}

/// Finds the first occurrence of a byte sequence in another.
///
/// # Arguments
///
/// - `&[u8]` - The haystack.
/// - `&[u8]` - The needle.
///
/// # Returns
///
/// - `Option<usize>` - The offset of the needle, when it is present.
#[inline(always)]
pub fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }
    haystack
        .windows(needle.len())
        .position(|window: &[u8]| window == needle)
}

/// Reports whether the line starts with the name, ignoring ASCII case.
///
/// # Arguments
///
/// - `&[u8]` - The line.
/// - `&[u8]` - The name.
///
/// # Returns
///
/// - `bool` - Whether the line starts with the name.
fn starts_with_name_ignore_case(line: &[u8], name: &[u8]) -> bool {
    if line.len() < name.len() {
        return false;
    }
    let Some(prefix) = line.get(..name.len()) else {
        return false;
    };
    for (index, byte) in prefix.iter().enumerate() {
        let Some(expected) = name.get(index) else {
            return false;
        };
        if !byte.eq_ignore_ascii_case(expected) {
            return false;
        }
    }
    true
}

/// Reads the announced body length out of a response head.
///
/// The head is scanned line by line, so a header whose name appears inside
/// another header's value is not mistaken for the length header.
///
/// # Arguments
///
/// - `&[u8]` - The head bytes, up to and including the blank line.
///
/// # Returns
///
/// - `usize` - The announced body length, or zero when the head carries none.
pub fn parse_content_length(head: &[u8]) -> usize {
    let mut index: usize = 0;
    while index < head.len() {
        let rest: &[u8] = &head[index..];
        if starts_with_name_ignore_case(rest, CONTENT_LENGTH_NAME)
            && let Some(length) = parse_content_length_line(rest)
        {
            return length;
        }
        match find_subsequence(rest, LINE_TERMINATOR) {
            Some(offset) => index += offset + LINE_TERMINATOR.len(),
            None => break,
        }
    }
    0
}

/// Reads the length header out of one line of a response head.
///
/// # Arguments
///
/// - `&[u8]` - The line, starting at its name.
///
/// # Returns
///
/// - `Option<usize>` - The announced length, when the line carries one.
fn parse_content_length_line(line: &[u8]) -> Option<usize> {
    let mut index: usize = CONTENT_LENGTH_NAME.len();
    if line.get(index) != Some(&b':') {
        return None;
    }
    index += 1;
    while matches!(line.get(index), Some(b' ') | Some(b'\t')) {
        index += 1;
    }
    let mut value: usize = 0;
    let mut digits: usize = 0;
    while let Some(byte) = line.get(index) {
        if !byte.is_ascii_digit() {
            break;
        }
        value = value * 10 + usize::from(*byte - b'0');
        digits += 1;
        index += 1;
    }
    if digits == 0 {
        return None;
    }
    Some(value)
}

/// One request in the mix.
#[derive(Clone, Debug)]
struct Case {
    /// The bytes to write.
    bytes: Vec<u8>,
    /// The body length the answer must have.
    expect_len: usize,
}

/// Builds the request mix.
///
/// The mix covers the route shapes the server under test exposes, weighted
/// towards the plain static route because that is what a typical workload hits
/// most.
///
/// # Returns
///
/// - `Vec<Case>` - The mix.
fn build_cases() -> Vec<Case> {
    let mut cases: Vec<Case> = Vec::new();
    for _ in 0..4 {
        cases.push(Case {
            bytes: REQUEST_ROOT.as_bytes().to_vec(),
            expect_len: EXPECT_ROOT,
        });
    }
    for _ in 0..2 {
        cases.push(Case {
            bytes: REQUEST_USER.as_bytes().to_vec(),
            expect_len: EXPECT_USER,
        });
    }
    cases.push(Case {
        bytes: REQUEST_SEARCH.as_bytes().to_vec(),
        expect_len: EXPECT_SEARCH,
    });
    cases.push(Case {
        bytes: REQUEST_HEADERS.as_bytes().to_vec(),
        expect_len: EXPECT_HEADERS,
    });
    cases.push(Case {
        bytes: REQUEST_ECHO.as_bytes().to_vec(),
        expect_len: EXPECT_ECHO,
    });
    cases
}

/// The counters of one connection.
#[derive(Clone, Debug, Default)]
struct ConnStats {
    /// The number of requests that completed with the expected body.
    completed: u64,
    /// The number of requests that failed.
    errors: u64,
    /// The latency of every completed request, in microseconds.
    latencies_us: Vec<u64>,
}

/// A connection that keeps its buffers between requests.
struct Connection {
    /// The socket the connection runs over.
    stream: TcpStream,
    /// The bytes read from the socket that the current response has not consumed.
    buffer: Vec<u8>,
    /// The offset of the first unconsumed byte in the buffer.
    start: usize,
    /// The offset one past the last valid byte in the buffer.
    end: usize,
    /// The scratch buffer a read lands in.
    chunk: Vec<u8>,
}

impl Connection {
    /// The socket the connection runs over.
    ///
    /// # Returns
    ///
    /// - `&mut TcpStream` - The socket.
    fn get_mut_stream(&mut self) -> &mut TcpStream {
        &mut self.stream
    }

    /// The bytes read from the socket that the current response has not consumed.
    ///
    /// # Returns
    ///
    /// - `&Vec<u8>` - The unconsumed bytes.
    fn get_buffer(&self) -> &Vec<u8> {
        &self.buffer
    }

    /// The offset of the first unconsumed byte in the buffer.
    ///
    /// # Returns
    ///
    /// - `usize` - The offset.
    fn get_start(&self) -> usize {
        self.start
    }

    /// The offset one past the last valid byte in the buffer.
    ///
    /// # Returns
    ///
    /// - `usize` - The offset.
    fn get_end(&self) -> usize {
        self.end
    }

    /// Sets the offset of the first unconsumed byte in the buffer.
    ///
    /// # Arguments
    ///
    /// - `usize` - The offset.
    fn set_start(&mut self, start: usize) {
        self.start = start;
    }

    /// Opens a connection to the server under test.
    ///
    /// # Arguments
    ///
    /// - `&str` - The address of the server.
    ///
    /// # Returns
    ///
    /// - `Result<Connection, String>` - The connection, or the reason it could
    ///   not be opened.
    async fn open(address: &str) -> Result<Connection, String> {
        let stream: TcpStream = match TcpStream::connect(address).await {
            Ok(stream) => stream,
            Err(error) => return Err(error.to_string()),
        };
        let _: Result<(), Error> = stream.set_nodelay(true);
        Ok(Connection {
            stream,
            buffer: Vec::with_capacity(READ_BUFFER_SIZE),
            start: 0,
            end: 0,
            chunk: vec![0; READ_BUFFER_SIZE],
        })
    }

    /// Reads more bytes from the socket into the buffer.
    ///
    /// # Returns
    ///
    /// - `Result<usize, String>` - The number of bytes read, or the reason the
    ///   read failed.
    async fn fill(&mut self) -> Result<usize, String> {
        // The fields are taken apart so the socket and the two buffers can be
        // borrowed at the same time.
        let Self {
            stream,
            buffer,
            chunk,
            start,
            end,
        } = self;
        let read: usize = match stream.read(chunk).await {
            Ok(read) => read,
            Err(error) => return Err(error.to_string()),
        };
        if read == 0 {
            return Err("the connection was closed".to_owned());
        }
        if *start == *end {
            buffer.clear();
            *start = 0;
            *end = 0;
        }
        buffer.extend_from_slice(&chunk[..read]);
        *end += read;
        Ok(read)
    }

    /// The bytes that have been read but not consumed.
    ///
    /// # Returns
    ///
    /// - `&[u8]` - The unconsumed bytes.
    #[inline(always)]
    fn available(&self) -> &[u8] {
        match self.get_buffer().get(self.get_start()..self.get_end()) {
            Some(bytes) => bytes,
            None => &[],
        }
    }

    /// Marks the given number of bytes as consumed.
    ///
    /// # Arguments
    ///
    /// - `usize` - The number of bytes to consume.
    #[inline(always)]
    fn consume(&mut self, amount: usize) {
        let limit: usize = self.get_end().saturating_sub(self.get_start());
        self.set_start(self.get_start() + amount.min(limit));
    }

    /// Reads one response and returns the length of its body.
    ///
    /// # Arguments
    ///
    /// - `&mut Vec<u8>` - The buffer the body is collected into.
    ///
    /// # Returns
    ///
    /// - `Result<usize, String>` - The body length, or the reason the response
    ///   could not be read.
    async fn read_response(&mut self, body: &mut Vec<u8>) -> Result<usize, String> {
        let content_length: usize = loop {
            let head_end: usize = match find_subsequence(self.available(), HEAD_TERMINATOR) {
                Some(offset) => offset + HEAD_TERMINATOR.len(),
                None => {
                    self.fill().await?;
                    continue;
                }
            };
            let head: &[u8] = match self.available().get(..head_end) {
                Some(head) => head,
                None => &[],
            };
            let length: usize = parse_content_length(head);
            self.consume(head_end);
            break length;
        };
        body.clear();
        while body.len() < content_length {
            let take: usize = self.available().len().min(content_length - body.len());
            if take > 0 {
                body.extend_from_slice(&self.available()[..take]);
                self.consume(take);
            } else {
                self.fill().await?;
            }
        }
        Ok(body.len())
    }
}

/// Drives one connection until the deadline.
///
/// # Arguments
///
/// - `String` - The address of the server under test.
/// - `Vec<Case>` - The request mix.
/// - `Arc<Barrier>` - The barrier that starts every connection together.
/// - `Instant` - The moment the warm-up window ends.
/// - `Instant` - The moment the run ends.
///
/// # Returns
///
/// - `ConnStats` - The counters of the connection.
async fn run_connection(
    address: String,
    cases: Vec<Case>,
    barrier: Arc<Barrier>,
    warmup_until: Instant,
    deadline: Instant,
) -> ConnStats {
    let mut stats: ConnStats = ConnStats::default();
    let mut connection: Connection = match Connection::open(&address).await {
        Ok(connection) => connection,
        Err(_) => return stats,
    };
    let mut body: Vec<u8> = Vec::with_capacity(READ_BUFFER_SIZE);
    let mut index: usize = 0;
    let _: BarrierWaitResult = barrier.wait().await;
    loop {
        let now: Instant = Instant::now();
        if now >= deadline {
            break;
        }
        let Some(case) = cases.get(index % cases.len()) else {
            break;
        };
        index = index.wrapping_add(1);
        if connection
            .get_mut_stream()
            .write_all(&case.bytes)
            .await
            .is_err()
        {
            stats.errors += 1;
            break;
        }
        match connection.read_response(&mut body).await {
            Ok(length) => {
                if length == case.expect_len {
                    stats.completed += 1;
                } else {
                    stats.errors += 1;
                }
            }
            Err(_) => {
                stats.errors += 1;
                break;
            }
        }
        let micros: u128 = now.elapsed().as_micros();
        if now >= warmup_until {
            stats
                .latencies_us
                .push(u64::try_from(micros).unwrap_or(u64::MAX));
        }
    }
    stats
}

/// Runs the load generator.
///
/// # Arguments
///
/// - `Config` - The configuration of the run.
///
/// # Returns
///
/// - `Result<Outcome, String>` - The outcome of the run, or the reason it could
///   not run.
pub async fn run(config: Config) -> Result<Outcome, String> {
    let cases: Vec<Case> = build_cases();
    let barrier: Arc<Barrier> = Arc::new(Barrier::new(config.get_connections()));
    let warmup_until: Instant = Instant::now() + Duration::from_secs(config.get_warmup_secs());
    let deadline: Instant = warmup_until + Duration::from_secs(config.get_duration_secs());
    let mut handles: Vec<JoinHandle<ConnStats>> = Vec::new();
    for _ in 0..config.get_connections() {
        let address: String = config.get_address().clone();
        let cases: Vec<Case> = cases.clone();
        let barrier: Arc<Barrier> = Arc::clone(&barrier);
        handles.push(tokio::spawn(async move {
            run_connection(address, cases, barrier, warmup_until, deadline).await
        }));
    }
    let mut outcome: Outcome = Outcome::default();
    for handle in handles {
        match handle.await {
            Ok(stats) => {
                outcome.completed += stats.completed;
                outcome.errors += stats.errors;
                outcome.latencies_us.extend(stats.latencies_us);
            }
            Err(error) => return Err(error.to_string()),
        }
    }
    Ok(outcome)
}
