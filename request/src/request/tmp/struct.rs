use super::*;

/// Per-request scratch space.
///
/// Holds redirect-loop detection (`visit_url`) and the TLS root cert store.
/// Lives inside [`HttpRequest`] but is **not** serialized — purely an
/// implementation detail of the client state machine.
///
/// Fields are private; access through the methods on [`HttpRequest`].
#[derive(Clone, Data, Debug)]
pub struct Tmp {
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) visit_url: HashSet<String>,
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
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
