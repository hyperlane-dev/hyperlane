use crate::*;

#[tokio::test]
async fn panic() {
    let ctx: Context = Context::default();
    let set_panic: Panic = Panic::new("test".to_string(), "test".to_string(), "test".to_string());
    ctx.set_panic(set_panic.clone()).await;
    let get_panic: Panic = ctx.get_panic().await.unwrap();
    assert_eq!(set_panic, get_panic);
}
