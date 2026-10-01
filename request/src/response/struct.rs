use super::*;

/// Parsed HTTP response.
///
/// Value type with all fields public — no `Arc<RwLock>` wrapping, no
/// associated-type / boxed-trait indirection. Use the field accessors
/// directly (`response.status_code`, `response.body`, ...) or the
/// convenience helpers (`response.text()`, `response.is_success()`).
///
/// `HttpResponse` is `Clone` and `Debug`. Decoding is a method
/// ([`HttpResponse::decode`]) that returns a fresh `HttpResponse` with
/// the body expanded — original is untouched.
#[derive(Clone, Debug, Getter, Setter)]
pub struct HttpResponse {
    /// HTTP version (`HTTP/1.1`, `HTTP/2`, ...).
    pub version: HttpVersion,
    /// 3-digit numeric status code (`200`, `404`, ...).
    #[get(type(copy))]
    pub status_code: ResponseStatusCode,
    /// Reason phrase (`"OK"`, `"Not Found"`, ...).
    #[set(type(AsRef<str>))]
    pub reason_phrase: String,
    /// Response headers (single-value, lowercase keys).
    pub headers: HttpResponseHeaders,
    /// Raw response body bytes.
    #[set(type(AsRef<[u8]>))]
    pub body: ResponseBody,
}
