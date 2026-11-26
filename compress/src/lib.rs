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
#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;
mod xx_hash3_64;

pub use {compress::*, xx_hash3_64::*};

use std::{
    borrow::Cow,
    collections::HashMap,
    fmt,
    io::{self, Read, Write},
    mem::MaybeUninit,
    ptr, slice,
    str::FromStr,
};

use {
    ::brotli::{CompressorWriter, Decompressor},
    core::hash::BuildHasherDefault,
    flate2::{
        Compression,
        read::{DeflateDecoder, MultiGzDecoder},
        write::{DeflateEncoder, GzEncoder},
    },
    twox_hash::xxhash3_64::{self, DEFAULT_SECRET_LENGTH, RawHasher, SecretBuffer},
};
