use crate::*;

/// Represents the different types of segments that can make up a route path.
///
/// A route path is parsed into a sequence of these segments. For example, the path
/// `/users/:id/posts` would be broken down into `Static("users")`, `Dynamic("id")`,
/// and `Static("posts")`.
#[derive(Clone, CustomDebug, DisplayDebug)]
pub enum RouteSegment {
    /// A static, literal segment of a path.
    /// This must be an exact match. For example, in `/users/active`, "users" and "active"
    /// are both static segments.
    Static(String),
    /// A dynamic segment that captures a value from the path.
    /// It is denoted by a colon prefix. The captured value
    /// is stored as a parameter in the request context.
    Dynamic(String),
    /// A segment that is matched against a regular expression.
    /// This allows for more complex and flexible routing logic. The first element is the parameter
    /// name, and the second is the compiled `Regex` object.
    Regex(String, Regex),
}
