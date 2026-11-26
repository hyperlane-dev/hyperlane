use super::*;

#[test]
fn test_security_decompression_bomb_capped_across_gzip_deflate_brotli() {
    let huge: Vec<u8> = vec![0u8; 68 * 1024 * 1024];
    for algo in [Compress::Gzip, Compress::Deflate, Compress::Br] {
        let compressed: Cow<'_, [u8]> = algo.encode(&huge, 8192);
        assert!(compressed.len() < 512 * 1024);
        let decoded: Cow<'_, [u8]> = algo.decode(&compressed, 8192);
        assert!(decoded.len() <= 64 * 1024 * 1024);
    }
}

#[test]
fn test_security_accept_encoding_strict_qvalue_x_gzip_and_multi_member_gzip() {
    assert_eq!(
        Compress::from_accept_encoding("x-gzip;q=0.9, deflate;q=0.5"),
        Some(Compress::Gzip)
    );
    assert_eq!(
        Compress::from_accept_encoding("GZIP;q=1.000"),
        Some(Compress::Gzip)
    );
    assert_eq!(Compress::from_accept_encoding("gzip;q=1.5"), None);
    assert_eq!(Compress::from_accept_encoding("gzip;q=NaN"), None);
    assert_eq!(Compress::from_accept_encoding("gzip;q=-0.1"), None);
    assert_eq!(Compress::from_accept_encoding("gzip;q=1.0000"), None);
    assert_eq!(Compress::from_accept_encoding("gzip;q=0.0"), None);
    let mut multi: Vec<u8> = Compress::Gzip.encode(b"part-one-", 1024).into_owned();
    multi.extend_from_slice(&Compress::Gzip.encode(b"part-two", 1024));
    let decoded: Cow<'_, [u8]> = Compress::Gzip.decode(&multi, 1024);
    assert_eq!(decoded.as_ref(), b"part-one-part-two");
}
