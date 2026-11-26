use super::*;

/// Builds a rustls server configuration from the configured TLS material.
///
/// Only one ALPN identifier is advertised: the one belonging to the selected
/// protocol. Advertising a second one would let a client silently fall back to
/// HTTP/1.1 while the caller believes it negotiated the newer protocol, so the
/// handshake fails instead of answering under the wrong version.
///
/// # Arguments
///
/// - `&TlsConfig` - The certificate chain and private key paths to load.
/// - `&[u8]` - The single ALPN protocol identifier to advertise.
/// - `&[&'static SupportedProtocolVersion]` - The TLS versions to offer.
///
/// # Returns
///
/// - `Result<Arc<TlsServerConfig>, Box<ServerError>>` - The TLS configuration, or
///   the first failure while reading the PEM material.
pub fn build_server_tls_config(
    tls: &TlsConfig,
    alpn: &[u8],
    versions: &[&'static SupportedProtocolVersion],
) -> Result<Arc<TlsServerConfig>, Box<ServerError>> {
    let certificate_chain: Vec<CertificateDer<'static>> =
        CertificateDer::pem_file_iter(tls.get_certificate_path())
            .map_err(|error: pem::Error| ServerError::Tls(tls_error_string(&error)))?
            .collect::<Result<Vec<CertificateDer<'static>>, pem::Error>>()
            .map_err(|error: pem::Error| ServerError::Tls(tls_error_string(&error)))?;
    let private_key: PrivateKeyDer<'static> =
        PrivateKeyDer::from_pem_file(tls.get_private_key_path())
            .map_err(|error: pem::Error| ServerError::Tls(tls_error_string(&error)))?;
    let mut provider: CryptoProvider = default_provider();
    let preferred_suites: [SupportedCipherSuite; 9] = [
        TLS13_AES_128_GCM_SHA256,
        TLS13_AES_256_GCM_SHA384,
        TLS13_CHACHA20_POLY1305_SHA256,
        TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256,
        TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384,
        TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256,
        TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256,
        TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384,
        TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256,
    ];
    provider.cipher_suites = preferred_suites.to_vec();
    let mut server_config: TlsServerConfig =
        TlsServerConfig::builder_with_provider(Arc::new(provider))
            .with_protocol_versions(versions)
            .map_err(|error: TlsError| Box::new(ServerError::Tls(error.to_string())))?
            .with_no_client_auth()
            .with_single_cert(certificate_chain, private_key)
            .map_err(|error: TlsError| Box::new(ServerError::Tls(error.to_string())))?;
    server_config.alpn_protocols = vec![alpn.to_vec()];
    server_config.ignore_client_order = true;
    server_config.session_storage = ServerSessionMemoryCache::new(8192);
    if let Ok(ticketer) = Ticketer::new() {
        server_config.ticketer = ticketer;
    }
    Ok(Arc::new(server_config))
}

/// Builds the rustls configuration a stream listener terminates its handshake with.
///
/// # Arguments
///
/// - `&TlsConfig` - The certificate chain and private key paths to load.
/// - `&ServerProtocol` - The protocol whose ALPN identifier is advertised.
///
/// # Returns
///
/// - `Result<Arc<TlsServerConfig>, Box<ServerError>>` - The TLS configuration, or
///   the first failure while reading the PEM material.
pub fn build_stream_tls_config(
    tls: &TlsConfig,
    protocol: &ServerProtocol,
) -> Result<Arc<TlsServerConfig>, Box<ServerError>> {
    let versions: &[&'static SupportedProtocolVersion] = &[&TLS13, &TLS12];
    build_server_tls_config(tls, protocol.get_alpn_protocol(), versions)
}

/// Builds the rustls configuration a QUIC endpoint terminates its handshake with.
///
/// QUIC carries the handshake inside its own transport, which admits TLS 1.3
/// only, so no other version is offered. Zero round trip time is left disabled:
/// replaying an early request without waiting for the handshake is a
/// correctness hazard that the protocol selection does not ask for.
///
/// # Arguments
///
/// - `&TlsConfig` - The certificate chain and private key paths to load.
///
/// # Returns
///
/// - `Result<Arc<TlsServerConfig>, Box<ServerError>>` - The TLS configuration, or
///   the first failure while reading the PEM material.
pub fn build_quic_tls_config(tls: &TlsConfig) -> Result<Arc<TlsServerConfig>, Box<ServerError>> {
    let versions: &[&'static SupportedProtocolVersion] = &[&TLS13];
    build_server_tls_config(tls, ServerProtocol::Http3.get_alpn_protocol(), versions)
}

/// Renders a PEM error into the server error payload.
///
/// The error enum is non exhaustive and carries borrowed data, so it is rendered
/// through its display implementation instead of being destructured.
///
/// # Arguments
///
/// - `&pem::Error` - The failure reported while reading PEM material.
///
/// # Returns
///
/// - `String` - The rendered failure.
fn tls_error_string(error: &pem::Error) -> String {
    format!("{error}")
}
