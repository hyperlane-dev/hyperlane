mod closed;
mod common;
mod context;
mod filter;
mod flush;
mod from_stream;
mod hook;
mod host;
mod hyperlane;
mod inject;
mod method;
mod referer;
mod reject;
mod request;
mod request_middleware;
mod response;
mod response_middleware;
mod route;
mod send;
mod stream;
mod upgrade;
mod version;

use {
    closed::*, common::*, context::*, filter::*, flush::*, from_stream::*, hook::*, host::*,
    hyperlane::*, inject::*, method::*, referer::*, reject::*, request::*, request_middleware::*,
    response::*, response_middleware::*, route::*, send::*, stream::*, upgrade::*, version::*,
};

use {
    proc_macro::TokenStream,
    proc_macro2::Span,
    quote::quote,
    syn::{
        Ident, Token,
        parse::{Parse, ParseStream, Parser, Result},
        punctuated::Punctuated,
        token::Comma,
        *,
    },
};

/// Wraps function body with WebSocket stream processing.
///
/// This attribute macro generates code that wraps the function body with a check to see if
/// data can be read from a WebSocket stream. The function body is only executed
/// if data is successfully read from the stream.
///
/// # Arguments
///
/// - `TokenStream`: Optional variable name to store the read request data.
/// - `TokenStream`: The function item to be modified
///
/// # Returns
///
/// Returns a TokenStream containing the modified function with WebSocket stream processing logic.
///
/// # Examples
///
/// Using no parameters:
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/is_ws_upgrade_type")]
/// struct Websocket;
///
/// impl ServerHook for Websocket {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[is_ws_upgrade_type]
///     #[try_get_websocket_request(body)]
///     async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
///         let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(&body);
///         stream.send_list(body_list).await;
///     }
/// }
/// ```
///
/// Using variable name to store request data:
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/is_ws_upgrade_type")]
/// struct Websocket;
///
/// impl ServerHook for Websocket {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[is_ws_upgrade_type]
///     #[try_get_websocket_request(request)]
///     async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
///         let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(&request);
///         stream.send_list(body_list).await;
///     }
/// }
///
/// impl Websocket {
///     #[try_get_websocket_request(request)]
///     async fn try_get_websocket_request_with_ref_self(&self, stream: &mut Stream, ctx: &mut Context) -> Status {}
/// }
///
/// #[try_get_websocket_request]
/// async fn standalone_try_get_websocket_request_handler(stream: &mut Stream, ctx: &mut Context) -> Status {}
/// ```
#[proc_macro_attribute]
pub fn try_get_websocket_request(attr: TokenStream, item: TokenStream) -> TokenStream {
    try_get_websocket_request_macro(attr, item)
}

/// Wraps function body with HTTP stream processing.
///
/// This attribute macro generates code that wraps the function body with a check to see if
/// data can be read from an HTTP stream. The function body is only executed
/// if data is successfully read from the stream.
///
/// # Arguments
///
/// - `TokenStream`: Optional variable name to store the read request data.
/// - `TokenStream`: The function item to be modified
///
/// # Returns
///
/// Returns a TokenStream containing the modified function with HTTP stream processing logic.
///
/// # Examples
///
/// Using no parameters:
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/try_get_http_request")]
/// struct HttpFromStreamTest;
///
/// impl ServerHook for HttpFromStreamTest {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[try_get_http_request]
///     async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {}
/// }
/// ```
///
/// Using with variable name:
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/try_get_http_request")]
/// struct HttpFromStreamTest;
///
/// impl ServerHook for HttpFromStreamTest {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[try_get_http_request(_request)]
///     async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {}
/// }
///
/// impl HttpFromStreamTest {
///     #[try_get_http_request(_request)]
///     async fn try_get_http_request_with_ref_self(&self, stream: &mut Stream, ctx: &mut Context) -> Status {}
/// }
///
/// #[try_get_http_request]
/// async fn standalone_try_get_http_request_handler(stream: &mut Stream, ctx: &mut Context) -> Status {}
/// ```
#[proc_macro_attribute]
pub fn try_get_http_request(attr: TokenStream, item: TokenStream) -> TokenStream {
    try_get_http_request_macro(attr, item)
}

/// Restricts function execution to HTTP GET requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the GET HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/is_get_method")]
/// struct Get;
///
/// impl ServerHook for Get {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(is_get_method, response_body("is_get_method"))]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl Get {
///     #[is_get_method]
///     async fn get_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[is_get_method]
/// async fn standalone_get_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn is_get_method(_attr: TokenStream, item: TokenStream) -> TokenStream {
    is_get_method_handler(item, Position::Prologue)
}

/// Restricts function execution to HTTP POST requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the POST HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/is_post_method")]
/// struct Post;
///
/// impl ServerHook for Post {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(is_post_method, response_body("is_post_method"))]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl Post {
///     #[is_post_method]
///     async fn post_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[is_post_method]
/// async fn standalone_post_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn is_post_method(_attr: TokenStream, item: TokenStream) -> TokenStream {
    is_post_method_handler(item, Position::Prologue)
}

/// Restricts function execution to HTTP PUT requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the PUT HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/is_put_method")]
/// struct Put;
///
/// impl ServerHook for Put {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(is_put_method, response_body("is_put_method"))]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl Put {
///     #[is_put_method]
///     async fn put_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[is_put_method]
/// async fn standalone_put_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn is_put_method(_attr: TokenStream, item: TokenStream) -> TokenStream {
    is_put_method_handler(item, Position::Prologue)
}

/// Restricts function execution to HTTP DELETE requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the DELETE HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/is_delete_method")]
/// struct Delete;
///
/// impl ServerHook for Delete {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(is_delete_method, response_body("is_delete_method"))]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl Delete {
///     #[is_delete_method]
///     async fn delete_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[is_delete_method]
/// async fn standalone_delete_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn is_delete_method(_attr: TokenStream, item: TokenStream) -> TokenStream {
    is_delete_method_handler(item, Position::Prologue)
}

/// Restricts function execution to HTTP PATCH requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the PATCH HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/is_patch_method")]
/// struct Patch;
///
/// impl ServerHook for Patch {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(is_patch_method, response_body("is_patch_method"))]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl Patch {
///     #[is_patch_method]
///     async fn patch_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[is_patch_method]
/// async fn standalone_patch_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn is_patch_method(_attr: TokenStream, item: TokenStream) -> TokenStream {
    is_patch_method_handler(item, Position::Prologue)
}

/// Restricts function execution to HTTP HEAD requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the HEAD HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/is_head_method")]
/// struct Head;
///
/// impl ServerHook for Head {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(is_head_method, response_body("is_head_method"))]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl Head {
///     #[is_head_method]
///     async fn head_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[is_head_method]
/// async fn standalone_head_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn is_head_method(_attr: TokenStream, item: TokenStream) -> TokenStream {
    is_head_method_handler(item, Position::Prologue)
}

/// Restricts function execution to HTTP OPTIONS requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the OPTIONS HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/is_options_method")]
/// struct Options;
///
/// impl ServerHook for Options {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(is_options_method, response_body("is_options_method"))]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl Options {
///     #[is_options_method]
///     async fn options_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[is_options_method]
/// async fn standalone_options_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn is_options_method(_attr: TokenStream, item: TokenStream) -> TokenStream {
    is_options_method_handler(item, Position::Prologue)
}

/// Restricts function execution to HTTP CONNECT requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the CONNECT HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/is_connect_method")]
/// struct Connect;
///
/// impl ServerHook for Connect {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(is_connect_method, response_body("is_connect_method"))]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl Connect {
///     #[is_connect_method]
///     async fn connect_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[is_connect_method]
/// async fn standalone_connect_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn is_connect_method(_attr: TokenStream, item: TokenStream) -> TokenStream {
    is_connect_method_handler(item, Position::Prologue)
}

/// Restricts function execution to HTTP TRACE requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses the TRACE HTTP method. Requests with other methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/is_trace_method")]
/// struct Trace;
///
/// impl ServerHook for Trace {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(is_trace_method, response_body("is_trace_method"))]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl Trace {
///     #[is_trace_method]
///     async fn trace_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[is_trace_method]
/// async fn standalone_trace_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn is_trace_method(_attr: TokenStream, item: TokenStream) -> TokenStream {
    is_trace_method_handler(item, Position::Prologue)
}

/// Restricts function execution to unknown HTTP methods only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses an HTTP method that is not one of the standard methods (GET, POST, PUT, DELETE, PATCH,
/// HEAD, OPTIONS, CONNECT, TRACE). Requests with standard methods will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/is_unknown_method")]
/// struct UnknownMethod;
///
/// impl ServerHook for UnknownMethod {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(
///         clear_response_headers,
///         filter(ctx.get_request().get_method().is_unknown()),
///         response_body("is_unknown_method")
///     )]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl UnknownMethod {
///     #[is_unknown_method]
///     async fn unknown_method_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[is_unknown_method]
/// async fn standalone_is_unknown_method_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn is_unknown_method(_attr: TokenStream, item: TokenStream) -> TokenStream {
    is_unknown_method_handler(item, Position::Prologue)
}

/// Allows function to handle multiple HTTP methods.
///
/// This attribute macro configures the decorated function to execute for any of the specified
/// HTTP methods. Methods should be provided as a comma-separated list.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/methods")]
/// struct GetPost;
///
/// impl ServerHook for GetPost {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(
///         is_http_version,
///         methods(get, post),
///         response_body("methods")
///     )]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl GetPost {
///     #[methods(get, post)]
///     async fn methods_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[methods(get, post)]
/// async fn standalone_methods_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a comma-separated list of HTTP method names (lowercase) and should be
/// applied to async functions that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn methods(attr: TokenStream, item: TokenStream) -> TokenStream {
    methods_macro(attr, item, Position::Prologue)
}

/// Restricts function execution to HTTP/0.9 requests only.
///
/// This attribute macro ensures the decorated function only executes for HTTP/0.9
/// protocol requests, the earliest version of the HTTP protocol.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/is_http0_9_version")]
/// struct Http09;
///
/// impl ServerHook for Http09 {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(is_http0_9_version, response_body("is_http0_9_version"))]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl Http09 {
///     #[is_http0_9_version]
///     async fn http0_9_version_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[is_http0_9_version]
/// async fn standalone_http0_9_version_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn is_http0_9_version(_attr: TokenStream, item: TokenStream) -> TokenStream {
    is_http0_9_version_macro(item, Position::Prologue)
}

/// Restricts function execution to HTTP/1.0 requests only.
///
/// This attribute macro ensures the decorated function only executes for HTTP/1.0
/// protocol requests.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/is_http1_0_version")]
/// struct Http10;
///
/// impl ServerHook for Http10 {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(is_http1_0_version, response_body("is_http1_0_version"))]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl Http10 {
///     #[is_http1_0_version]
///     async fn http1_0_version_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[is_http1_0_version]
/// async fn standalone_http1_0_version_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn is_http1_0_version(_attr: TokenStream, item: TokenStream) -> TokenStream {
    is_http1_0_version_macro(item, Position::Prologue)
}

/// Restricts function execution to HTTP/1.1 requests only.
///
/// This attribute macro ensures the decorated function only executes for HTTP/1.1
/// protocol requests.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/is_http1_1_version")]
/// struct Http11;
///
/// impl ServerHook for Http11 {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(is_http1_1_version, response_body("is_http1_1_version"))]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl Http11 {
///     #[is_http1_1_version]
///     async fn http1_1_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[is_http1_1_version]
/// async fn standalone_http1_1_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn is_http1_1_version(_attr: TokenStream, item: TokenStream) -> TokenStream {
    is_http1_1_version_macro(item, Position::Prologue)
}

/// Restricts function execution to HTTP/2 requests only.
///
/// This attribute macro ensures the decorated function only executes for HTTP/2
/// protocol requests.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/is_http2_version")]
/// struct Http2;
///
/// impl ServerHook for Http2 {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(is_http2_version, response_body("is_http2_version"))]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl Http2 {
///     #[is_http2_version]
///     async fn http2_version_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[is_http2_version]
/// async fn standalone_http2_version_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn is_http2_version(_attr: TokenStream, item: TokenStream) -> TokenStream {
    is_http2_version_macro(item, Position::Prologue)
}

/// Restricts function execution to HTTP/3 requests only.
///
/// This attribute macro ensures the decorated function only executes for HTTP/3
/// protocol requests, the latest version of the HTTP protocol.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/is_http3_version")]
/// struct Http3;
///
/// impl ServerHook for Http3 {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(is_http3_version, response_body("is_http3_version"))]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl Http3 {
///     #[is_http3_version]
///     async fn http3_version_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[is_http3_version]
/// async fn standalone_http3_version_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn is_http3_version(_attr: TokenStream, item: TokenStream) -> TokenStream {
    is_http3_version_macro(item, Position::Prologue)
}

/// Restricts function execution to HTTP/1.1 or higher protocol versions.
///
/// This attribute macro ensures the decorated function only executes for HTTP/1.1
/// or newer protocol versions, including HTTP/2, HTTP/3, and future versions.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/is_http1_1_or_higher_version")]
/// struct Http11OrHigher;
///
/// impl ServerHook for Http11OrHigher {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(is_http1_1_or_higher_version, response_body("is_http1_1_or_higher_version"))]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl Http11OrHigher {
///     #[is_http1_1_or_higher_version]
///     async fn http1_1_or_higher_version_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[is_http1_1_or_higher_version]
/// async fn standalone_http1_1_or_higher_version_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn is_http1_1_or_higher_version(_attr: TokenStream, item: TokenStream) -> TokenStream {
    is_http1_1_or_higher_version_macro(item, Position::Prologue)
}

/// Restricts function execution to standard HTTP requests only.
///
/// This attribute macro ensures the decorated function only executes for standard HTTP requests,
/// excluding WebSocket upgrades and other protocol upgrade requests.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/http")]
/// struct HttpOnly;
///
/// impl ServerHook for HttpOnly {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(is_http_version, response_body("http"))]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl HttpOnly {
///     #[is_http_version]
///     async fn http_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[is_http_version]
/// async fn standalone_http_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn is_http_version(_attr: TokenStream, item: TokenStream) -> TokenStream {
    is_http_version_macro(item, Position::Prologue)
}

/// Restricts function execution to requests with unknown HTTP versions only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses an unrecognized or non-standard HTTP version (not HTTP/0.9, HTTP/1.0, HTTP/1.1, HTTP/2, or HTTP/3).
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/is_unknown_version")]
/// struct UnknownVersionHandler;
///
/// impl ServerHook for UnknownVersionHandler {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(is_unknown_version, response_body("unknown version"))]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl UnknownVersionHandler {
///     #[is_unknown_version]
///     async fn handle_unknown_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[is_unknown_version]
/// async fn standalone_unknown_version_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn is_unknown_version(_attr: TokenStream, item: TokenStream) -> TokenStream {
    is_unknown_version_macro(item, Position::Prologue)
}

/// Restricts function execution to WebSocket upgrade requests only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// is a valid WebSocket upgrade request with proper request headers and protocol negotiation.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/is_ws_upgrade_type")]
/// struct Websocket;
///
/// impl ServerHook for Websocket {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[is_ws_upgrade_type]
///     #[try_get_websocket_request(body)]
///     async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
///         let body_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(&body);
///         stream.send_list(body_list).await;
///     }
/// }
///
/// impl Websocket {
///     #[is_ws_upgrade_type]
///     async fn ws_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[is_ws_upgrade_type]
/// async fn standalone_ws_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn is_ws_upgrade_type(_attr: TokenStream, item: TokenStream) -> TokenStream {
    is_ws_upgrade_type_macro(item, Position::Prologue)
}

/// Restricts function execution to HTTP/2 Cleartext (is_h2c_upgrade_type) requests only.
///
/// This attribute macro ensures the decorated function only executes for HTTP/2 cleartext
/// requests that use the is_h2c_upgrade_type upgrade mechanism.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/is_h2c_upgrade_type")]
/// struct H2c;
///
/// impl ServerHook for H2c {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(is_h2c_upgrade_type, response_body("is_h2c_upgrade_type"))]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl H2c {
///     #[is_h2c_upgrade_type]
///     async fn h2c_upgrade_type_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[is_h2c_upgrade_type]
/// async fn standalone_h2c_upgrade_type_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn is_h2c_upgrade_type(_attr: TokenStream, item: TokenStream) -> TokenStream {
    is_h2c_upgrade_type_macro(item, Position::Prologue)
}

/// Restricts function execution to TLS-encrypted requests only.
///
/// This attribute macro ensures the decorated function only executes for requests
/// that use TLS/SSL encryption on the connection.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/is_tls_upgrade_type")]
/// struct Tls;
///
/// impl ServerHook for Tls {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(is_tls_upgrade_type, response_body("is_tls_upgrade_type"))]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl Tls {
///     #[is_tls_upgrade_type]
///     async fn tls_upgrade_type_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[is_tls_upgrade_type]
/// async fn standalone_tls_upgrade_type_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn is_tls_upgrade_type(_attr: TokenStream, item: TokenStream) -> TokenStream {
    is_tls_upgrade_type_macro(item, Position::Prologue)
}

/// Restricts function execution to requests with unknown protocol upgrade types only.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// uses an unrecognized or non-standard protocol upgrade type (not WebSocket, h2c, or TLS).
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/is_unknown_upgrade_type")]
/// struct UnknownUpgrade;
///
/// impl ServerHook for UnknownUpgrade {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(is_unknown_upgrade_type, response_body("unknown upgrade type"))]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl UnknownUpgrade {
///     #[is_unknown_upgrade_type]
///     async fn unknown_upgrade_type_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[is_unknown_upgrade_type]
/// async fn standalone_unknown_upgrade_type_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn is_unknown_upgrade_type(_attr: TokenStream, item: TokenStream) -> TokenStream {
    is_unknown_upgrade_type_macro(item, Position::Prologue)
}

/// Sets the HTTP status code for the response.
///
/// This attribute macro configures the HTTP status code that will be sent with the response.
/// The status code can be provided as a numeric literal or a global constant.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// const CUSTOM_STATUS_CODE: i32 = 200;
///
/// #[route("/response_status_code")]
/// struct ResponseStatusCode;
///
/// impl ServerHook for ResponseStatusCode {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_status_code(CUSTOM_STATUS_CODE)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl ResponseStatusCode {
///     #[response_status_code(CUSTOM_STATUS_CODE)]
///     async fn response_status_code_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[response_status_code(200)]
/// async fn standalone_response_status_code_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a numeric HTTP status code or a global constant
/// and should be applied to async functions that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn response_status_code(attr: TokenStream, item: TokenStream) -> TokenStream {
    response_status_code_macro(attr, item, Position::Prologue)
}

/// Sets the HTTP reason phrase for the response.
///
/// This attribute macro configures the HTTP reason phrase that accompanies the status code.
/// The reason phrase can be provided as a string literal or a global constant.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// const CUSTOM_REASON: &str = "Accepted";
///
/// #[route("/response_reason")]
/// struct ResponseReason;
///
/// impl ServerHook for ResponseReason {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_reason_phrase(CUSTOM_REASON)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl ResponseReason {
///     #[response_reason_phrase(CUSTOM_REASON)]
///     async fn response_reason_phrase_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[response_reason_phrase("OK")]
/// async fn standalone_response_reason_phrase_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a string literal or global constant for the reason phrase and should be
/// applied to async functions that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn response_reason_phrase(attr: TokenStream, item: TokenStream) -> TokenStream {
    response_reason_phrase_macro(attr, item, Position::Prologue)
}

/// Sets or replaces a specific HTTP response header.
///
/// This attribute macro configures a specific HTTP response header that will be sent with the response.
/// Both the header name and value can be provided as string literals or global constants.
/// Use `"key", "value"` to set a header (add to existing headers) or `"key" => "value"` to replace a header (overwrite existing).
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// const CUSTOM_HEADER_NAME: &str = "X-Custom-Header";
/// const CUSTOM_HEADER_VALUE: &str = "custom-value";
///
/// #[route("/response_header")]
/// struct ResponseHeader;
///
/// impl ServerHook for ResponseHeader {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_header(CUSTOM_HEADER_NAME => CUSTOM_HEADER_VALUE)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl ResponseHeader {
///     #[response_header(CUSTOM_HEADER_NAME => CUSTOM_HEADER_VALUE)]
///     async fn response_header_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[route("/response_header")]
/// struct ResponseHeaderTest;
///
/// impl ServerHook for ResponseHeaderTest {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body("Testing header set and replace operations")]
///     #[response_header("X-Add-Header", "add-value")]
///     #[response_header("X-Set-Header" => "set-value")]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[response_header("X-Custom" => "value")]
/// async fn standalone_response_header_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts header name and header value, both can be string literals or global constants.
/// Use `"key", "value"` for setting headers and `"key" => "value"` for replacing headers.
/// Should be applied to async functions that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn response_header(attr: TokenStream, item: TokenStream) -> TokenStream {
    response_header_macro(attr, item, Position::Prologue)
}

/// Sets the HTTP response body.
///
/// This attribute macro configures the HTTP response body that will be sent with the response.
/// The body content can be provided as a string literal or a global constant.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// const RESPONSE_DATA: &str = "{\"status\": \"success\"}";
///
/// #[route("/response_body")]
/// struct ResponseBody;
///
/// impl ServerHook for ResponseBody {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&RESPONSE_DATA)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl ResponseBody {
///     #[response_body(&RESPONSE_DATA)]
///     async fn response_body_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[response_body("standalone response body")]
/// async fn standalone_response_body_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a string literal or global constant for the response body and should be
/// applied to async functions that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn response_body(attr: TokenStream, item: TokenStream) -> TokenStream {
    response_body_macro(attr, item, Position::Prologue)
}

/// Clears all response headers.
///
/// This attribute macro clears all response headers from the response.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/clear_response_headers")]
/// struct ClearResponseHeaders;
///
/// impl ServerHook for ClearResponseHeaders {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(
///         clear_response_headers,
///         filter(ctx.get_request().get_method().is_unknown()),
///         response_body("clear_response_headers")
///     )]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl ClearResponseHeaders {
///     #[clear_response_headers]
///     async fn clear_response_headers_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[clear_response_headers]
/// async fn standalone_clear_response_headers_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro should be applied to async functions that accept a `&mut Context` parameter.   
#[proc_macro_attribute]
pub fn clear_response_headers(_attr: TokenStream, item: TokenStream) -> TokenStream {
    clear_response_headers_macro(item, Position::Prologue)
}

/// Sets the HTTP response version.
///
/// This attribute macro configures the HTTP response version that will be sent with the response.
/// The version can be provided as a variable or code block.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[request_middleware]
/// struct RequestMiddleware;
///
/// impl ServerHook for RequestMiddleware {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[epilogue_macros(
///         response_status_code(200),
///         response_version(HttpVersion::Http1_1),
///         response_header(SERVER => HYPERLANE)
///     )]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl RequestMiddleware {
///     #[response_version(HttpVersion::Http2)]
///     async fn response_version_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[response_version(HttpVersion::Http1_0)]
/// async fn standalone_response_version_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a variable or code block for the response version and should be
/// applied to async functions that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn response_version(attr: TokenStream, item: TokenStream) -> TokenStream {
    response_version_macro(attr, item, Position::Prologue)
}

/// Handles closed connection scenarios.
///
/// This attribute macro configures the function to handle cases where the connection
/// has been closed, providing appropriate handling for terminated or disconnected connections.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/closed")]
/// struct ClosedTest;
///
/// impl ServerHook for ClosedTest {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[closed]
///     async fn handle(self, stream: &mut Stream, _: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl ClosedTest {
///     #[closed]
///     async fn closed_with_ref_self(&self, stream: &mut Stream, _: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[closed]
/// async fn standalone_closed_handler(stream: &mut Stream, _: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn closed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    closed_macro(item, Position::Prologue)
}

/// Filters requests based on a boolean condition.
///
/// The function continues execution only if the provided code block returns `true`.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/is_unknown_method")]
/// struct UnknownMethod;
///
/// impl ServerHook for UnknownMethod {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(
///         filter(ctx.get_request().get_method().is_unknown()),
///         response_body("is_unknown_method")
///     )]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl UnknownMethod {
///     #[filter(true)]
///     async fn filter_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[filter(true)]
/// async fn standalone_filter_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
#[proc_macro_attribute]
pub fn filter(attr: TokenStream, item: TokenStream) -> TokenStream {
    filter_macro(attr, item, Position::Prologue)
}

/// Rejects requests based on a boolean condition.
///
/// The function continues execution only if the provided code block returns `false`.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[response_middleware(2)]
/// struct ResponseMiddleware2;
///
/// impl ServerHook for ResponseMiddleware2 {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(
///         reject(ctx.get_request().get_upgrade_type().is_ws())
///     )]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl ResponseMiddleware2 {
///     #[reject(false)]
///     async fn reject_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[reject(false)]
/// async fn standalone_reject_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
#[proc_macro_attribute]
pub fn reject(attr: TokenStream, item: TokenStream) -> TokenStream {
    reject_macro(attr, item, Position::Prologue)
}

/// Restricts function execution to requests with a specific host.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// has a host header that matches the specified value. Requests with different or missing host headers will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/host")]
/// struct Host;
///
/// impl ServerHook for Host {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[host("localhost")]
///     #[prologue_macros(response_body("host string literal: localhost"), send)]
///     async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl Host {
///     #[host("localhost")]
///     async fn host_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[host("localhost")]
/// async fn standalone_host_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a string literal specifying the expected host value and should be
/// applied to async functions that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn host(attr: TokenStream, item: TokenStream) -> TokenStream {
    host_macro(attr, item, Position::Prologue)
}

/// Reject requests that have no host header.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// has a host header present. Requests without a host header will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/reject_host")]
/// struct RejectHost;
///
/// impl ServerHook for RejectHost {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(
///         reject_host("filter.localhost"),
///         response_body("host filter string literal")
///     )]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl RejectHost {
///     #[reject_host("filter.localhost")]
///     async fn reject_host_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[reject_host("filter.localhost")]
/// async fn standalone_reject_host_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn reject_host(attr: TokenStream, item: TokenStream) -> TokenStream {
    reject_host_macro(attr, item, Position::Prologue)
}

/// Restricts function execution to requests with a specific referer.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// has a referer header that matches the specified value. Requests with different or missing referer headers will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/referer")]
/// struct Referer;
///
/// impl ServerHook for Referer {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(
///         referer("http://localhost"),
///         response_body("referer string literal: http://localhost")
///     )]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl Referer {
///     #[referer("http://localhost")]
///     async fn referer_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[referer("http://localhost")]
/// async fn standalone_referer_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a string literal specifying the expected referer value and should be
/// applied to async functions that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn referer(attr: TokenStream, item: TokenStream) -> TokenStream {
    referer_macro(attr, item, Position::Prologue)
}

/// Reject requests that have a specific referer header.
///
/// This attribute macro ensures the decorated function only executes when the incoming request
/// does not have a referer header that matches the specified value. Requests with the matching referer header will be filtered out.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/reject_referer")]
/// struct RejectReferer;
///
/// impl ServerHook for RejectReferer {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(
///         reject_referer("http://localhost"),
///         response_body("referer filter string literal")
///     )]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl RejectReferer {
///     #[reject_referer("http://localhost")]
///     async fn reject_referer_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[reject_referer("http://localhost")]
/// async fn standalone_reject_referer_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a string literal specifying the referer value to filter out and should be
/// applied to async functions that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn reject_referer(attr: TokenStream, item: TokenStream) -> TokenStream {
    reject_referer_macro(attr, item, Position::Prologue)
}

/// Executes multiple specified functions before the main handler function.
///
/// This attribute macro configures multiple pre-execution hooks that run before the main function logic.
/// The specified hook functions will be called in the order provided, followed by the main function execution.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// struct PrologueHooks;
///
/// impl ServerHook for PrologueHooks {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[is_get_method]
///     #[is_http_version]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// async fn prologue_hooks_fn(stream: &mut Stream, ctx: &mut Context) {
///     let hook = PrologueHooks::new(stream, ctx).await;
///     hook.handle(stream, ctx).await;
/// }
///
/// #[route("/hook")]
/// struct Hook;
///
/// impl ServerHook for Hook {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_hooks(prologue_hooks_fn)]
///     #[response_body("Testing hook macro")]
///     async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
///
/// The macro accepts a comma-separated list of function names as parameters. All hook functions
/// and the main function must accept a `Context` parameter. Avoid combining this macro with other
/// macros on the same function to prevent macro expansion conflicts.
///
/// # Advanced Usage with Method Expressions
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/hooks_expression")]
/// struct HooksExpression;
///
/// impl ServerHook for HooksExpression {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[is_get_method]
///     #[prologue_hooks(HooksExpression::new_hook, HooksExpression::method_hook)]
///     #[response_body("hooks expression test")]
///     async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl HooksExpression {
///     async fn new_hook(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
///
///     async fn method_hook(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
#[proc_macro_attribute]
pub fn prologue_hooks(attr: TokenStream, item: TokenStream) -> TokenStream {
    prologue_hooks_macro(attr, item, Position::Prologue)
}

/// Executes multiple specified functions after the main handler function.
///
/// This attribute macro configures multiple post-execution hooks that run after the main function logic.
/// The main function will execute first, followed by the specified hook functions in the order provided.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// struct EpilogueHooks;
///
/// impl ServerHook for EpilogueHooks {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_status_code(200)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// async fn epilogue_hooks_fn(stream: &mut Stream, ctx: &mut Context) -> Status {
///     let hook = EpilogueHooks::new(stream, ctx).await;
///     hook.handle(stream, ctx).await
/// }
///
/// #[route("/hook")]
/// struct Hook;
///
/// impl ServerHook for Hook {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[epilogue_hooks(epilogue_hooks_fn)]
///     #[response_body("Testing hook macro")]
///     async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// ```
///
/// The macro accepts a comma-separated list of function names as parameters. All hook functions
/// and the main function must accept a `Context` parameter. Avoid combining this macro with other
/// macros on the same function to prevent macro expansion conflicts.
///
/// # Advanced Usage with Method Expressions
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/hooks_expression")]
/// struct HooksExpression;
///
/// impl ServerHook for HooksExpression {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[is_get_method]
///     #[epilogue_hooks(HooksExpression::new_hook, HooksExpression::method_hook)]
///     #[response_body("hooks expression test")]
///     async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl HooksExpression {
///     async fn new_hook(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
///
///     async fn method_hook(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
#[proc_macro_attribute]
pub fn epilogue_hooks(attr: TokenStream, item: TokenStream) -> TokenStream {
    epilogue_hooks_macro(attr, item, Position::Epilogue)
}

/// Extracts the raw request body into a specified variable.
///
/// This attribute macro extracts the raw request body content into a variable
/// with the fixed type `RequestBody`. The body content is not parsed or deserialized.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/request_body")]
/// struct RequestBodyRoute;
///
/// impl ServerHook for RequestBodyRoute {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("raw body: {raw_body:?}"))]
///     #[request_body(raw_body)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl RequestBodyRoute {
///     #[request_body(raw_body)]
///     async fn request_body_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[request_body(raw_body)]
/// async fn standalone_request_body_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// # Multi-Parameter Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/multi_body")]
/// struct MultiBody;
///
/// impl ServerHook for MultiBody {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("bodies: {body1:?}, {body2:?}"))]
///     #[request_body(body1, body2)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
///
/// The macro accepts one or more variable names separated by commas.
/// Each variable will be available in the function scope as a `RequestBody` type.
#[proc_macro_attribute]
pub fn request_body(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_body_macro(attr, item, Position::Prologue)
}

/// Parses the request body as JSON into a specified variable and type with panic on parsing failure.
///
/// This attribute macro extracts and deserializes the request body content as JSON into a variable
/// with the specified type. The body content is parsed as JSON using serde.
/// If the request body does not exist or JSON parsing fails, the function will panic with an error message.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Clone, Debug, Deserialize, Serialize)]
/// struct TestData {
///     name: String,
///     age: u32,
/// }
///
/// #[route("/request_body_json_result")]
/// struct RequestBodyJson;
///
/// impl ServerHook for RequestBodyJson {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("request data: {request_data_result:?}"))]
///     #[request_body_json_result(request_data_result: TestData)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl RequestBodyJson {
///     #[request_body_json_result(request_data_result: TestData)]
///     async fn request_body_json_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[request_body_json_result(request_data_result: TestData)]
/// async fn standalone_request_body_json_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// # Multi-Parameter Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Clone, Debug, Deserialize, Serialize)]
/// struct User {
///     name: String,
/// }
///
/// #[derive(Clone, Debug, Deserialize, Serialize)]
/// struct Config {
///     debug: bool,
/// }
///
/// #[route("/request_body_json_result")]
/// struct TestData;
///
/// impl ServerHook for TestData {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("user: {user:?}, config: {config:?}"))]
///     #[request_body_json_result(user: User, config: Config)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
///
/// The macro accepts one or more `variable_name: Type` pairs separated by commas.
/// Each variable will be available in the function scope as a `Result<Type, serde_json::Error>`.
#[proc_macro_attribute]
pub fn request_body_json_result(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_body_json_result_macro(attr, item, Position::Prologue)
}

/// Parses the request body as JSON into a specified variable and type with panic on parsing failure.
///
/// This attribute macro extracts and deserializes the request body content as JSON into a variable
/// with the specified type. The body content is parsed as JSON using serde.
/// If the request body does not exist or JSON parsing fails, the function will panic with an error message.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Clone, Debug, Deserialize, Serialize)]
/// struct TestData {
///     name: String,
///     age: u32,
/// }
///
/// #[route("/request_body_json")]
/// struct RequestBodyJson;
///
/// impl ServerHook for RequestBodyJson {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("request data: {request_data_result:?}"))]
///     #[request_body_json(request_data_result: TestData)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl RequestBodyJson {
///     #[request_body_json(request_data_result: TestData)]
///     async fn request_body_json_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[request_body_json(request_data_result: TestData)]
/// async fn standalone_request_body_json_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// # Multi-Parameter Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Clone, Debug, Deserialize, Serialize)]
/// struct User {
///     name: String,
/// }
///
/// #[derive(Clone, Debug, Deserialize, Serialize)]
/// struct Config {
///     debug: bool,
/// }
///
/// #[route("/request_body_json")]
/// struct TestData;
///
/// impl ServerHook for TestData {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("user: {user:?}, config: {config:?}"))]
///     #[request_body_json(user: User, config: Config)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
///
/// The macro accepts one or more `variable_name: Type` pairs separated by commas.
/// Each variable will be available in the function scope as a `Result<Type, serde_json::Error>`.
///
/// # Panics
///
/// This macro will panic if the request body does not exist or JSON parsing fails.
#[proc_macro_attribute]
pub fn request_body_json(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_body_json_macro(attr, item, Position::Prologue)
}

/// Extracts a specific attribute value into a variable wrapped in Option type.
///
/// This attribute macro retrieves a specific attribute by key and makes it available
/// as a typed Option variable from the request context. The extracted value is wrapped
/// in an Option type to safely handle cases where the attribute may not exist.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
/// use serde::{Deserialize, Serialize};
///
/// const TEST_ATTRIBUTE_KEY: &str = "test_attribute_key";
///
/// #[derive(Clone, Debug, Deserialize, Serialize)]
/// struct TestData {
///     name: String,
///     age: u32,
/// }
///
/// #[route("/try_get_attribute")]
/// struct Attribute;
///
/// impl ServerHook for Attribute {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("request attribute: {request_try_get_attribute:?}"))]
///     #[try_get_attribute(TEST_ATTRIBUTE_KEY => request_try_get_attribute: TestData)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl Attribute {
///     #[try_get_attribute(TEST_ATTRIBUTE_KEY => request_try_get_attribute: TestData)]
///     async fn attribute_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[try_get_attribute(TEST_ATTRIBUTE_KEY => request_try_get_attribute: TestData)]
/// async fn standalone_attribute_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a key-to-variable mapping in the format `key => variable_name: Type`.
/// The variable will be available as an `Option<Type>` in the function scope.
///
/// # Multi-Parameter Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/try_get_attribute")]
/// struct MultiAttr;
///
/// impl ServerHook for MultiAttr {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("attrs: {attr1:?}, {attr2:?}"))]
///     #[try_get_attribute("key1" => attr1: String, "key2" => attr2: i32)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
///
/// The macro accepts multiple `key => variable_name: Type` tuples separated by commas.
#[proc_macro_attribute]
pub fn try_get_attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
    try_get_attribute_macro(attr, item, Position::Prologue)
}

/// Extracts a specific attribute value into a variable with panic on missing value.
///
/// This attribute macro retrieves a specific attribute by key and makes it available
/// as a typed variable from the request context. If the attribute does not exist,
/// the function will panic with an error message indicating the missing attribute.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
/// use serde::{Deserialize, Serialize};
///
/// const TEST_ATTRIBUTE_KEY: &str = "test_attribute_key";
///
/// #[derive(Clone, Debug, Deserialize, Serialize)]
/// struct TestData {
///     name: String,
///     age: u32,
/// }
///
/// #[route("/attribute")]
/// struct Attribute;
///
/// impl ServerHook for Attribute {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("request attribute: {request_attribute:?}"))]
///     #[attribute(TEST_ATTRIBUTE_KEY => request_attribute: TestData)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl Attribute {
///     #[attribute(TEST_ATTRIBUTE_KEY => request_attribute: TestData)]
///     async fn attribute_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[attribute(TEST_ATTRIBUTE_KEY => request_attribute: TestData)]
/// async fn standalone_attribute_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a key-to-variable mapping in the format `key => variable_name: Type`.
/// The variable will be available as an `Type` in the function scope.
///
/// # Multi-Parameter Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/attribute")]
/// struct MultiAttr;
///
/// impl ServerHook for MultiAttr {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("attrs: {attr1}, {attr2}"))]
///     #[attribute("key1" => attr1: String, "key2" => attr2: i32)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
///
/// The macro accepts multiple `key => variable_name: Type` tuples separated by commas.
///
/// # Panics
///
/// This macro will panic if the requested attribute does not exist in the request context.
#[proc_macro_attribute]
pub fn attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
    attribute_macro(attr, item, Position::Prologue)
}

/// Extracts all attributes into a ThreadSafeAttributeStore variable.
///
/// This attribute macro retrieves all available attributes from the request context
/// and makes them available as a ThreadSafeAttributeStore for comprehensive attribute access.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/attributes")]
/// struct Attributes;
///
/// impl ServerHook for Attributes {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("request attributes: {request_attributes:?}"))]
///     #[attributes(request_attributes)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl Attributes {
///     #[attributes(request_attributes)]
///     async fn attributes_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[attributes(request_attributes)]
/// async fn standalone_attributes_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a variable name that will contain a HashMap of all attributes.
/// The variable will be available as a HashMap in the function scope.
///
/// # Multi-Parameter Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/multi_attrs")]
/// struct MultiAttrs;
///
/// impl ServerHook for MultiAttrs {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("attrs1: {attrs1:?}, attrs2: {attrs2:?}"))]
///     #[attributes(attrs1, attrs2)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
///
/// The macro accepts multiple variable names separated by commas.
#[proc_macro_attribute]
pub fn attributes(attr: TokenStream, item: TokenStream) -> TokenStream {
    attributes_macro(attr, item, Position::Prologue)
}

/// Extracts panic data into a variable wrapped in Option type.
///
/// This attribute macro retrieves panic information if a panic occurred during handling
/// and makes it available as an Option variable. The extracted value is wrapped
/// in an Option type to safely handle cases where no panic occurred.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/try_get_task_panic_data")]
/// struct PanicDataOptionTest;
///
/// impl ServerHook for PanicDataOptionTest {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("Panic data: {try_get_task_panic_data:?}"))]
///     #[try_get_task_panic_data(try_get_task_panic_data)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl PanicDataOptionTest {
///     #[try_get_task_panic_data(try_get_task_panic_data)]
///     async fn try_get_task_panic_data_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[try_get_task_panic_data(try_get_task_panic_data)]
/// async fn standalone_try_get_task_panic_data_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a variable name that will contain the panic data.
/// The variable will be available as an `Option<PanicData>` in the function scope.
///
/// # Multi-Parameter Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/try_get_task_panic_data")]
/// struct MultiPanicDataOption;
///
/// impl ServerHook for MultiPanicDataOption {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("panic1: {panic1:?}, panic2: {panic2:?}"))]
///     #[try_get_task_panic_data(panic1, panic2)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
///
/// The macro accepts multiple variable names separated by commas.
#[proc_macro_attribute]
pub fn try_get_task_panic_data(attr: TokenStream, item: TokenStream) -> TokenStream {
    try_get_task_panic_data_macro(attr, item, Position::Prologue)
}

/// Extracts panic data into a variable with panic on missing value.
///
/// This attribute macro retrieves panic information if a panic occurred during handling
/// and makes it available as a variable. If no panic data exists,
/// the function will panic with an error message.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/task_panic_data")]
/// struct PanicDataTest;
///
/// impl ServerHook for PanicDataTest {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("Panic data: {task_panic_data}"))]
///     #[task_panic_data(task_panic_data)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl PanicDataTest {
///     #[task_panic_data(task_panic_data)]
///     async fn task_panic_data_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[task_panic_data(task_panic_data)]
/// async fn standalone_task_panic_data_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a variable name that will contain the panic data.
/// The variable will be available as a `PanicData` in the function scope.
///
/// # Multi-Parameter Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/task_panic_data")]
/// struct MultiPanicData;
///
/// impl ServerHook for MultiPanicData {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("panic1: {panic1}, panic2: {panic2}"))]
///     #[task_panic_data(panic1, panic2)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
///
/// The macro accepts multiple variable names separated by commas.
///
/// # Panics
///
/// This macro will panic if no panic data exists in the request context.
#[proc_macro_attribute]
pub fn task_panic_data(attr: TokenStream, item: TokenStream) -> TokenStream {
    task_panic_data_macro(attr, item, Position::Prologue)
}

/// Extracts request error data into a variable wrapped in Option type.
///
/// This attribute macro retrieves request error information if an error occurred during handling
/// and makes it available as an Option variable. The extracted value is wrapped
/// in an Option type to safely handle cases where no error occurred.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/try_get_request_error_data")]
/// struct RequestErrorDataOptionTest;
///
/// impl ServerHook for RequestErrorDataOptionTest {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("Request error data: {try_get_request_error_data:?}"))]
///     #[try_get_request_error_data(try_get_request_error_data)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl RequestErrorDataOptionTest {
///     #[try_get_request_error_data(try_get_request_error_data)]
///     async fn try_get_request_error_data_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[try_get_request_error_data(try_get_request_error_data)]
/// async fn standalone_try_get_request_error_data_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a variable name that will contain the request error data.
/// The variable will be available as an `Option<RequestError>` in the function scope.
///
/// # Multi-Parameter Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/try_get_request_error_data")]
/// struct MultiRequestErrorDataOption;
///
/// impl ServerHook for MultiRequestErrorDataOption {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("error1: {error1:?}, error2: {error2:?}"))]
///     #[try_get_request_error_data(error1, error2)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
///
/// The macro accepts multiple variable names separated by commas.
#[proc_macro_attribute]
pub fn try_get_request_error_data(attr: TokenStream, item: TokenStream) -> TokenStream {
    try_get_request_error_data_macro(attr, item, Position::Prologue)
}

/// Extracts request error data into a variable with panic on missing value.
///
/// This attribute macro retrieves request error information if an error occurred during handling
/// and makes it available as a variable. If no error data exists,
/// the function will panic with an error message.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/request_error_data")]
/// struct RequestErrorDataTest;
///
/// impl ServerHook for RequestErrorDataTest {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("Request error data: {request_error_data}"))]
///     #[request_error_data(request_error_data)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl RequestErrorDataTest {
///     #[request_error_data(request_error_data)]
///     async fn request_error_data_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[request_error_data(request_error_data)]
/// async fn standalone_request_error_data_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a variable name that will contain the request error data.
/// The variable will be available as a `RequestError` in the function scope.
///
/// # Multi-Parameter Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/request_error_data")]
/// struct MultiRequestErrorData;
///
/// impl ServerHook for MultiRequestErrorData {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("error1: {error1}, error2: {error2}"))]
///     #[request_error_data(error1, error2)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
///
/// The macro accepts multiple variable names separated by commas.
///
/// # Panics
///
/// This macro will panic if no request error data exists in the request context.
#[proc_macro_attribute]
pub fn request_error_data(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_error_data_macro(attr, item, Position::Prologue)
}

/// Extracts a specific route parameter into a variable wrapped in Option type.
///
/// This attribute macro retrieves a specific route parameter by key and makes it
/// available as an Option variable. Route parameters are extracted from the URL path segments
/// and wrapped in an Option type to safely handle cases where the parameter may not exist.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/try_get_route_param/:test")]
/// struct RouteParam;
///
/// impl ServerHook for RouteParam {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("route param: {request_route_param:?}"))]
///     #[try_get_route_param("test" => request_route_param)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl RouteParam {
///     #[try_get_route_param("test" => request_route_param)]
///     async fn route_param_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[try_get_route_param("test" => request_route_param)]
/// async fn standalone_route_param_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a key-to-variable mapping in the format `"key" => variable_name`.
/// The variable will be available as an `Option<String>` in the function scope.
///
/// # Multi-Parameter Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/multi_param/:id/:name")]
/// struct MultiParam;
///
/// impl ServerHook for MultiParam {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("id: {id:?}, name: {name:?}"))]
///     #[try_get_route_param("id" => id, "name" => name)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
///
/// The macro accepts multiple `"key" => variable_name` pairs separated by commas.
#[proc_macro_attribute]
pub fn try_get_route_param(attr: TokenStream, item: TokenStream) -> TokenStream {
    try_get_route_param_macro(attr, item, Position::Prologue)
}

/// Extracts a specific route parameter into a variable with panic on missing value.
///
/// This attribute macro retrieves a specific route parameter by key and makes it
/// available as a variable. Route parameters are extracted from the URL path segments.
/// If the requested route parameter does not exist, the function will panic with an error message.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/route_param/:test")]
/// struct RouteParam;
///
/// impl ServerHook for RouteParam {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("route param: {request_route_param:?}"))]
///     #[route_param("test" => request_route_param)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl RouteParam {
///     #[route_param("test" => request_route_param)]
///     async fn route_param_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[route_param("test" => request_route_param)]
/// async fn standalone_route_param_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a key-to-variable mapping in the format `"key" => variable_name`.
/// The variable will be available as an `String` in the function scope.
///
///
/// # Multi-Parameter Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/multi_param/:id/:name")]
/// struct MultiParam;
///
/// impl ServerHook for MultiParam {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("id: {id:?}, name: {name:?}"))]
///     #[route_param("id" => id, "name" => name)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
///
/// The macro accepts multiple `"key" => variable_name` pairs separated by commas.
///
/// # Panics
///
/// This macro will panic if the requested route parameter does not exist in the URL path.
#[proc_macro_attribute]
pub fn route_param(attr: TokenStream, item: TokenStream) -> TokenStream {
    route_param_macro(attr, item, Position::Prologue)
}

/// Extracts all route parameters into a collection variable.
///
/// This attribute macro retrieves all available route parameters from the URL path
/// and makes them available as a collection for comprehensive route parameter access.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/route_params/:test")]
/// struct RouteParams;
///
/// impl ServerHook for RouteParams {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("request route params: {request_route_params:?}"))]
///     #[route_params(request_route_params)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl RouteParams {
///     #[route_params(request_route_params)]
///     async fn route_params_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[route_params(request_route_params)]
/// async fn standalone_route_params_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a variable name that will contain all route parameters.
/// The variable will be available as a RouteParams type in the function scope.
///
/// # Multi-Parameter Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/multi_params/:id")]
/// struct MultiParams;
///
/// impl ServerHook for MultiParams {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("params1: {params1:?}, params2: {params2:?}"))]
///     #[route_params(params1, params2)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
///
/// The macro accepts multiple variable names separated by commas.
#[proc_macro_attribute]
pub fn route_params(attr: TokenStream, item: TokenStream) -> TokenStream {
    route_params_macro(attr, item, Position::Prologue)
}

/// Extracts a specific request query parameter into a variable wrapped in Option type.
///
/// This attribute macro retrieves a specific request query parameter by key and makes it
/// available as an Option variable. Query parameters are extracted from the URL request query string
/// and wrapped in an Option type to safely handle cases where the parameter may not exist.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/try_get_request_query")]
/// struct RequestQuery;
///
/// impl ServerHook for RequestQuery {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(
///         try_get_request_query("test" => try_get_request_query),
///         response_body(&format!("request query: {try_get_request_query:?}")),
///         send
///     )]
///     async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl RequestQuery {
///     #[try_get_request_query("test" => try_get_request_query)]
///     async fn request_query_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[try_get_request_query("test" => try_get_request_query)]
/// async fn standalone_request_query_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a key-to-variable mapping in the format `"key" => variable_name`.
/// The variable will be available as an `Option<RequestQuerysValue>` in the function scope.
///
/// Supports multiple parameters: `#[try_get_request_query("k1" => v1, "k2" => v2)]`
#[proc_macro_attribute]
pub fn try_get_request_query(attr: TokenStream, item: TokenStream) -> TokenStream {
    try_get_request_query_macro(attr, item, Position::Prologue)
}

/// Extracts a specific request query parameter into a variable with panic on missing value.
///
/// This attribute macro retrieves a specific request query parameter by key and makes it
/// available as a variable. Query parameters are extracted from the URL request query string.
/// If the requested query parameter does not exist, the function will panic with an error message.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/request_query")]
/// struct RequestQuery;
///
/// impl ServerHook for RequestQuery {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(
///         request_query("test" => request_query),
///         response_body(&format!("request query: {request_query}")),
///         send
///     )]
///     async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl RequestQuery {
///     #[request_query("test" => request_query)]
///     async fn request_query_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[request_query("test" => request_query)]
/// async fn standalone_request_query_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a key-to-variable mapping in the format `"key" => variable_name`.
/// The variable will be available as an `RequestQuerysValue` in the function scope.
///
/// Supports multiple parameters: `#[request_query("k1" => v1, "k2" => v2)]`
///
/// # Panics
///
/// This macro will panic if the requested query parameter does not exist in the URL query string.
#[proc_macro_attribute]
pub fn request_query(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_query_macro(attr, item, Position::Prologue)
}

/// Extracts all request query parameters into a RequestQuerys variable.
///
/// This attribute macro retrieves all available request query parameters from the URL request query string
/// and makes them available as a RequestQuerys for comprehensive request query parameter access.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/request_querys")]
/// struct RequestQuerys;
///
/// impl ServerHook for RequestQuerys {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(
///         request_querys(request_querys),
///         response_body(&format!("request querys: {request_querys:?}")),
///         send
///     )]
///     async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl RequestQuerys {
///     #[request_querys(request_querys)]
///     async fn request_querys_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[request_querys(request_querys)]
/// async fn standalone_request_querys_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a variable name that will contain all request query parameters.
/// The variable will be available as a collection in the function scope.
///
/// Supports multiple parameters: `#[request_querys(querys1, querys2)]`
#[proc_macro_attribute]
pub fn request_querys(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_querys_macro(attr, item, Position::Prologue)
}

/// Extracts a specific HTTP request header into a variable wrapped in Option type.
///
/// This attribute macro retrieves a specific HTTP request header by name and makes it
/// available as an Option variable. Header values are extracted from the request request headers collection
/// and wrapped in an Option type to safely handle cases where the header may not exist.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/try_get_request_header")]
/// struct RequestHeader;
///
/// impl ServerHook for RequestHeader {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(
///         try_get_request_header(HOST => try_get_request_header),
///         response_body(&format!("request header: {try_get_request_header:?}")),
///         send
///     )]
///     async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl RequestHeader {
///     #[try_get_request_header(HOST => try_get_request_header)]
///     async fn request_header_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[try_get_request_header(HOST => try_get_request_header)]
/// async fn standalone_request_header_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a request header name-to-variable mapping in the format `HEADER_NAME => variable_name`
/// or `"Header-Name" => variable_name`. The variable will be available as an `Option<RequestHeadersValueItem>`.
#[proc_macro_attribute]
pub fn try_get_request_header(attr: TokenStream, item: TokenStream) -> TokenStream {
    try_get_request_header_macro(attr, item, Position::Prologue)
}

/// Extracts a specific HTTP request header into a variable with panic on missing value.
///
/// This attribute macro retrieves a specific HTTP request header by name and makes it
/// available as a variable. Header values are extracted from the request request headers collection.
/// If the requested header does not exist, the function will panic with an error message.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/request_header")]
/// struct RequestHeader;
///
/// impl ServerHook for RequestHeader {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(
///         request_header(HOST => request_header),
///         response_body(&format!("request header: {request_header}")),
///         send
///     )]
///     async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl RequestHeader {
///     #[request_header(HOST => request_header)]
///     async fn request_header_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[request_header(HOST => request_header)]
/// async fn standalone_request_header_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a request header name-to-variable mapping in the format `HEADER_NAME => variable_name`
/// or `"Header-Name" => variable_name`. The variable will be available as an `RequestHeadersValueItem`.
///
/// # Panics
///
/// This macro will panic if the requested header does not exist in the HTTP request headers.
#[proc_macro_attribute]
pub fn request_header(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_header_macro(attr, item, Position::Prologue)
}

/// Extracts all HTTP request headers into a collection variable.
///
/// This attribute macro retrieves all available HTTP request headers from the request
/// and makes them available as a collection for comprehensive request header access.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/request_headers")]
/// struct RequestHeaders;
///
/// impl ServerHook for RequestHeaders {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(
///         request_headers(request_headers),
///         response_body(&format!("request headers: {request_headers:?}")),
///         send
///     )]
///     async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl RequestHeaders {
///     #[request_headers(request_headers)]
///     async fn request_headers_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[request_headers(request_headers)]
/// async fn standalone_request_headers_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a variable name that will contain all HTTP request headers.
/// The variable will be available as a RequestHeaders type in the function scope.
#[proc_macro_attribute]
pub fn request_headers(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_headers_macro(attr, item, Position::Prologue)
}

/// Extracts a specific cookie value or all cookies into a variable wrapped in Option type.
///
/// This attribute macro supports two syntaxes:
/// 1. `cookie(key => variable_name)` - Extract a specific cookie value by key, wrapped in Option
/// 2. `cookie(variable_name)` - Extract all cookies as a raw string, wrapped in Option
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/cookie")]
/// struct Cookie;
///
/// impl ServerHook for Cookie {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("Session cookie: {session_cookie1_option:?}, {session_cookie2_option:?}"))]
///     #[try_get_request_cookie("test1" => session_cookie1_option, "test2" => session_cookie2_option)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl Cookie {
///     #[response_body(&format!("Session cookie: {session_cookie1_option:?}, {session_cookie2_option:?}"))]
///     #[try_get_request_cookie("test1" => session_cookie1_option, "test2" => session_cookie2_option)]
///     async fn request_cookie_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[response_body(&format!("Session cookie: {session_cookie1_option:?}, {session_cookie2_option:?}"))]
/// #[try_get_request_cookie("test1" => session_cookie1_option, "test2" => session_cookie2_option)]
/// async fn standalone_request_cookie_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// For specific cookie extraction, the variable will be available as `Option<String>`.
/// For all cookies extraction, the variable will be available as `String`.
#[proc_macro_attribute]
pub fn try_get_request_cookie(attr: TokenStream, item: TokenStream) -> TokenStream {
    try_get_request_cookie_macro(attr, item, Position::Prologue)
}

/// Extracts a specific cookie value or all cookies into a variable with panic on missing value.
///
/// This attribute macro supports two syntaxes:
/// 1. `cookie(key => variable_name)` - Extract a specific cookie value by key, panics if missing
/// 2. `cookie(variable_name)` - Extract all cookies as a raw string, panics if missing
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/cookie")]
/// struct Cookie;
///
/// impl ServerHook for Cookie {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("Session cookie: {session_cookie1}, {session_cookie2}"))]
///     #[request_cookie("test1" => session_cookie1, "test2" => session_cookie2)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl Cookie {
///     #[response_body(&format!("Session cookie: {session_cookie1}, {session_cookie2}"))]
///     #[request_cookie("test1" => session_cookie1, "test2" => session_cookie2)]
///     async fn request_cookie_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[response_body(&format!("Session cookie: {session_cookie1}, {session_cookie2}"))]
/// #[request_cookie("test1" => session_cookie1, "test2" => session_cookie2)]
/// async fn standalone_request_cookie_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// For specific cookie extraction, the variable will be available as `String`.
/// For all cookies extraction, the variable will be available as `String`.
///
/// # Panics
///
/// This macro will panic if the requested cookie does not exist in the HTTP request headers.
#[proc_macro_attribute]
pub fn request_cookie(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_cookie_macro(attr, item, Position::Prologue)
}

/// Extracts all cookies as a raw string into a variable.
///
/// This attribute macro retrieves the entire Cookie header from the request and makes it
/// available as a String variable. If no Cookie header is present, an empty string is used.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/cookies")]
/// struct Cookies;
///
/// impl ServerHook for Cookies {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("All cookies: {cookie_value:?}"))]
///     #[request_cookies(cookie_value)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl Cookies {
///     #[request_cookies(cookie_value)]
///     async fn request_cookies_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[request_cookies(cookie_value)]
/// async fn standalone_request_cookies_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a variable name that will contain all cookies.
/// The variable will be available as a Cookies type in the function scope.
///
/// # Multi-Parameter Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/multi_cookies")]
/// struct MultiCookies;
///
/// impl ServerHook for MultiCookies {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("cookies1: {cookies1:?}, cookies2: {cookies2:?}"))]
///     #[request_cookies(cookies1, cookies2)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
#[proc_macro_attribute]
pub fn request_cookies(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_cookies_macro(attr, item, Position::Prologue)
}

/// Extracts the HTTP request version into a variable.
///
/// This attribute macro retrieves the HTTP version from the request and makes it
/// available as a variable. The version represents the HTTP protocol version used.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/request_version")]
/// struct RequestVersionTest;
///
/// impl ServerHook for RequestVersionTest {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("HTTP Version: {is_http_version}"))]
///     #[request_version(is_http_version)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl RequestVersionTest {
///     #[request_version(is_http_version)]
///     async fn request_version_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[request_version(is_http_version)]
/// async fn standalone_request_version_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a variable name that will contain the HTTP request version.
/// The variable will be available as a RequestVersion type in the function scope.
#[proc_macro_attribute]
pub fn request_version(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_version_macro(attr, item, Position::Prologue)
}

/// Extracts the HTTP request path into a variable.
///
/// This attribute macro retrieves the request path from the HTTP request and makes it
/// available as a variable. The path represents the URL path portion of the request.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/request_path")]
/// struct RequestPathTest;
///
/// impl ServerHook for RequestPathTest {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body(&format!("Request Path: {request_path}"))]
///     #[request_path(request_path)]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl RequestPathTest {
///     #[request_path(request_path)]
///     async fn request_path_with_ref_self(&self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[request_path(request_path)]
/// async fn standalone_request_path_handler(_: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro accepts a variable name that will contain the HTTP request path.
/// The variable will be available as a RequestPath type in the function scope.
#[proc_macro_attribute]
pub fn request_path(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_path_macro(attr, item, Position::Prologue)
}

/// Creates a new instance of a specified type with a given variable name.
///
/// This attribute macro generates an instance initialization at the beginning of the function.
///
/// # Usage
///
/// ```rust,no_run
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[hyperlane(server: Server)]
/// #[hyperlane(server_config: ServerConfig)]
/// #[tokio::main]
/// async fn main() {
///     server_config.set_nodelay(Some(false));
///     server.server_config(server_config);
///     let server_hook: ServerControlHook = server.run().await.unwrap_or_default();
///     server_hook.wait().await;
/// }
/// ```
///
/// Using in impl block method:
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// struct ServerInitializer;
///
/// impl ServerInitializer {
///     #[hyperlane(server: Server)]
///     #[hyperlane(server_config: ServerConfig)]
///     async fn initialize_server_1() -> Server {
///         server
///     }
///
///     #[hyperlane(server: Server)]
///     #[hyperlane(server_config: ServerConfig)]
///     async fn initialize_server_2(self) -> Server {
///         server
///     }
///
///     #[hyperlane(server: Server)]
///     #[hyperlane(server_config: ServerConfig)]
///     async fn initialize_server_3(&self) -> Server {
///         server
///     }
/// }
/// ```
///
/// The macro accepts a `variable_name: Type` pair.
/// The variable will be available as an instance of the specified type in the function scope.
#[proc_macro_attribute]
pub fn hyperlane(attr: TokenStream, item: TokenStream) -> TokenStream {
    hyperlane_macro(attr, item)
}

/// Registers a function as a route handler.
///
/// This attribute macro registers the decorated function as a route handler for a given path.
/// This macro requires the `#[hyperlane(server: Server)]` macro to be used to define the server instance.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/response")]
/// struct Response;
///
/// impl ServerHook for Response {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[response_body("response")]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
///
/// # Parameters
///
/// - `path`: String literal defining the route path
///
/// # Dependencies
///
/// This macro depends on the `#[hyperlane(server: Server)]` macro to define the server instance.
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    route_macro(attr, item)
}

/// Registers a function as a request middleware.
///
/// This attribute macro registers the decorated function to be executed as a middleware
/// for incoming requests. This macro requires the `#[hyperlane(server: Server)]` macro to be used to define the server instance.
///
/// # Note
///
/// If an order parameter is not specified, the hook will have a higher priority than hooks with a specified order.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[request_middleware]
/// struct RequestMiddleware;
///
/// impl ServerHook for RequestMiddleware {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[epilogue_macros(
///         response_status_code(200),
///         response_version(HttpVersion::Http1_1),
///         response_header(SERVER => HYPERLANE)
///     )]
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
///
/// # Dependencies
///
/// This macro depends on the `#[hyperlane(server: Server)]` macro to define the server instance.
#[proc_macro_attribute]
pub fn request_middleware(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_middleware_macro(attr, item)
}

/// Registers a function as a response middleware.
///
/// This attribute macro registers the decorated function to be executed as a middleware
/// for outgoing responses. This macro requires the `#[hyperlane(server: Server)]` macro to be used to define the server instance.
///
/// # Note
///
/// If an order parameter is not specified, the hook will have a higher priority than hooks with a specified order.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[response_middleware]
/// struct ResponseMiddleware1;
///
/// impl ServerHook for ResponseMiddleware1 {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
///
/// # Dependencies
///
/// This macro depends on the `#[hyperlane(server: Server)]` macro to define the server instance.
#[proc_macro_attribute]
pub fn response_middleware(attr: TokenStream, item: TokenStream) -> TokenStream {
    response_middleware_macro(attr, item)
}

/// Registers a function as a panic hook.
///
/// This attribute macro registers the decorated function to handle panics that occur
/// during request processing. This macro requires the `#[hyperlane(server: Server)]` macro to be used to define the server instance.
///
/// # Note
///
/// If an order parameter is not specified, the hook will have a higher priority than hooks with a specified order.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[task_panic]
/// #[task_panic(1)]
/// #[task_panic("2")]
/// struct PanicHook;
///
/// impl ServerHook for PanicHook {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[epilogue_macros(response_body("task_panic"), send)]
///     async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
///
/// # Dependencies
///
/// This macro depends on the `#[hyperlane(server: Server)]` macro to define the server instance.
#[proc_macro_attribute]
pub fn task_panic(attr: TokenStream, item: TokenStream) -> TokenStream {
    task_panic_macro(attr, item)
}

/// Registers a function as a request error hook.
///
/// This attribute macro registers the decorated function to handle request errors that occur
/// during request processing. This macro requires the `#[hyperlane(server: Server)]` macro to be used to define the server instance.
///
/// # Note
///
/// If an order parameter is not specified, the hook will have a higher priority than hooks with a specified order.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[request_error]
/// #[request_error(1)]
/// #[request_error("2")]
/// struct RequestErrorHook;
///
/// impl ServerHook for RequestErrorHook {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[epilogue_macros(response_body("request_error"), send)]
///     async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
///
/// # Dependencies
///
/// This macro depends on the `#[hyperlane(server: Server)]` macro to define the server instance.
#[proc_macro_attribute]
pub fn request_error(attr: TokenStream, item: TokenStream) -> TokenStream {
    request_error_macro(attr, item)
}

/// Injects a list of macros before the decorated function.
///
/// The macros are applied in head-insertion order, meaning the first macro in the list
/// is the outermost macro.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/prologue_macros")]
/// struct PrologueMacros;
///
/// impl ServerHook for PrologueMacros {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[prologue_macros(is_post_method, response_body("prologue_macros"), send)]
///     async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
#[proc_macro_attribute]
pub fn prologue_macros(attr: TokenStream, item: TokenStream) -> TokenStream {
    prologue_macros_macro(attr, item)
}

/// Injects a list of macros after the decorated function.
///
/// The macros are applied in tail-insertion order, meaning the last macro in the list
/// is the outermost macro.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[response_middleware(2)]
/// struct ResponseMiddleware2;
///
/// impl ServerHook for ResponseMiddleware2 {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[epilogue_macros(try_send, flush)]
///     async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
#[proc_macro_attribute]
pub fn epilogue_macros(attr: TokenStream, item: TokenStream) -> TokenStream {
    epilogue_macros_macro(attr, item)
}

/// Automatically tries to send data via stream after function execution.
///
/// This attribute macro tries to send data to the client after the function completes execution.
/// If no argument is provided, the response is built from the context automatically.
/// If an argument is provided, it is used as the data expression to send.
///
/// # Usage
///
/// Using without arguments (default from context):
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/try_send")]
/// struct TrySendTest;
///
/// impl ServerHook for TrySendTest {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[epilogue_macros(try_send)]
///     async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl TrySendTest {
///     #[try_send]
///     async fn try_send_with_ref_self(&self, stream: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[try_send]
/// async fn standalone_try_send_handler(stream: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// Using with a data expression:
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/try_send_with_data")]
/// struct TrySendWithDataTest;
///
/// impl ServerHook for TrySendWithDataTest {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[epilogue_macros(try_send(ctx.get_mut_response().build()))]
///     async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
///
/// The macro accepts an optional data expression. If omitted, it defaults to sending
/// the response built from the context.
#[proc_macro_attribute]
pub fn try_send(attr: TokenStream, item: TokenStream) -> TokenStream {
    try_send_macro(attr, item, Position::Epilogue)
}

/// Automatically sends data via stream after function execution.
///
/// This attribute macro sends data to the client after the function completes execution.
/// If no argument is provided, the response is built from the context automatically.
/// If an argument is provided, it is used as the data expression to send.
///
/// # Usage
///
/// Using without arguments (default from context):
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/send")]
/// struct SendTest;
///
/// impl ServerHook for SendTest {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[epilogue_macros(send)]
///     async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl SendTest {
///     #[send]
///     async fn send_with_ref_self(&self, stream: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[send]
/// async fn standalone_send_handler(stream: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// ```
///
/// Using with a data expression:
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/send_with_data")]
/// struct SendWithDataTest;
///
/// impl ServerHook for SendWithDataTest {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[epilogue_macros(send(ctx.get_mut_response().build()))]
///     async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status { Status::Continue }
/// }
/// ```
///
/// The macro accepts an optional data expression. If omitted, it defaults to sending
/// the response built from the context.
///
/// # Panics
///
/// This macro will panic if the send operation fails.
#[proc_macro_attribute]
pub fn send(attr: TokenStream, item: TokenStream) -> TokenStream {
    send_macro(attr, item, Position::Epilogue)
}

/// Tries to flush the response stream after function execution.
///
/// This attribute macro ensures that the response stream is tried to be flushed to guarantee immediate
/// data transmission, forcing any buffered response data to be sent to the client. This will not panic on failure.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/try_flush")]
/// struct TryFlushTest;
///
/// impl ServerHook for TryFlushTest {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[epilogue_macros(try_flush)]
///     async fn handle(self, stream: &mut Stream, _: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl TryFlushTest {
///     #[try_flush]
///     async fn try_flush_with_ref_self(&self, stream: &mut Stream, _: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[try_flush]
/// async fn standalone_try_flush_handler(stream: &mut Stream, _: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
#[proc_macro_attribute]
pub fn try_flush(_attr: TokenStream, item: TokenStream) -> TokenStream {
    try_flush_macro(item, Position::Prologue)
}

/// Flushes the response stream after function execution.
///
/// This attribute macro ensures that the response stream is flushed to guarantee immediate
/// data transmission, forcing any buffered response data to be sent to the client.
///
/// # Usage
///
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/flush")]
/// struct FlushTest;
///
/// impl ServerHook for FlushTest {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     #[epilogue_macros(flush)]
///     async fn handle(self, stream: &mut Stream, _: &mut Context) -> Status { Status::Continue }
/// }
///
/// impl FlushTest {
///     #[flush]
///     async fn flush_with_ref_self(&self, stream: &mut Stream, _: &mut Context) -> Status { Status::Continue }
/// }
///
/// #[flush]
/// async fn standalone_flush_handler(stream: &mut Stream, _: &mut Context) -> Status { Status::Continue }
/// ```
///
/// The macro takes no parameters and should be applied directly to async functions
/// that accept a `&mut Context` parameter.
///
/// # Panics
///
/// This macro will panic if the flush operation fails.
#[proc_macro_attribute]
pub fn flush(_attr: TokenStream, item: TokenStream) -> TokenStream {
    flush_macro(item, Position::Prologue)
}

/// Generates a context reference binding statement.
///
/// This function-like procedural macro generates a let statement that converts
/// a context pointer into a reference to `::hyperlane_core::Context`.
/// The conversion is performed through the `Into` trait with an intermediate
/// conversion to usize.
///
/// The mutability of the generated reference is determined by the optional type annotation:
/// - If the type annotation contains `&mut`, generates `leak_mut()` call
/// - Otherwise, generates `leak()` call (default behavior)
///
/// # Arguments
///
/// - `TokenStream` - The input token stream containing the variable name identifier
///   and an optional type annotation (e.g., `ctx` or `ctx: &mut Context`).
///
/// # Returns
///
/// - `TokenStream` - A let statement binding the specified variable name
///   to a `&mut ::hyperlane_core::Context` or `&::hyperlane_core::Context` obtained through pointer conversion.
///
/// # Examples
///
/// With explicit type annotation for mutable reference:
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/context_mut")]
/// struct ContextMut;
///
/// impl ServerHook for ContextMut {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
///         let new_ctx: &mut Context = unsafe { context!(ctx: &mut Context) };
///         let _ = new_ctx.get_mut_response();
///         Status::Continue
///     }
/// }
///
/// async fn example(_: &mut Stream, ctx: &mut Context) {
///     let new_ctx: &mut Context = unsafe { context!(ctx: &mut Context) };
///     let _ = new_ctx.get_mut_response();
/// }
/// ```
///
/// With explicit type annotation for immutable reference:
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/context_ref")]
/// struct ContextRef;
///
/// impl ServerHook for ContextRef {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
///         let new_ctx: &::hyperlane_core::Context = unsafe { context!(ctx: &::hyperlane_core::Context) };
///         let _ = new_ctx.get_request();
///         Status::Continue
///     }
/// }
///
/// async fn example(_: &mut Stream, ctx: &mut Context) {
///     let new_ctx: &::hyperlane_core::Context = unsafe { context!(ctx: &::hyperlane_core::Context) };
///     let _ = new_ctx.get_request();
/// }
/// ```
///
/// Without type annotation (defaults to immutable):
/// ```rust
/// use hyperlane_core::*;
/// use hyperlane_macros::*;
///
/// #[route("/context_default")]
/// struct ContextDefault;
///
/// impl ServerHook for ContextDefault {
///     async fn new(_: &mut Stream, _: &mut Context) -> Self {
///         Self
///     }
///
///     async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
///         let new_ctx = unsafe { context!(ctx) };
///         let _ = new_ctx.get_request();
///         Status::Continue
///     }
/// }
///
/// async fn example(_: &mut Stream, ctx: &mut Context) {
///     let new_ctx = unsafe { context!(ctx) };
///     let _ = new_ctx.get_request();
/// }
/// ```
///
/// # Safety
///
/// - The address is guaranteed to be a valid `Self` instance
///   that was previously converted from a reference and is managed by the runtime.
#[proc_macro]
pub fn context(input: TokenStream) -> TokenStream {
    context_macro(input)
}
