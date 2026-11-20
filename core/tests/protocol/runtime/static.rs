use super::*;

pub(crate) static URING_DROP_STATE: OnceLock<Arc<UringDropState>> = OnceLock::new();
