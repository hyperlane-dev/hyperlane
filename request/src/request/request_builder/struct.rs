use super::*;

/// Fluent builder for [`HttpRequest`].
///
/// Single-purpose, chainable. Each method returns `&mut Self` for fluent
/// chaining; call [`RequestBuilder::build`] to obtain the final
/// [`HttpRequest`].
///
/// # Examples
///
/// The built request can be sent with [`HttpRequest::send`] or
/// [`HttpRequest::send_async`]. This example inspects it without making a network request.
///
/// ```rust
/// use http_request::{Method, Proxy, RequestBuilder};
///
/// let get = RequestBuilder::new()
///     .get("https://example.com/api")
///     .header("Accept", "application/json")
///     .timeout(5_000)
///     .build();
/// assert_eq!(get.get_method(), Method::Get);
/// assert_eq!(get.get_config().get_timeout(), 5_000);
///
/// let post = RequestBuilder::new()
///     .post("https://example.com/api")
///     .body_json(&serde_json::json!({"key": "value"}))
///     .header("Content-Type", "application/json")
///     .proxy(Proxy::https("proxy.example.com", 7890))
///     .build();
/// assert_eq!(post.get_method(), Method::Post);
/// assert_eq!(post.get_body().as_slice(), br#"{"key":"value"}"#);
/// assert!(post.get_config().get_proxy().is_some());
/// ```
#[derive(Clone, Data, Debug, Default)]
pub struct RequestBuilder {
    pub(crate) request: HttpRequest,
}
