//! # Panic Module
//!
//! This module provides utilities for handling panics within the framework,
//! including custom panic hooks and panic information capturing.

// The `fn` module contains functions related to panic handling.
pub(crate) mod r#fn;
// The `impl` module provides implementations for panic-related structures.
pub(crate) mod r#impl;
// The `struct` module defines structures for capturing panic information.
pub(crate) mod r#struct;
// The `type` module defines types used in panic handling.
pub(crate) mod r#type;

// Re-exports panic-related structures for public use.
pub use r#struct::*;
// Re-exports panic-related types for public use.
pub use r#type::*;

// Makes panic-related functions available within the crate.
pub(crate) use r#fn::*;
