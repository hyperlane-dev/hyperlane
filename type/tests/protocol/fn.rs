use super::*;

#[test]
fn test_is_http_with_lowercase() {
    assert!(Protocol::is_http("http"));
}

#[test]
fn test_is_http_with_uppercase() {
    assert!(Protocol::is_http("HTTP"));
}

#[test]
fn test_is_http_with_mixed_case() {
    assert!(Protocol::is_http("Http"));
    assert!(Protocol::is_http("hTtP"));
}

#[test]
fn test_is_http_with_non_http_protocol() {
    assert!(!Protocol::is_http("https"));
    assert!(!Protocol::is_http("ftp"));
    assert!(!Protocol::is_http(""));
}

#[test]
fn test_is_https_with_lowercase() {
    assert!(Protocol::is_https("https"));
}

#[test]
fn test_is_https_with_uppercase() {
    assert!(Protocol::is_https("HTTPS"));
}

#[test]
fn test_is_https_with_mixed_case() {
    assert!(Protocol::is_https("Https"));
    assert!(Protocol::is_https("hTtPs"));
}

#[test]
fn test_is_https_with_non_https_protocol() {
    assert!(!Protocol::is_https("http"));
    assert!(!Protocol::is_https("ftp"));
    assert!(!Protocol::is_https(""));
}

#[test]
fn test_get_port_for_http() {
    assert_eq!(Protocol::get_port("http"), 80);
    assert_eq!(Protocol::get_port("HTTP"), 80);
    assert_eq!(Protocol::get_port("Http"), 80);
}

#[test]
fn test_get_port_for_https() {
    assert_eq!(Protocol::get_port("https"), 443);
    assert_eq!(Protocol::get_port("HTTPS"), 443);
    assert_eq!(Protocol::get_port("Https"), 443);
}

#[test]
fn test_get_port_for_ftp() {
    assert_eq!(Protocol::get_port("ftp"), 21);
    assert_eq!(Protocol::get_port("FTP"), 21);
    assert_eq!(Protocol::get_port("Ftp"), 21);
}

#[test]
fn test_get_port_for_ftps() {
    assert_eq!(Protocol::get_port("ftps"), 990);
    assert_eq!(Protocol::get_port("FTPS"), 990);
    assert_eq!(Protocol::get_port("Ftps"), 990);
}

#[test]
fn test_get_port_for_ssh() {
    assert_eq!(Protocol::get_port("ssh"), 22);
    assert_eq!(Protocol::get_port("SSH"), 22);
    assert_eq!(Protocol::get_port("Ssh"), 22);
}

#[test]
fn test_get_port_for_sftp() {
    assert_eq!(Protocol::get_port("sftp"), 22);
    assert_eq!(Protocol::get_port("SFTP"), 22);
    assert_eq!(Protocol::get_port("Sftp"), 22);
}

#[test]
fn test_get_port_for_telnet() {
    assert_eq!(Protocol::get_port("telnet"), 23);
    assert_eq!(Protocol::get_port("TELNET"), 23);
    assert_eq!(Protocol::get_port("Telnet"), 23);
}

#[test]
fn test_get_port_for_smtp() {
    assert_eq!(Protocol::get_port("smtp"), 25);
    assert_eq!(Protocol::get_port("SMTP"), 25);
    assert_eq!(Protocol::get_port("Smtp"), 25);
}

#[test]
fn test_get_port_for_smtps() {
    assert_eq!(Protocol::get_port("smtps"), 465);
    assert_eq!(Protocol::get_port("SMTPS"), 465);
    assert_eq!(Protocol::get_port("Smtps"), 465);
}

#[test]
fn test_get_port_for_pop3() {
    assert_eq!(Protocol::get_port("pop3"), 110);
    assert_eq!(Protocol::get_port("POP3"), 110);
    assert_eq!(Protocol::get_port("Pop3"), 110);
}

#[test]
fn test_get_port_for_pop3s() {
    assert_eq!(Protocol::get_port("pop3s"), 995);
    assert_eq!(Protocol::get_port("POP3S"), 995);
    assert_eq!(Protocol::get_port("Pop3s"), 995);
}

#[test]
fn test_get_port_for_imap() {
    assert_eq!(Protocol::get_port("imap"), 143);
    assert_eq!(Protocol::get_port("IMAP"), 143);
    assert_eq!(Protocol::get_port("Imap"), 143);
}

#[test]
fn test_get_port_for_imaps() {
    assert_eq!(Protocol::get_port("imaps"), 993);
    assert_eq!(Protocol::get_port("IMAPS"), 993);
    assert_eq!(Protocol::get_port("Imaps"), 993);
}

#[test]
fn test_get_port_for_dns() {
    assert_eq!(Protocol::get_port("dns"), 53);
    assert_eq!(Protocol::get_port("DNS"), 53);
    assert_eq!(Protocol::get_port("Dns"), 53);
}

#[test]
fn test_get_port_for_ws() {
    assert_eq!(Protocol::get_port("ws"), 80);
    assert_eq!(Protocol::get_port("WS"), 80);
    assert_eq!(Protocol::get_port("Ws"), 80);
}

#[test]
fn test_get_port_for_wss() {
    assert_eq!(Protocol::get_port("wss"), 443);
    assert_eq!(Protocol::get_port("WSS"), 443);
    assert_eq!(Protocol::get_port("Wss"), 443);
}

#[test]
fn test_get_port_for_unknown_protocol() {
    assert_eq!(Protocol::get_port("unknown"), 80);
    assert_eq!(Protocol::get_port(""), 80);
    assert_eq!(Protocol::get_port("custom"), 80);
}

#[test]
fn test_protocol_struct_creation() {
    let protocol: Protocol = Protocol::new();
    let _protocol_copy: Protocol = protocol;
}

#[test]
fn test_protocol_default() {
    let _protocol: Protocol = Protocol;
}
