/// The OpenSSL executable.
pub(crate) const OPENSSL_PROGRAM: &str = "openssl";
/// The request subcommand.
pub(crate) const REQUEST_SUBCOMMAND: &str = "req";
/// The certificate request input flag.
pub(crate) const REQUEST_INPUT_FLAG: &str = "-req";
/// The X.509 subcommand.
pub(crate) const X509_SUBCOMMAND: &str = "x509";
/// The verify subcommand.
pub(crate) const VERIFY_SUBCOMMAND: &str = "verify";
/// The self-signed certificate flag.
pub(crate) const X509_FLAG: &str = "-x509";
/// The new key flag.
pub(crate) const NEWKEY_FLAG: &str = "-newkey";
/// The RSA 2048-bit key setting.
pub(crate) const RSA_2048_SETTING: &str = "rsa:2048";
/// The SHA-256 setting.
pub(crate) const SHA256_SETTING: &str = "-sha256";
/// The certificate lifetime in days.
pub(crate) const CERTIFICATE_DAYS: &str = "3650";
/// The certificate lifetime flag.
pub(crate) const DAYS_FLAG: &str = "-days";
/// The unencrypted key flag.
pub(crate) const NODES_FLAG: &str = "-nodes";
/// The key output flag.
pub(crate) const KEYOUT_FLAG: &str = "-keyout";
/// The output flag.
pub(crate) const OUT_FLAG: &str = "-out";
/// The subject flag.
pub(crate) const SUBJECT_FLAG: &str = "-subj";
/// The added extension flag.
pub(crate) const ADDEXT_FLAG: &str = "-addext";
/// The input flag.
pub(crate) const INPUT_FLAG: &str = "-in";
/// The CA certificate flag.
pub(crate) const CA_FLAG: &str = "-CA";
/// The CA key flag.
pub(crate) const CAKEY_FLAG: &str = "-CAkey";
/// The CA serial creation flag.
pub(crate) const CACREATESERIAL_FLAG: &str = "-CAcreateserial";
/// The extension file flag.
pub(crate) const EXTFILE_FLAG: &str = "-extfile";
/// The CA verification file flag.
pub(crate) const CAFILE_FLAG: &str = "-CAfile";
/// The CA subject.
pub(crate) const CA_SUBJECT: &str = "/CN=hyperlane-bench-ca";
/// The leaf subject.
pub(crate) const LEAF_SUBJECT: &str = "/CN=perf-server";
/// The CA basic constraints.
pub(crate) const CA_EXTENSION: &str = "basicConstraints=critical,CA:TRUE";
/// The temporary CA key name.
pub(crate) const CA_KEY_NAME: &str = "ca.key";
/// The temporary leaf CSR name.
pub(crate) const LEAF_CSR_NAME: &str = "leaf.csr";
/// The temporary leaf extension file name.
pub(crate) const LEAF_EXT_NAME: &str = "leaf.ext";
/// The leaf X.509 extensions.
pub(crate) const LEAF_EXTENSIONS: &str = "basicConstraints=CA:FALSE\nkeyUsage=critical,digitalSignature,keyEncipherment\nextendedKeyUsage=serverAuth\nsubjectAltName=DNS:perf-server-a,DNS:perf-server-b,DNS:perf-server-control,DNS:localhost,IP:127.0.0.1\n";
/// The temporary directory prefix.
pub(crate) const CERT_WORK_PREFIX: &str = "hyperlane-perf-certs-";
/// The private key permissions.
#[cfg(unix)]
pub(crate) const PRIVATE_KEY_MODE: u32 = 0o600;
