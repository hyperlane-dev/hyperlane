use super::*;

/// The accepted connection one accept loop is serving.
///
/// The listener, shutdown receiver and running-state sender are owned by one
/// accept loop. They are moved together and destructured when the loop starts,
/// so they stop being three more parameters repeated at every accept-loop entry
/// point. The loop drops the listener before it returns.
///
/// The two halves belong to two different `watch` channels: the receiver waits
/// on the shutdown signal, the sender publishes whether the loop is still
/// serving, and the other half of each channel stays with the server's control
/// hooks.
#[derive(Debug)]
pub(crate) struct AcceptSignal {
    /// The bound listener to accept connections from.
    pub(crate) listener: TcpListener,
    /// The shutdown signal the loop selects on.
    pub(crate) shutdown_receiver: Receiver<()>,
    /// Publishes whether the loop is still serving.
    pub(crate) running_sender: Sender<bool>,
}
