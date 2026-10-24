/// The scheme a cleartext request carries.
pub(crate) const SCHEME_CLEARTEXT: &str = "http";

/// The scheme a secure request carries.
pub(crate) const SCHEME_SECURE: &str = "https";

/// The authority a request addresses.
pub(crate) const AUTHORITY: &str = "bench";

/// The name the server certificate is verified against.
pub(crate) const VERIFY_NAME: &str = "localhost";

/// The value the header route reads.
pub(crate) const HEADER_TAG_VALUE: &str = "alpha";

/// The name of the header the header route reads.
pub(crate) const HEADER_TAG: &str = "x-bench-tag";

/// The reason the generator refused a rate of zero.
pub(crate) const ERROR_RATE_ZERO: &str = "the connection rate must be above zero";

/// The reason the secure protocol refused to start without a certificate
/// authority.
pub(crate) const ERROR_TLS_NEEDS_CA: &str = "the secure protocol needs a certificate authority";

/// The reason the certificate authority could not be read.
pub(crate) const ERROR_CA_UNREADABLE: &str = "cannot read the certificate authority";

/// The reason the client could not resolve the address.
pub(crate) const ERROR_RESOLVE: &str = "cannot resolve the address";

/// The path the static route lives at.
pub(crate) const PATH_ROOT: &str = "/";

/// The address the QUIC client binds its endpoint to.
pub(crate) const QUIC_BIND_ADDRESS: &str = "0.0.0.0:0";
