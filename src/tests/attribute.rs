use crate::*;

#[tokio::test]
async fn get_panic_from_context() {
    let ctx: Context = Context::default();
    let set_panic: Panic = Panic::new(
        Some("test".to_string()),
        Some("test".to_string()),
        Some("test".to_string()),
    );
    ctx.set_panic(set_panic.clone()).await;
    let get_panic: Panic = ctx.try_get_panic().await.unwrap();
    assert_eq!(set_panic, get_panic);
}

#[tokio::test]
async fn get_panic_from_join_error() {
    let message: &'static str = "Test panic message";
    let join_handle: JoinHandle<()> = spawn(async {
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

#[tokio::test]
async fn run_set_func() {
    let ctx: Context = Context::default();
    const KEY: &str = "string";
    const PARAM: &str = "test";
    let func: &(dyn Fn(&str) -> String + Send + Sync) = &|msg: &str| {
        return msg.to_string();
    };
    ctx.set_attribute(KEY, func).await;
    let get_key = ctx
        .try_get_attribute::<&(dyn Fn(&str) -> String + Send + Sync)>(KEY)
        .await
        .unwrap();
    assert_eq!(get_key(PARAM), func(PARAM));
}

#[tokio::test]
async fn send_body_hook() {
    let ctx: Context = Context::default();
    async fn test_send_body_hook_fn(ctx: Context) {
        let _ = ctx.send_body().await;
    }
    ctx.set_send_body_hook(test_send_body_hook_fn).await;
    assert!(ctx.try_get_send_body_hook().await.is_some());
}
