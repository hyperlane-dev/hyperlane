/// Initial byte size of pooled TLS incoming/outgoing record buffers (fits one max 16 KiB TLS record + overhead).
pub(crate) const TLS_RECORD_BUFFER_SIZE: usize = 18 * 1024;

/// Maximum plaintext bytes encrypted into a single TLS record.
pub(crate) const TLS_MAX_PLAINTEXT_CHUNK: usize = 16 * 1024;

/// Maximum incoming ciphertext buffer growth allowed during handshake or record framing.
pub(crate) const TLS_MAX_INCOMING_BUFFER: usize = 64 * 1024;

/// Maximum number of reusable TLS record buffers kept per worker thread.
pub(crate) const MAX_POOLED_TLS_BUFFERS: usize = 64;

/// Failure reported if the TCP socket accepts no bytes during TLS transmission.
pub(crate) const TLS_TRANSMIT_WRITE_ZERO: &str = "tcp write zero during tls transmit";
