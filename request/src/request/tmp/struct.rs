use super::*;

/// Per-request scratch space.
///
/// Holds redirect-loop detection (`visit_url`) and the TLS root cert store.
/// Lives inside [`HttpRequest`] but is **not** serialized — purely an
/// implementation detail of the client state machine.
///
/// Fields are private; access through the methods on [`HttpRequest`].
#[derive(Clone, Debug)]
pub struct Tmp {
    pub(crate) visit_url: HashSet<String>,
    pub(crate) root_cert: RootCertStore,
}

impl Default for Tmp {
    #[inline(always)]
    fn default() -> Self {
        Self {
            visit_url: HashSet::new(),
            root_cert: RootCertStore {
                roots: TLS_SERVER_ROOTS.to_vec(),
            },
        }
    }
}
