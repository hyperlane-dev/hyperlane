//! # Context Module
//!
//! This module defines the application context, which holds shared state
//! and provides access to request and response data.

// The `impl` module provides implementations for context-related functionality.
pub(crate) mod r#impl;
// The `struct` module defines the core context structures.
pub(crate) mod r#struct;
// The `type` module defines types used within the context module.
pub(crate) mod r#type;

// Re-exports the context structures for public use.
pub use r#struct::*;

// Makes context-related types available within the crate.
pub(crate) use r#type::*;
