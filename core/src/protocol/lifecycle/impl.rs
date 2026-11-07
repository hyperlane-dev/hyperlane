use super::*;

/// Inherent implementation of [`ServerLifecycle`].
impl ServerLifecycle {
    /// Creates a lifecycle for one protocol request.
    ///
    /// # Arguments
    ///
    /// - `Arc<Server>` - The server whose pipeline serves the request.
    /// - `Request` - The request converted from the protocol head and body.
    /// - `String` - The complete original request target.
    ///
    pub(crate) fn new(server: Arc<Server>, request: Request, target: String) -> Self {
        Self {
            server,
            request,
            target,
        }
    }

    /// Runs the pipeline and returns once the handler and its writes finish.
    ///
    /// # Arguments
    ///
    /// - `Self` - The `stream` argument.
    /// - `Stream` - The stream the pipeline writes its response onto.
    ///
    pub(crate) async fn run(self, mut stream: Stream) {
        let mut ctx: Context = Context::default();
        self.get_server()
            .run_protocol_lifecycle(&mut stream, &mut ctx, self.get_request(), self.get_target())
            .await;
    }
}
