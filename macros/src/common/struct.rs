use super::*;

/// Represents a parsed macro attribute containing an optional order.
///
/// This struct is used during macro parsing to hold the extracted order expression.
/// Hooks or attributes that do not specify an order will have `None`.
#[derive(Clone)]
pub(crate) struct OrderAttr {
    /// The optional order expression provided in the macro attribute.
    pub(crate) order: Option<Expr>,
}

/// Represents a macro that can be injected.
///
/// This struct is used to define a macro that can be collected and used by the `inventory` crate.
pub(crate) struct InjectableMacro {
    /// The name of the macro.
    pub(crate) name: &'static str,
    /// The handler for the macro.
    pub(crate) handler: Handler,
}
