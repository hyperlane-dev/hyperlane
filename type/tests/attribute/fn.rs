use super::*;

#[tokio::test]
async fn get_panic_from_join_error() {
    let message: &'static str = "Test panic message";
    let join_handle: JoinHandle<()> = spawn(async {
        panic!("{}", message.to_string());
    });
    let join_error: JoinError = join_handle.await.unwrap_err();
    let panic_struct: PanicData = PanicData::from_join_error(join_error);
    assert!(!panic_struct.try_get_message().is_none());
    assert!(
        panic_struct
            .try_get_message()
            .clone()
            .unwrap_or_default()
            .contains(message)
    );
}
