use super::*;

/// Represents an HTTP request, encapsulating various components such as the method, URL, protocol,
/// headers, body, and additional metadata.
#[derive(Clone, Debug)]
pub(crate) struct HttpRequest {
    /// HTTP request methods.
    pub(crate) methods: Arc<Method>,
    /// Target URL for the request.
    pub(crate) url: Arc<String>,
    /// HTTP request headers.
    pub(crate) header: Arc<RequestHeaders>,
    /// HTTP request body content.
    pub(crate) body: Arc<Body>,
    /// Request configuration settings.
    pub(crate) config: ArcRwLock<Config>,
    /// Temporary storage for request processing.
    pub(crate) tmp: ArcRwLock<Tmp>,
    /// Response storage for the request.
    pub(crate) response: ArcRwLock<HttpResponseBinary>,
}
