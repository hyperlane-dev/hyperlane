//! http-compress
//!
//! A high-performance async library for HTTP compression/decompression,
//! supporting Brotli, Deflate, and Gzip algorithms. Provides both compression
//! and decompression capabilities with optimized memory usage,
//! ideal for HTTP clients/servers and network programming.

mod brotli;
mod compress;
mod deflate;
mod gzip;

pub use compress::*;

pub use twox_hash::XxHash3_64;

use std::{
    borrow::Cow,
    collections::HashMap,
    fmt,
    io::{BufReader, BufWriter, Read, prelude::*},
    str::FromStr,
};

use {
    ::brotli::Decompressor,
    core::hash::BuildHasherDefault,
    flate2::{
        Compression,
        read::{DeflateDecoder, GzDecoder},
        write::{DeflateEncoder, GzEncoder},
    },
};
