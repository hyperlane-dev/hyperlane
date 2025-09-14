use crate::*;

#[test]
fn panic_new() {
    let panic: Panic = Panic::new(
        Some("message".to_string()),
        Some("location".to_string()),
        Some("payload".to_string()),
    );
    assert_eq!(panic.get_message(), &Some("message".to_string()));
    assert_eq!(panic.get_location(), &Some("location".to_string()));
    assert_eq!(panic.get_payload(), &Some("payload".to_string()));
}

#[tokio::test]
async fn from_join_error() {
    let handle: JoinHandle<()> = tokio::spawn(async {
        panic!("test panic");
    });
    let result: Result<(), JoinError> = handle.await;
    assert!(result.is_err());
    if let Err(join_error) = result {
        let is_panic: bool = Panic::from_join_error(join_error)
            .get_message()
            .clone()
            .unwrap_or_default()
            .contains("test panic");
        assert!(is_panic);
    }
}
