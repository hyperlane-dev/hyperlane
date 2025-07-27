//! # Hook Module
//!
//! This module provides traits and types for hooking into the application's
//! request-response lifecycle, allowing for custom middleware-like functionality.

// The `impl` module provides implementations for hook-related functionality.
pub(crate) mod r#impl;
// The `trait` module defines the core hook traits.
pub(crate) mod r#trait;
// The `type` module defines types used within the hook system.
pub(crate) mod r#type;

// Re-exports the hook traits for public use.
pub use r#trait::*;
// Re-exports the hook types for public use.
pub use r#type::*;
