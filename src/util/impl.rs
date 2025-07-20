use crate::*;

impl Wake for SimpleWaker {
    fn wake(self: Arc<Self>) {}
}
