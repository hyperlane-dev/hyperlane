//! The command line entry point of the hyperlane load generator.

use bench_loadgen::{Config, Outcome, run};

use std::process::exit;

/// Parses the command line, runs the generator and prints its result.
#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let config: Config = match Config::from_args(&args) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{error}");
            exit(2);
        }
    };
    let outcome: Outcome = match run(config.clone()).await {
        Ok(outcome) => outcome,
        Err(error) => {
            eprintln!("{error}");
            exit(1);
        }
    };
    println!("{}", outcome.to_json(&config));
}
