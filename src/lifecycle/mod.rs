//! # Lifecycle Module
//!
//! This module manages the connection lifecycle, including its state
//! and transitions.

// The `enum` module defines the lifecycle states.
pub(crate) mod r#enum;
// The `impl` module provides implementations for lifecycle management.
pub(crate) mod r#impl;

// Makes the lifecycle enumerations available within the crate.
pub(crate) use r#enum::*;
