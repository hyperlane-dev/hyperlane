use crate::*;

/// Returns a static reference to a default Server instance.
///
/// # Returns
///
/// - `&'static Server` - A static reference to a lazily initialized default Server.
pub(crate) fn default_server() -> &'static Server {
    static DEFAULT_SERVER: OnceLock<Server> = OnceLock::new();
    DEFAULT_SERVER.get_or_init(Server::default)
}
