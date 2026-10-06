use super::*;

impl Lifetime for TestLifetimeStruct {
    unsafe fn leak(&self) -> &'static Self {
        let boxed: Box<Self> = Box::new(Self { value: self.value });
        Box::leak(boxed)
    }
    unsafe fn leak_mut(&self) -> &'static mut Self {
        let mut boxed: Box<Self> = Box::new(Self { value: self.value });
        let reference: *mut Self = ptr::addr_of_mut!(*boxed);
        mem::forget(boxed);
        unsafe { &mut *reference }
    }
}
