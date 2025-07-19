use crate::*;

unsafe impl Send for PanicHook {}
unsafe impl Sync for PanicHook {}

impl PanicHook {
    const UNINITIALIZED: usize = 0;
    const INITIALIZING: usize = 1;
    const INITIALIZED: usize = 2;

    pub(crate) const fn new() -> Self {
        Self {
            error_hook: AtomicPtr::new(ptr::null_mut()),
            is_initialized: AtomicUsize::new(Self::UNINITIALIZED),
        }
    }

    pub(crate) fn set_error_hook(&self, handler: ArcErrorHandlerSendSync) {
        let boxed_hook: Box<ArcErrorHandlerSendSync> = Box::new(handler);
        let handler_ptr: *mut ArcErrorHandlerSendSync = Box::into_raw(boxed_hook);
        let old_ptr: *mut ArcErrorHandlerSendSync =
            self.get_error_hook().swap(handler_ptr, Ordering::AcqRel);
        if !old_ptr.is_null() {
            unsafe {
                let _: Box<ArcErrorHandlerSendSync> = Box::from_raw(old_ptr);
            }
        }
    }

    pub(crate) fn initialize_once(&self) {
        let is_ok: bool = self
            .get_is_initialized()
            .compare_exchange(
                Self::UNINITIALIZED,
                Self::INITIALIZING,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_ok();
        if is_ok {
            let hook_ref: &'static Self = unsafe { mem::transmute(self) };
            set_hook(Box::new(move |panic_info: &PanicHookInfo<'_>| {
                hook_ref.handle_panic(panic_info);
            }));
            self.get_is_initialized()
                .store(Self::INITIALIZED, Ordering::Release);
            return;
        }
        while self.get_is_initialized().load(Ordering::Acquire) != Self::INITIALIZED {
            hint::spin_loop();
        }
    }

    fn handle_panic(&self, panic_info: &PanicHookInfo<'_>) {
        let handler_ptr: *mut ArcErrorHandlerSendSync =
            self.get_error_hook().load(Ordering::Acquire);
        if handler_ptr.is_null() {
            self.default_panic_hook(panic_info);
            return;
        }
        let handler: &ArcErrorHandlerSendSync = unsafe { &*handler_ptr };
        let panic_info_struct: PanicInfo = PanicInfo::from_panic_hook_info(panic_info);
        let default_ctx: Context = Context::default();
        tokio::spawn(async move {
            let _ = default_ctx.set_panic_info(panic_info_struct).await;
            let handler_clone: ArcErrorHandlerSendSync = handler.clone();
            handler_clone(default_ctx).await;
        });
    }

    fn default_panic_hook(&self, panic_info: &PanicHookInfo<'_>) {
        let panic_info_struct: PanicInfo = PanicInfo::from_panic_hook_info(panic_info);
        let default_ctx: Context = Context::default();
        tokio::spawn(async move {
            let _ = default_ctx.set_panic_info(panic_info_struct).await;
            default_error_hook(default_ctx).await;
        });
    }
}

impl Drop for PanicHook {
    fn drop(&mut self) {
        let handler_ptr: *mut ArcErrorHandlerSendSync =
            self.get_error_hook().load(Ordering::Acquire);
        if !handler_ptr.is_null() {
            unsafe {
                let _: Box<ArcErrorHandlerSendSync> = Box::from_raw(handler_ptr);
            }
        }
    }
}
