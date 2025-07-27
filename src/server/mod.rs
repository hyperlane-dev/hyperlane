//! # Server Module
//!
//! This module contains the core HTTP server implementation, including the main
//! server structure and connection handling logic.

// The `impl` module provides implementations for the server.
pub(crate) mod r#impl;
// The `struct` module defines the main server structure.
pub(crate) mod r#struct;
// The `type` module defines types used by the server.
pub(crate) mod r#type;

// Re-exports the server structure for public use.
pub use r#struct::*;
// Re-exports server-related types for public use.
pub use r#type::*;
