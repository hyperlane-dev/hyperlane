use super::*;

/// Runs one OpenSSL step and reports its captured standard error.
///
/// # Arguments
///
/// - `&str` - The human-readable step name.
/// - `&[String]` - The OpenSSL arguments.
///
/// # Returns
///
/// - `Result<(), String>` - Success, or the process error.
fn run_openssl(step: &str, args: &[String]) -> Result<(), String> {
    let output: Output = match Command::new(OPENSSL_PROGRAM).args(args).output() {
        Ok(output) => output,
        Err(error) => return Err(format!("could not start OpenSSL for {step}: {error}")),
    };
    if !output.status.success() {
        return Err(format!(
            "the certificate generator failed at {step}: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(())
}

/// Copies one generated certificate file.
///
/// # Arguments
///
/// - `&Path` - The source path.
/// - `&Path` - The destination path.
///
/// # Returns
///
/// - `Result<(), String>` - Success, or the copy error.
fn copy_certificate_file(source: &Path, destination: &Path) -> Result<(), String> {
    match fs::copy(source, destination) {
        Ok(_) => Ok(()),
        Err(error) => Err(format!(
            "could not copy {} to {}: {error}",
            source.display(),
            destination.display()
        )),
    }
}

/// Generates and verifies the certificate chain inside a temporary directory.
///
/// # Arguments
///
/// - `&Path` - The destination directory.
/// - `&Path` - The temporary working directory.
///
/// # Returns
///
/// - `Result<(), String>` - Success, or the failed generation step.
fn generate_in_work(target: &Path, work: &Path) -> Result<(), String> {
    let ca_key: PathBuf = work.join(CA_KEY_NAME);
    let ca_pem: PathBuf = work.join(CA_FILE_NAME);
    let leaf_key: PathBuf = work.join(LEAF_KEY_FILE_NAME);
    let leaf_csr: PathBuf = work.join(LEAF_CSR_NAME);
    let leaf_ext: PathBuf = work.join(LEAF_EXT_NAME);
    let leaf_pem: PathBuf = work.join(LEAF_CERT_FILE_NAME);
    let ca_args: Vec<String> = vec![
        REQUEST_SUBCOMMAND.to_owned(),
        X509_FLAG.to_owned(),
        NEWKEY_FLAG.to_owned(),
        RSA_2048_SETTING.to_owned(),
        SHA256_SETTING.to_owned(),
        DAYS_FLAG.to_owned(),
        CERTIFICATE_DAYS.to_owned(),
        NODES_FLAG.to_owned(),
        KEYOUT_FLAG.to_owned(),
        ca_key.display().to_string(),
        OUT_FLAG.to_owned(),
        ca_pem.display().to_string(),
        SUBJECT_FLAG.to_owned(),
        CA_SUBJECT.to_owned(),
        ADDEXT_FLAG.to_owned(),
        CA_EXTENSION.to_owned(),
    ];
    run_openssl("the certificate authority", &ca_args)?;
    let leaf_args: Vec<String> = vec![
        REQUEST_SUBCOMMAND.to_owned(),
        NEWKEY_FLAG.to_owned(),
        RSA_2048_SETTING.to_owned(),
        NODES_FLAG.to_owned(),
        KEYOUT_FLAG.to_owned(),
        leaf_key.display().to_string(),
        OUT_FLAG.to_owned(),
        leaf_csr.display().to_string(),
        SUBJECT_FLAG.to_owned(),
        LEAF_SUBJECT.to_owned(),
    ];
    run_openssl("the leaf key", &leaf_args)?;
    match fs::write(&leaf_ext, LEAF_EXTENSIONS) {
        Ok(()) => {}
        Err(error) => return Err(format!("could not write {}: {error}", leaf_ext.display())),
    }
    let sign_args: Vec<String> = vec![
        X509_SUBCOMMAND.to_owned(),
        REQUEST_INPUT_FLAG.to_owned(),
        INPUT_FLAG.to_owned(),
        leaf_csr.display().to_string(),
        CA_FLAG.to_owned(),
        ca_pem.display().to_string(),
        CAKEY_FLAG.to_owned(),
        ca_key.display().to_string(),
        CACREATESERIAL_FLAG.to_owned(),
        OUT_FLAG.to_owned(),
        leaf_pem.display().to_string(),
        DAYS_FLAG.to_owned(),
        CERTIFICATE_DAYS.to_owned(),
        SHA256_SETTING.to_owned(),
        EXTFILE_FLAG.to_owned(),
        leaf_ext.display().to_string(),
    ];
    run_openssl("the leaf certificate", &sign_args)?;
    let target_ca: PathBuf = target.join(CA_FILE_NAME);
    let target_leaf: PathBuf = target.join(LEAF_CERT_FILE_NAME);
    let target_key: PathBuf = target.join(LEAF_KEY_FILE_NAME);
    copy_certificate_file(&ca_pem, &target_ca)?;
    copy_certificate_file(&leaf_pem, &target_leaf)?;
    copy_certificate_file(&leaf_key, &target_key)?;
    #[cfg(unix)]
    {
        let permissions: fs::Permissions = fs::Permissions::from_mode(PRIVATE_KEY_MODE);
        match fs::set_permissions(&target_key, permissions) {
            Ok(()) => {}
            Err(error) => {
                return Err(format!(
                    "could not protect {}: {error}",
                    target_key.display()
                ));
            }
        }
    }
    let verify_args: Vec<String> = vec![
        VERIFY_SUBCOMMAND.to_owned(),
        CAFILE_FLAG.to_owned(),
        target_ca.display().to_string(),
        target_leaf.display().to_string(),
    ];
    run_openssl("the chain verification", &verify_args)
}

/// Generates a certificate authority and a verified server certificate.
///
/// # Arguments
///
/// - `&Path` - The directory that receives `ca.pem`, `leaf.pem` and `leaf.key`.
///
/// # Returns
///
/// - `Result<(), String>` - Success, or the failed generation or cleanup step.
pub fn generate_certificates(target: &Path) -> Result<(), String> {
    match fs::create_dir_all(target) {
        Ok(()) => {}
        Err(error) => {
            return Err(format!("could not create {}: {error}", target.display()));
        }
    }
    let nonce: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_nanos(),
        Err(error) => return Err(format!("could not read the clock: {error}")),
    };
    let work: PathBuf = env::temp_dir().join(format!("{CERT_WORK_PREFIX}{}-{nonce}", id()));
    match fs::create_dir(&work) {
        Ok(()) => {}
        Err(error) => {
            return Err(format!(
                "could not create temporary directory {}: {error}",
                work.display()
            ));
        }
    }
    let result: Result<(), String> = generate_in_work(target, &work);
    let cleanup: Result<(), io::Error> = fs::remove_dir_all(&work);
    match (result, cleanup) {
        (Err(error), _) => Err(error),
        (Ok(()), Err(error)) => Err(format!(
            "could not remove temporary directory {}: {error}",
            work.display()
        )),
        (Ok(()), Ok(())) => Ok(()),
    }
}
