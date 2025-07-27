//! # Config Module
//!
//! This module defines the configuration structures and related implementations
//! for the Hyperlane framework.

// The `impl` module provides implementations for configuration-related functionality.
pub(crate) mod r#impl;
// The `struct` module defines the core configuration structures.
pub(crate) mod r#struct;

// Makes the configuration structures available within the crate.
pub(crate) use r#struct::*;
