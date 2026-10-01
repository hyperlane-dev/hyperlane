use super::*;

/// Fluent builder for [`HttpRequest`].
///
/// Single-purpose, chainable. Each method returns `&mut Self` for fluent
/// chaining; call [`RequestBuilder::build`] to obtain the final
/// [`HttpRequest`].
///
/// # Examples
///
/// ```ignore
/// use http_request::{RequestBuilder, Proxy};
///
/// // sync GET
/// let mut req = RequestBuilder::new()
///     .get("https://example.com/api")
///     .header("Accept", "application/json")
///     .timeout(5_000)
///     .build();
/// let _resp = req.send();
///
/// // async POST with JSON body
/// let req = RequestBuilder::new()
///     .post("https://example.com/api")
///     .body_json(&serde_json::json!({"k": "v"}))
///     .header("Content-Type", "application/json")
///     .proxy(Proxy::https("127.0.0.1", 7890).auth("u", "p"))
///     .build();
/// let _resp = req.send_async().await;
/// ```
#[derive(Clone, Data, Debug, Default)]
pub struct RequestBuilder {
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) request: HttpRequest,
}
