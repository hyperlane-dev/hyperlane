//! # Attribute Module
//!
//! This module provides custom attributes for route handling, enabling
//! declarative routing in the Hyperlane framework.

// The `enum` module defines enumerations for route attributes.
pub(crate) mod r#enum;
// The `impl` module provides implementations for attribute handling.
pub(crate) mod r#impl;
// The `type` module defines types related to route attributes.
pub(crate) mod r#type;

// Re-exports types for public use.
pub use r#type::*;

// Makes enumerations available within the crate.
pub(crate) use r#enum::*;
