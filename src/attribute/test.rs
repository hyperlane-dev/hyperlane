use crate::*;

#[test]
fn get_panic_from_context() {
    let mut ctx: Context = Context::default();
    let set_panic: PanicData = PanicData::new(
        Some("test".to_string()),
        Some("test".to_string()),
        Some("test".to_string()),
    );
    ctx.set_task_panic(set_panic.clone());
    let get_panic: PanicData = ctx.try_get_task_panic_data().unwrap();
    assert_eq!(set_panic, get_panic);
}

#[test]
fn context_attributes() {
    let mut ctx: Context = Context::default();
    ctx.set_attribute("key1", "value1".to_string());
    let value: Option<String> = ctx.try_get_attribute("key1");
    assert_eq!(value, Some("value1".to_string()));
    ctx.remove_attribute("key1");
    let value: Option<String> = ctx.try_get_attribute("key1");
    assert_eq!(value, None);
    ctx.set_attribute("key2", 123);
    ctx.clear_attribute();
    let value: Option<i32> = ctx.try_get_attribute("key2");
    assert_eq!(value, None);
}

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

#[test]
fn run_set_func() {
    let mut ctx: Context = Context::default();
    const KEY: &str = "string";
    const PARAM: &str = "test";
    let func: &(dyn Fn(&str) -> String + Send + Sync) = &|msg: &str| msg.to_string();
    ctx.set_attribute(KEY, func);
    let get_key: &(dyn Fn(&str) -> String + Send + Sync) = ctx.try_get_attribute(KEY).unwrap();
    assert_eq!(get_key(PARAM), func(PARAM));
    let func: &(dyn Fn(&str) + Send + Sync) = &|msg: &str| {
        assert_eq!(msg, PARAM);
    };
    ctx.set_attribute(KEY, func);
    let hyperlane = ctx.get_attribute::<&(dyn Fn(&str) + Send + Sync)>(KEY);
    hyperlane(PARAM);
}
