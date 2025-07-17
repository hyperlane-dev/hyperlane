use crate::*;

tokio::task_local! {
    pub(crate) static REQUEST_CONTEXT: Context;
}
