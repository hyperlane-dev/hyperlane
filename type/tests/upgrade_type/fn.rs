use super::*;

#[test]
fn test_upgrade_type_default() {
    let upgrade: UpgradeType = UpgradeType::default();
    assert!(upgrade.is_unknown());
    assert_eq!(upgrade.to_string(), "");
}

#[test]
fn test_upgrade_type_websocket() {
    let upgrade: UpgradeType = UpgradeType::WebSocket;
    assert!(upgrade.is_ws());
    assert!(!upgrade.is_h2c());
    assert!(!upgrade.is_tls());
    assert!(!upgrade.is_unknown());
    assert_eq!(upgrade.to_string(), "websocket");
}

#[test]
fn test_upgrade_type_h2c() {
    let upgrade: UpgradeType = UpgradeType::H2c;
    assert!(!upgrade.is_ws());
    assert!(upgrade.is_h2c());
    assert!(!upgrade.is_tls());
    assert!(!upgrade.is_unknown());
    assert_eq!(upgrade.to_string(), "h2c");
}

#[test]
fn test_upgrade_type_tls() {
    let upgrade: UpgradeType = UpgradeType::Tls("tls1.3".to_string());
    assert!(!upgrade.is_ws());
    assert!(!upgrade.is_h2c());
    assert!(upgrade.is_tls());
    assert!(!upgrade.is_unknown());
    assert_eq!(upgrade.to_string(), "tls1.3");
}

#[test]
fn test_upgrade_type_unknown() {
    let upgrade: UpgradeType = UpgradeType::Unknown("custom".to_string());
    assert!(!upgrade.is_ws());
    assert!(!upgrade.is_h2c());
    assert!(!upgrade.is_tls());
    assert!(upgrade.is_unknown());
    assert_eq!(upgrade.to_string(), "custom");
}

#[test]
fn test_upgrade_type_from_str_websocket() {
    let upgrade: UpgradeType = "websocket".parse().unwrap();
    assert_eq!(upgrade, UpgradeType::WebSocket);
}

#[test]
fn test_upgrade_type_from_str_h2c() {
    let upgrade: UpgradeType = "h2c".parse().unwrap();
    assert_eq!(upgrade, UpgradeType::H2c);
}

#[test]
fn test_upgrade_type_from_str_tls() {
    let upgrade: UpgradeType = "tls1.2".parse().unwrap();
    assert_eq!(upgrade, UpgradeType::Tls("tls1.2".to_string()));
}

#[test]
fn test_upgrade_type_from_str_unknown() {
    let upgrade: UpgradeType = "custom".parse().unwrap();
    assert_eq!(upgrade, UpgradeType::Unknown("custom".to_string()));
}

#[test]
fn test_upgrade_type_from_str_case_insensitive() {
    let upgrade1: UpgradeType = "WebSocket".parse().unwrap();
    let upgrade2: UpgradeType = "WEBSOCKET".parse().unwrap();
    let upgrade3: UpgradeType = "H2C".parse().unwrap();
    assert_eq!(upgrade1, UpgradeType::WebSocket);
    assert_eq!(upgrade2, UpgradeType::WebSocket);
    assert_eq!(upgrade3, UpgradeType::H2c);
}

#[test]
fn test_upgrade_type_clone() {
    let upgrade: UpgradeType = UpgradeType::WebSocket;
    let cloned: UpgradeType = upgrade.clone();
    assert_eq!(upgrade, cloned);
}

#[test]
fn test_upgrade_type_eq() {
    assert_eq!(UpgradeType::WebSocket, UpgradeType::WebSocket);
    assert_ne!(UpgradeType::WebSocket, UpgradeType::H2c);
    assert_eq!(
        UpgradeType::Unknown("a".to_string()),
        UpgradeType::Unknown("a".to_string())
    );
    assert_ne!(
        UpgradeType::Unknown("a".to_string()),
        UpgradeType::Unknown("b".to_string())
    );
}
