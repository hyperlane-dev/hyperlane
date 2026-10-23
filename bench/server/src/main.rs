//! The command line entry point of the server the harness measures.

use bench_server::serve;

use std::process::exit;

/// Serves until the process stops.
#[tokio::main]
async fn main() {
    if let Err(error) = serve().await {
        eprintln!("{error}");
        exit(1);
    }
}
