//! bench-server
//!
//! The server the hyperlane benchmark harness measures. It answers the route
//! shapes a typical workload carries, through the public API only.
mod r#const;

pub use r#const::*;

use std::{collections::VecDeque, env::var};

use hyperlane::{
    Context, ResponseError, Server, ServerConfig, ServerControlHook, ServerHook, Status, Stream,
};

/// A route that answers with a fixed body.
struct RootRoute;

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
    println!("bench-server listening on {address}");
    let mut server: Server = Server::default();
    let mut server_config: ServerConfig = ServerConfig::default();
    server_config.set_address(address);
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
    hook.wait().await;
    Ok(())
}
