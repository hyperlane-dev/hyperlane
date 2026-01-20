use crate::*;

#[test]
fn panic_new() {
    let panic: PanicData = PanicData::new(
        Some("message".to_string()),
        Some("location".to_string()),
        Some("payload".to_string()),
    );
    assert_eq!(panic.try_get_message(), &Some("message".to_string()));
    assert_eq!(panic.try_get_location(), &Some("location".to_string()));
    assert_eq!(panic.try_get_payload(), &Some("payload".to_string()));
}

#[tokio::test]
async fn from_join_error() {
    let handle: JoinHandle<()> = tokio::spawn(async {
        panic!("test panic");
    });
    let result: Result<(), JoinError> = handle.await;
    assert!(result.is_err());
    if let Err(join_error) = result {
        let is_panic: bool = PanicData::from_join_error(join_error)
            .try_get_message()
            .clone()
            .unwrap_or_default()
            .contains("test panic");
        assert!(is_panic);
    }
}
