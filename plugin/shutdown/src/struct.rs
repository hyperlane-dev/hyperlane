/// The configuration of the shutdown plugin.
#[derive(Clone, Debug)]
pub struct ShutdownConfig {
    /// Whether the server also stops when the process is terminated.
    pub watch_terminate: bool,
    /// The message the default handler writes when the server stops.
    pub message: String,
}

/// The plugin that stops the server when the process is asked to.
#[derive(Clone, Copy, Debug, Default)]
pub struct ShutdownPlugin;
