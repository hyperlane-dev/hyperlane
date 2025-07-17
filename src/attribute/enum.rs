#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum AttributeKey {
    External(String),
    Internal(InternalAttributeKey),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum InternalAttributeKey {
    PanicInfo,
}
