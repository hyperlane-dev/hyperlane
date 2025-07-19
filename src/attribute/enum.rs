use crate::*;

#[derive(CustomDebug, Clone, PartialEq, Eq, Hash, DisplayDebug)]
pub(crate) enum AttributeKey {
    External(String),
    Internal(InternalAttributeKey),
}

#[derive(CustomDebug, Clone, PartialEq, Eq, Hash, DisplayDebug)]
pub(crate) enum InternalAttributeKey {
    Panic,
}
