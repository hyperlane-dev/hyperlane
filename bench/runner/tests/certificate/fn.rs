use super::*;

fn temporary_directory() -> PathBuf {
    let nonce: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_nanos(),
        Err(error) => panic!("{error}"),
    };
    env::temp_dir().join(format!("bench-perf-certificates-{}-{nonce}", id()))
}

#[test]
fn generates_a_verified_certificate_chain() {
    let directory: PathBuf = temporary_directory();
    match generate_certificates(&directory) {
        Ok(()) => {}
        Err(error) => panic!("{error}"),
    }
    let ca: PathBuf = directory.join("ca.pem");
    let leaf: PathBuf = directory.join("leaf.pem");
    let key: PathBuf = directory.join("leaf.key");
    assert!(ca.is_file());
    assert!(leaf.is_file());
    assert!(key.is_file());
    assert!(!directory.join("ca.key").exists());
    let verify_args: Vec<String> = vec![
        "verify".to_owned(),
        "-CAfile".to_owned(),
        ca.display().to_string(),
        leaf.display().to_string(),
    ];
    let verify: Output = match Command::new("openssl").args(verify_args).output() {
        Ok(output) => output,
        Err(error) => panic!("{error}"),
    };
    assert!(
        verify.status.success(),
        "{}",
        String::from_utf8_lossy(&verify.stderr)
    );
    let inspect_args: Vec<String> = vec![
        "x509".to_owned(),
        "-in".to_owned(),
        leaf.display().to_string(),
        "-noout".to_owned(),
        "-text".to_owned(),
    ];
    let inspect: Output = match Command::new("openssl").args(inspect_args).output() {
        Ok(output) => output,
        Err(error) => panic!("{error}"),
    };
    assert!(inspect.status.success());
    let certificate_text: String = String::from_utf8_lossy(&inspect.stdout).into_owned();
    assert!(certificate_text.contains("DNS:perf-server-a"));
    assert!(certificate_text.contains("DNS:perf-server-b"));
    assert!(certificate_text.contains("DNS:perf-server-control"));
    assert!(certificate_text.contains("DNS:localhost"));
    assert!(certificate_text.contains("IP Address:127.0.0.1"));
    #[cfg(unix)]
    {
        let mode: u32 = match fs::metadata(&key) {
            Ok(metadata) => metadata.permissions().mode() & 0o777,
            Err(error) => panic!("{error}"),
        };
        assert_eq!(mode, 0o600);
    }
    match fs::remove_dir_all(&directory) {
        Ok(()) => {}
        Err(error) => panic!("{error}"),
    }
}
