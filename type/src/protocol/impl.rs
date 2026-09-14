use super::*;

/// Implementation of protocol identification and port resolution methods.
///
/// This implementation block provides utility functions for working with
/// HTTP protocol strings, enabling identification of HTTP/HTTPS variants
/// and retrieval of their standard port numbers.
impl Protocol {
    /// Checks if the given protocol string represents HTTP.
    ///
    /// Performs a case-insensitive comparison against the HTTP protocol identifier.
    ///
    /// # Arguments
    /// - `&str`: A string slice representing the protocol to check.
    ///
    /// # Returns
    /// - `bool`: Returns `true` if the protocol is HTTP (case-insensitive), `false` otherwise.
    #[inline(always)]
    pub fn is_http(protocol: &str) -> bool {
        matches!(protocol.to_lowercase().as_str(), HTTP_LOWERCASE)
    }

    /// Checks if the given protocol string represents HTTPS.
    ///
    /// Performs a case-insensitive comparison against the HTTPS protocol identifier.
    ///
    /// # Arguments
    /// - `&str`: A string slice representing the protocol to check.
    ///
    /// # Returns
    /// - `bool`: Returns `true` if the protocol is HTTPS (case-insensitive), `false` otherwise.
    #[inline(always)]
    pub fn is_https(protocol: &str) -> bool {
        matches!(protocol.to_lowercase().as_str(), HTTPS_LOWERCASE)
    }

    /// Returns the default port number for the given protocol.
    ///
    /// Performs a case-insensitive comparison to determine the protocol type
    /// and returns the corresponding standard port number.
    ///
    /// # Arguments
    /// - `&str`: A string slice representing the protocol to lookup.
    ///
    /// # Returns
    /// - `u16`: The default port number for the protocol.
    #[inline(always)]
    pub fn get_port(protocol: &str) -> u16 {
        match protocol.to_lowercase().as_str() {
            HTTP_LOWERCASE => 80,
            HTTPS_LOWERCASE => 443,
            FTP_LOWERCASE => 21,
            FTPS_LOWERCASE => 990,
            SFTP_LOWERCASE => 22,
            SSH_LOWERCASE => 22,
            TELNET_LOWERCASE => 23,
            SMTP_LOWERCASE => 25,
            SMTPS_LOWERCASE => 465,
            POP3_LOWERCASE => 110,
            POP3S_LOWERCASE => 995,
            IMAP_LOWERCASE => 143,
            IMAPS_LOWERCASE => 993,
            DNS_LOWERCASE => 53,
            WS_LOWERCASE => 80,
            WSS_LOWERCASE => 443,
            _ => 80,
        }
    }
}
