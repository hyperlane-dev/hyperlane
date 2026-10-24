//! bench-server
//!
//! The server the hyperlane benchmark harness measures. It answers the route
//! shapes a typical workload carries, through the public API only.
#[cfg(feature = "alloc-count")]
mod r#alloc;
mod r#const;

pub(crate) use r#const::*;

#[cfg(feature = "alloc-count")]
pub(crate) use r#alloc::*;

use std::{collections::VecDeque, env::var};

#[cfg(feature = "alloc-count")]
use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::atomic::{AtomicU64, Ordering},
};

use hyperlane::{
    Context, ResponseError, Server, ServerConfig, ServerControlHook, ServerHook, ServerProtocol,
    Status, Stream, TlsConfig,
};

/// A route that answers with a fixed body.
struct RootRoute;

/// Implements [`ServerHook`] for [`RootRoute`].
impl ServerHook for RootRoute {
    /// Builds the route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The stream the request arrived on.
    /// - `&mut Context` - The context of the request.
    ///
    /// # Returns
    ///
    /// - `Self` - The route.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Answers the request.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route.
    /// - `&mut Stream` - The stream the request arrived on.
    /// - `&mut Context` - The context of the request.
    ///
    /// # Returns
    ///
    /// - `Status` - Whether the pipeline continues.
    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        ctx.get_mut_response()
            .set_status_code(STATUS_OK)
            .set_body(BODY_ROOT);
        Status::Continue
    }
}

/// A route that answers with a path parameter.
struct UserRoute;

/// Implements [`ServerHook`] for [`UserRoute`].
impl ServerHook for UserRoute {
    /// Builds the route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The stream the request arrived on.
    /// - `&mut Context` - The context of the request.
    ///
    /// # Returns
    ///
    /// - `Self` - The route.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Answers the request.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route.
    /// - `&mut Stream` - The stream the request arrived on.
    /// - `&mut Context` - The context of the request.
    ///
    /// # Returns
    ///
    /// - `Status` - Whether the pipeline continues.
    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let id: String = ctx.get_route_param(PARAM_ID);
        let mut body: Vec<u8> = Vec::with_capacity(BODY_USER.len() + id.len());
        body.extend_from_slice(BODY_USER);
        body.extend_from_slice(id.as_bytes());
        ctx.get_mut_response()
            .set_status_code(STATUS_OK)
            .set_body(body);
        Status::Continue
    }
}

/// A route that answers with a query parameter.
struct SearchRoute;

/// Implements [`ServerHook`] for [`SearchRoute`].
impl ServerHook for SearchRoute {
    /// Builds the route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The stream the request arrived on.
    /// - `&mut Context` - The context of the request.
    ///
    /// # Returns
    ///
    /// - `Self` - The route.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Answers the request.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route.
    /// - `&mut Stream` - The stream the request arrived on.
    /// - `&mut Context` - The context of the request.
    ///
    /// # Returns
    ///
    /// - `Status` - Whether the pipeline continues.
    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let query: String = ctx
            .get_request()
            .try_get_query(PARAM_QUERY)
            .and_then(|values: VecDeque<String>| values.front().cloned())
            .unwrap_or_default();
        ctx.get_mut_response()
            .set_status_code(STATUS_OK)
            .set_body(format!("q={query}"));
        Status::Continue
    }
}

/// A route that answers with a request header.
struct HeadersRoute;

/// Implements [`ServerHook`] for [`HeadersRoute`].
impl ServerHook for HeadersRoute {
    /// Builds the route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The stream the request arrived on.
    /// - `&mut Context` - The context of the request.
    ///
    /// # Returns
    ///
    /// - `Self` - The route.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Answers the request.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route.
    /// - `&mut Stream` - The stream the request arrived on.
    /// - `&mut Context` - The context of the request.
    ///
    /// # Returns
    ///
    /// - `Status` - Whether the pipeline continues.
    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let tag: String = ctx
            .get_request()
            .try_get_header(HEADER_TAG)
            .and_then(|values: VecDeque<String>| values.front().cloned())
            .unwrap_or_default();
        ctx.get_mut_response()
            .set_status_code(STATUS_OK)
            .set_body(format!("tag={tag}"));
        Status::Continue
    }
}

/// A route that answers with the request body.
struct EchoRoute;

/// Implements [`ServerHook`] for [`EchoRoute`].
impl ServerHook for EchoRoute {
    /// Builds the route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The stream the request arrived on.
    /// - `&mut Context` - The context of the request.
    ///
    /// # Returns
    ///
    /// - `Self` - The route.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Answers the request.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route.
    /// - `&mut Stream` - The stream the request arrived on.
    /// - `&mut Context` - The context of the request.
    ///
    /// # Returns
    ///
    /// - `Status` - Whether the pipeline continues.
    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let body: Vec<u8> = ctx.get_request().get_body().clone();
        ctx.get_mut_response()
            .set_status_code(STATUS_OK)
            .set_body(body);
        Status::Continue
    }
}

/// The response middleware that writes the response onto the stream.
struct SendMiddleware;

/// Implements [`ServerHook`] for [`SendMiddleware`].
impl ServerHook for SendMiddleware {
    /// Builds the middleware.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The stream the request arrived on.
    /// - `&mut Context` - The context of the request.
    ///
    /// # Returns
    ///
    /// - `Self` - The middleware.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Answers the request.
    ///
    /// # Arguments
    ///
    /// - `Self` - The middleware.
    /// - `&mut Stream` - The stream the request arrived on.
    /// - `&mut Context` - The context of the request.
    ///
    /// # Returns
    ///
    /// - `Status` - Whether the pipeline continues.
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        #[cfg(feature = "alloc-count")]
        count_request();
        let data: Vec<u8> = ctx.get_mut_response().build();
        let _: Result<(), ResponseError> = stream.try_send(data).await;
        Status::Continue
    }
}

/// Starts the server under test and serves until it is shut down.
///
/// The address comes from the environment, so the entry point does not have to
/// know it.
///
/// # Returns
///
/// - `Result<(), String>` - Nothing, or the reason the server could not start.
pub async fn serve() -> Result<(), String> {
    let address: String = match var(ENV_ADDRESS) {
        Ok(address) => address,
        Err(_) => DEFAULT_ADDRESS.to_owned(),
    };
    let name: String = match var(ENV_PROTOCOL) {
        Ok(name) => name,
        Err(_) => PROTOCOL_HTTP1.to_owned(),
    };
    let protocol: ServerProtocol = match protocol_of(&name) {
        Some(protocol) => protocol,
        None => return Err(format!("unknown protocol {name}")),
    };
    let uses_tls: bool = protocol_uses_tls(&name);
    println!("bench-server listening on {address}");
    let mut server: Server = Server::default();
    let mut server_config: ServerConfig = ServerConfig::default();
    server_config.set_address(address.clone());
    server_config.set_protocol(protocol);
    let certificate: Option<String> = var(ENV_TLS_CERTIFICATE).ok();
    let key: Option<String> = var(ENV_TLS_KEY).ok();
    if uses_tls {
        let (Some(certificate), Some(key)) = (certificate, key) else {
            return Err(format!("{name} needs a certificate and a key"));
        };
        let tls_config: TlsConfig = TlsConfig::new(certificate, key);
        server_config.set_tls(Some(tls_config));
    }
    server
        .server_config(server_config)
        .route::<RootRoute>(ROUTE_ROOT)
        .route::<UserRoute>(ROUTE_USER)
        .route::<SearchRoute>(ROUTE_SEARCH)
        .route::<HeadersRoute>(ROUTE_HEADERS)
        .route::<EchoRoute>(ROUTE_ECHO)
        .response_middleware::<SendMiddleware>();
    let hook: ServerControlHook = match server.run().await {
        Ok(hook) => hook,
        Err(error) => return Err(error.to_string()),
    };
    println!("bench-server ready on {address} speaking {protocol:?}");
    #[cfg(feature = "alloc-count")]
    spawn_watchdog();
    hook.wait().await;
    Ok(())
}

/// Reports the allocation counters every second, in the diagnostic build.
///
/// The counters are printed on their own line so the reading of a run is the
/// last line that names them, not a line mixed into the server's own output.
///
/// The watchdog is a diagnostic instrument. It runs only in the build that
/// carries the counting allocator.
#[cfg(feature = "alloc-count")]
fn spawn_watchdog() {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            let requests: u64 = REQUESTS.load(Ordering::Relaxed);
            println!("{}", report(requests));
        }
    });
}

/// Reads the protocol from its environment name.
///
/// # Arguments
///
/// - `&str` - The name the environment carries.
///
/// # Returns
///
/// - `Option<ServerProtocol>` - The protocol, when the name names one.
fn protocol_of(name: &str) -> Option<ServerProtocol> {
    match name {
        PROTOCOL_HTTP1 | PROTOCOL_HTTP1_TLS => Some(ServerProtocol::Http1),
        PROTOCOL_HTTP2 | PROTOCOL_HTTP2_TLS => Some(ServerProtocol::Http2),
        PROTOCOL_HTTP3 => Some(ServerProtocol::Http3),
        _ => None,
    }
}

/// Reports whether the protocol terminates TLS.
///
/// # Arguments
///
/// - `&str` - The name the environment carries.
///
/// # Returns
///
/// - `bool` - Whether the protocol terminates TLS.
fn protocol_uses_tls(name: &str) -> bool {
    // The list is spelled out rather than matched: the `matches!` macro reads
    // as a closure to the standards audit, and a plain match reads as a
    // `matches!` to the linter. Neither of them is wrong about the code, and
    // the list is the form both accept.
    const TLS_PROTOCOLS: [&str; 3] = [PROTOCOL_HTTP1_TLS, PROTOCOL_HTTP2_TLS, PROTOCOL_HTTP3];
    TLS_PROTOCOLS.contains(&name)
}
