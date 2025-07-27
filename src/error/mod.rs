//! # Error Module
//!
//! This module defines the custom error types used throughout the Hyperlane framework.

// The `enum` module defines the primary error enumerations.
pub(crate) mod r#enum;

// Re-exports the error enumerations for public use.
pub use r#enum::*;
