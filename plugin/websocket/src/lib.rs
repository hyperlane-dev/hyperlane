//! hyperlane-plugin-websocket
//!
//! A WebSocket plugin for the Hyperlane framework,
//! providing robust WebSocket communication capabilities and integrating
//! with hyperlane-broadcast for efficient message dissemination.

mod r#const;
mod r#enum;
mod r#impl;
mod r#static;
mod r#struct;
mod r#trait;

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

pub use {r#enum::*, r#struct::*, r#trait::*};

pub(crate) use {r#const::*, r#static::*};

use std::{
    convert::Infallible,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    num::{
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize, NonZeroU8,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    },
    sync::OnceLock,
};

use {
    hyperlane::{
        tokio::sync::broadcast::{
            Receiver,
            error::{RecvError, SendError},
        },
        *,
    },
    lombok_macros::*,
    tokio_broadcast::*,
};
