#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Lifecycle {
    Abort(bool),
    Continue(bool),
}
