use super::*;

/// JSON-RPC 2.0 request/response correlation identifier.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum RpcId {
    /// Integer identifier.
    Number(i64),
    /// String identifier.
    Text(String),
    /// Explicit null identifier.
    #[default]
    Null,
}
