/// The answer one request gets.
///
/// The control is a machine-ceiling reference, so it answers with the same
/// bodies as the server under test. A control that answered something else
/// would measure a different workload.
pub(crate) struct Answer {
    /// The body the answer carries.
    pub(crate) body: Vec<u8>,
}
