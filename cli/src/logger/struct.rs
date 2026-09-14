use lombok_macros::{Data, New};

/// Custom logger implementation for the hyperlane-cli.
///
/// Implements the `log::Log` trait to provide colored console output
/// matching the euv CLI log format.
#[derive(Data, New)]
pub struct Logger;
