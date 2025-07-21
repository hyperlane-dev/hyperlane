use crate::*;

#[tokio::test]
async fn panic() {
    let ctx: Context = Context::default();
    let set_panic: Panic = Panic::new(
        Some("test".to_string()),
        Some("test".to_string()),
        Some("test".to_string()),
    );
    ctx.set_panic(set_panic.clone()).await;
    let get_panic: Panic = ctx.get_panic().await.unwrap();
    assert_eq!(set_panic, get_panic);
}

#[tokio::test]
async fn panic_from_join_error() {
    let message: &'static str = "Test panic message";
    let join_handle: JoinHandle<()> = tokio::spawn(async {
        panic!("{}", message.to_string());
    });
    let join_error: JoinError = join_handle.await.unwrap_err();
    let panic_struct: Panic = Panic::from_join_error(join_error);
    assert!(!panic_struct.get_message().is_none());
    assert!(
        panic_struct
            .get_message()
            .clone()
            .unwrap_or_default()
            .contains(message)
    );
}
