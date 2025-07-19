use crate::*;

impl From<&str> for AttributeKey {
    fn from(key: &str) -> Self {
        AttributeKey::External(key.to_string())
    }
}

impl From<String> for AttributeKey {
    fn from(key: String) -> Self {
        AttributeKey::External(key)
    }
}

impl From<InternalAttributeKey> for AttributeKey {
    fn from(key: InternalAttributeKey) -> Self {
        AttributeKey::Internal(key)
    }
}
