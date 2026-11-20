use super::*;

/// An indivisible group of kernel submission entries retained until SQ space is available.
pub(crate) enum UringSubmission {
    /// A standalone operation that has no linkage dependency.
    Single(squeue::Entry),
    /// A send and its dependent close, which must enter the SQ together.
    Linked([squeue::Entry; 2]),
}
