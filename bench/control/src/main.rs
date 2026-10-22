//! The command line entry point of the control server.

use bench_control::serve;

use std::process::exit;

/// Serves until the process stops.
#[tokio::main]
async fn main() {
    if let Err(error) = serve().await {
        eprintln!("{error}");
        exit(1);
    }
}
