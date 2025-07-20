use crate::*;

pub fn sync_block_on<F, T>(future: F) -> T
where
    F: Future<Output = T>,
{
    block_in_place(|| Handle::current().block_on(future))
}
