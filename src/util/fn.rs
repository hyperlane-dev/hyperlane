use crate::*;

pub fn sync_block_on<F, T>(future: F) -> T
where
    F: Future<Output = T>,
{
    let waker: Waker = Arc::new(SimpleWaker).into();
    let mut context: TaskContext = TaskContext::from_waker(&waker);
    let mut future: Pin<Box<F>> = Box::pin(future);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(result) => return result,
            Poll::Pending => {
                yield_now();
            }
        }
    }
}
