//! # Route Module
//!
//! This module handles request routing, matching incoming requests to
//! registered handlers based on their path and other attributes.

// The `const` module defines constants used in routing.
pub(crate) mod r#const;
// The `enum` module defines enumerations for routing.
pub(crate) mod r#enum;
// The `impl` module provides implementations for routing structures.
pub(crate) mod r#impl;
// The `struct` module defines the core routing structures.
pub(crate) mod r#struct;
// The `type` module defines types used in routing.
pub(crate) mod r#type;

// Re-exports routing-related types for public use.
pub use r#type::*;

// Makes routing constants available within the crate.
pub(crate) use r#const::*;
// Makes routing enumerations available within the crate.
pub(crate) use r#enum::*;
// Makes routing structures available within the crate.
pub(crate) use r#struct::*;
