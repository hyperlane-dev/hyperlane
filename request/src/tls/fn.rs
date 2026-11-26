use super::*;

/// Constructs a rustls [`ClientConfig`] with session resumption and protocol ALPN.
///
/// # Arguments
///
/// - `RootCertStore` - Trust anchor root certificate store.
/// - `bool` - Whether to advertise `h2` ALPN ahead of `http/1.1`.
///
/// # Returns
///
/// - `Arc<ClientConfig>` - Ready shared client TLS configuration.
fn build_client_config(roots: RootCertStore, enable_h2: bool) -> Arc<ClientConfig> {
    let mut cfg: ClientConfig = ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();
    cfg.resumption = Resumption::in_memory_sessions(TLS_CLIENT_SESSION_CACHE_CAP);
    cfg.alpn_protocols = if enable_h2 {
        vec![ALPN_H2.to_vec(), ALPN_HTTP1_1.to_vec()]
    } else {
        vec![ALPN_HTTP1_1.to_vec()]
    };
    Arc::new(cfg)
}

/// Returns a cached default TLS config or constructs one for a custom root certificate store.
///
/// # Arguments
///
/// - `&RootCertStore` - The request's configured root certificate store.
/// - `bool` - Whether HTTP/2 ALPN should be advertised.
///
/// # Returns
///
/// - `Arc<ClientConfig>` - Shared rustls client configuration.
pub(crate) fn resolve_tls_client_config(
    roots: &RootCertStore,
    enable_h2: bool,
) -> Arc<ClientConfig> {
    if roots.roots.len() == TLS_SERVER_ROOTS.len() {
        let cache: &OnceLock<Arc<ClientConfig>> = if enable_h2 {
            &DEFAULT_H2_TLS_CONFIG
        } else {
            &DEFAULT_H1_TLS_CONFIG
        };
        return Arc::clone(cache.get_or_init(|| {
            let default_roots: RootCertStore = RootCertStore {
                roots: TLS_SERVER_ROOTS.to_vec(),
            };
            build_client_config(default_roots, enable_h2)
        }));
    }
    build_client_config(roots.clone(), enable_h2)
}

/// Parses one or more PEM-encoded X.509 certificates and inserts them into `roots`.
///
/// # Arguments
///
/// - `&mut RootCertStore` - Target root certificate store.
/// - `&[u8]` - Raw PEM-encoded certificate bytes.
///
/// # Returns
///
/// - `Result<usize, RequestError>` - Number of certificates added, or an error when none are valid.
pub(crate) fn add_pem_certs_to_store(
    roots: &mut RootCertStore,
    pem_bytes: &[u8],
) -> Result<usize, RequestError> {
    let mut added: usize = 0;
    for parsed in CertificateDer::pem_slice_iter(pem_bytes) {
        let cert: CertificateDer<'static> =
            parsed.map_err(|_| RequestError::Request(INVALID_PEM_CERT_ERROR.to_owned()))?;
        roots
            .add(cert)
            .map_err(|_| RequestError::Request(INVALID_PEM_CERT_ERROR.to_owned()))?;
        added += 1;
    }
    if added == 0 {
        return Err(RequestError::Request(INVALID_PEM_CERT_ERROR.to_owned()));
    }
    Ok(added)
}
