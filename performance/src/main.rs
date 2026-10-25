//! The command line entry point of the hyperlane performance runner.

use hyperlane_performance::run;

use std::{env, process::exit};

/// Parses the command line and runs the requested benchmark operation.
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match run(&args) {
        Ok(()) => {}
        Err(error) => {
            eprintln!("error: {error}");
            exit(1);
        }
    }
}
