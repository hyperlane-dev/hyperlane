/// Reserved lifecycle metadata for the original wire request target.
pub(crate) const REQUEST_TARGET_ATTRIBUTE: &str = "hyperlane.request.target";

/// Static map key matching `Attribute::Internal(InternalAttribute::TaskPanicData).to_string()`.
pub(crate) const INTERNAL_TASK_PANIC_KEY: &str = "Internal(TaskPanicData)";

/// Static map key matching `Attribute::Internal(InternalAttribute::RequestErrorData).to_string()`.
pub(crate) const INTERNAL_REQUEST_ERROR_KEY: &str = "Internal(RequestErrorData)";

/// Prefix emitted by `Attribute::External` debug formatting.
pub(crate) const EXTERNAL_ATTR_PREFIX: &[u8] = b"External(\"";

/// Suffix emitted by `Attribute::External` debug formatting.
pub(crate) const EXTERNAL_ATTR_SUFFIX: &[u8] = b"\")";

/// Maximum unescaped external attribute key length formatted on the stack.
pub(crate) const EXTERNAL_ATTR_STACK_LIMIT: usize = 110;
